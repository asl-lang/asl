use asl_core_traits::{CapabilityContext, EnginePort};
use asl_spec::SkillDocument;
use serde::{Deserialize, Serialize};
use serde_json::Value;
use std::io::{BufRead, Write};

#[derive(Serialize, Deserialize, Debug)]
pub struct JsonRpcRequest {
    pub jsonrpc: String,
    pub id: Option<Value>,
    pub method: String,
    #[serde(default)]
    pub params: Option<Value>,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct JsonRpcResponse {
    pub jsonrpc: String,
    pub id: Option<Value>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub result: Option<Value>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub error: Option<Value>,
}

pub struct McpServer<'a> {
    skills: Vec<SkillDocument>,
    engine: &'a dyn EnginePort,
    context: &'a dyn CapabilityContext,
}

impl<'a> McpServer<'a> {
    pub fn new(
        skills: Vec<SkillDocument>,
        engine: &'a dyn EnginePort,
        context: &'a dyn CapabilityContext,
    ) -> Self {
        Self {
            skills,
            engine,
            context,
        }
    }

    pub fn handle_message(&self, msg_str: &str) -> Option<JsonRpcResponse> {
        let req: JsonRpcRequest = match serde_json::from_str(msg_str) {
            Ok(r) => r,
            Err(e) => {
                eprintln!("[ASL MCP] JSON-RPC parsing error: {}", e);
                return Some(JsonRpcResponse {
                    jsonrpc: "2.0".to_string(),
                    id: None,
                    result: None,
                    error: Some(serde_json::json!({
                        "code": -32700,
                        "message": "Parse error"
                    })),
                });
            }
        };

        match req.method.as_str() {
            "initialize" => Some(JsonRpcResponse {
                jsonrpc: "2.0".to_string(),
                id: req.id,
                result: Some(serde_json::json!({
                    "protocolVersion": "2024-11-05",
                    "capabilities": {
                        "tools": {}
                    },
                    "serverInfo": {
                        "name": "asl-mcp-server",
                        "version": "3.0.0"
                    }
                })),
                error: None,
            }),
            "ping" => Some(JsonRpcResponse {
                jsonrpc: "2.0".to_string(),
                id: req.id,
                result: Some(serde_json::json!({})),
                error: None,
            }),
            "notifications/initialized" => None,
            "tools/list" => {
                let tools: Vec<Value> = self
                    .skills
                    .iter()
                    .map(|s| {
                        serde_json::json!({
                            "name": s.manifest.name,
                            "description": s.manifest.description,
                            "inputSchema": s.manifest.interface.input_schema
                        })
                    })
                    .collect();

                Some(JsonRpcResponse {
                    jsonrpc: "2.0".to_string(),
                    id: req.id,
                    result: Some(serde_json::json!({ "tools": tools })),
                    error: None,
                })
            }
            "tools/call" => {
                let params = req.params.unwrap_or(Value::Null);
                let tool_name = params.get("name").and_then(|v| v.as_str()).unwrap_or("");
                let arguments = match params.get("arguments") {
                    Some(v) if !v.is_null() => v.clone(),
                    _ => serde_json::json!({}),
                };

                let matched_skill = self.skills.iter().find(|s| s.manifest.name == tool_name);

                match matched_skill {
                    Some(skill) => {
                        let exec_res = self.engine.execute(
                            &skill.deterministic_code,
                            &skill.manifest.interface.entrypoint,
                            &arguments,
                            self.context,
                            &skill.manifest.limits,
                        );

                        match exec_res {
                            Ok(res) => Some(JsonRpcResponse {
                                jsonrpc: "2.0".to_string(),
                                id: req.id,
                                result: Some(serde_json::json!({
                                    "content": [{
                                        "type": "text",
                                        "text": serde_json::to_string_pretty(&res.output).unwrap_or_default()
                                    }],
                                    "isError": !res.success
                                })),
                                error: None,
                            }),
                            Err(e) => Some(JsonRpcResponse {
                                jsonrpc: "2.0".to_string(),
                                id: req.id,
                                result: None,
                                error: Some(serde_json::json!({
                                    "code": -32000,
                                    "message": format!("ASL execution error: {}", e)
                                })),
                            }),
                        }
                    }
                    None => Some(JsonRpcResponse {
                        jsonrpc: "2.0".to_string(),
                        id: req.id,
                        result: None,
                        error: Some(serde_json::json!({
                            "code": -32601,
                            "message": format!("Tool '{}' not found", tool_name)
                        })),
                    }),
                }
            }
            _ => {
                // JSON-RPC 2.0 notifications (without id) never receive an error response
                if req.id.is_none() {
                    None
                } else {
                    Some(JsonRpcResponse {
                        jsonrpc: "2.0".to_string(),
                        id: req.id,
                        result: None,
                        error: Some(serde_json::json!({
                            "code": -32601,
                            "message": "Method not found"
                        })),
                    })
                }
            }
        }
    }

    pub fn run_stdio_loop<R: BufRead, W: Write>(
        &self,
        mut reader: R,
        mut writer: W,
    ) -> std::io::Result<()> {
        let mut line = String::new();
        while reader.read_line(&mut line)? > 0 {
            let trimmed = line.trim();
            if !trimmed.is_empty() {
                if let Some(resp) = self.handle_message(trimmed) {
                    let out_json = serde_json::to_string(&resp)?;
                    writeln!(writer, "{}", out_json)?;
                    writer.flush()?;
                }
            }
            line.clear();
        }
        Ok(())
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use asl_core_traits::CapabilityContext;
    use asl_spec::{ExecutionResult, Limits, Result, SkillInterface, SkillManifest};
    use std::io::Cursor;

    struct DummyEngine;
    impl EnginePort for DummyEngine {
        fn name(&self) -> &'static str {
            "dummy"
        }
        fn execute(
            &self,
            _code: &str,
            _entrypoint: &str,
            input_args: &Value,
            _context: &dyn CapabilityContext,
            _limits: &Limits,
        ) -> Result<ExecutionResult> {
            Ok(ExecutionResult {
                success: true,
                output: serde_json::json!({ "echo": input_args }),
                fuel_consumed: 1,
                execution_time_ns: 100,
                diagnostics: Vec::new(),
            })
        }
    }

    struct DummyContext;
    impl CapabilityContext for DummyContext {
        fn read_file(&self, _path: &str) -> Result<Option<String>> {
            Ok(None)
        }
        fn sha256(&self, data: &str) -> String {
            format!("hash-{}", data)
        }
        fn check_fuel(&self) -> Result<u64> {
            Ok(1000)
        }
        fn fuel_consumed(&self) -> u64 {
            0
        }
    }

    fn sample_doc() -> SkillDocument {
        SkillDocument {
            manifest: SkillManifest {
                asl_version: "3.0".to_string(),
                digest: Some("asl:sha256:test".to_string()),
                signature: None,
                signer_pubkey: None,
                name: "test-tool".to_string(),
                version: Some("1.0.0".to_string()),
                description: "A test tool".to_string(),
                license: Some("MIT".to_string()),
                interface: SkillInterface {
                    protocol: "mcp-tool-v1".to_string(),
                    entrypoint: "run".to_string(),
                    input_schema: serde_json::json!({ "type": "object" }),
                    output_schema: None,
                },
                capabilities: Default::default(),
                limits: Default::default(),
            },
            semantic_section: "Test".to_string(),
            deterministic_code: "def run(ctx, input): return input".to_string(),
            rules_code: None,
            digest: "asl:sha256:test".to_string(),
        }
    }

    #[test]
    fn test_mcp_initialize() {
        let engine = DummyEngine;
        let ctx = DummyContext;
        let server = McpServer::new(vec![sample_doc()], &engine, &ctx);

        let msg = r#"{"jsonrpc":"2.0","id":1,"method":"initialize","params":{}}"#;
        let resp = server.handle_message(msg).expect("Must return response");
        assert_eq!(resp.id, Some(Value::from(1)));
        assert!(resp.error.is_none());
        let res = resp.result.unwrap();
        assert_eq!(res["protocolVersion"], "2024-11-05");
    }

    #[test]
    fn test_mcp_tools_list_and_call() {
        let engine = DummyEngine;
        let ctx = DummyContext;
        let server = McpServer::new(vec![sample_doc()], &engine, &ctx);

        // tools/list
        let list_msg = r#"{"jsonrpc":"2.0","id":2,"method":"tools/list"}"#;
        let list_resp = server.handle_message(list_msg).unwrap();
        let tools = list_resp.result.unwrap()["tools"].as_array().unwrap().clone();
        assert_eq!(tools.len(), 1);
        assert_eq!(tools[0]["name"], "test-tool");

        // tools/call success
        let call_msg = r#"{"jsonrpc":"2.0","id":3,"method":"tools/call","params":{"name":"test-tool","arguments":{"msg":"hello"}}}"#;
        let call_resp = server.handle_message(call_msg).unwrap();
        assert!(call_resp.error.is_none());
        let content = call_resp.result.unwrap()["content"][0]["text"].as_str().unwrap().to_string();
        assert!(content.contains("echo"));

        // tools/call not found
        let not_found_msg = r#"{"jsonrpc":"2.0","id":4,"method":"tools/call","params":{"name":"unknown"}}"#;
        let not_found_resp = server.handle_message(not_found_msg).unwrap();
        assert!(not_found_resp.error.is_some());
    }

    #[test]
    fn test_mcp_invalid_json_and_unknown_method() {
        let engine = DummyEngine;
        let ctx = DummyContext;
        let server = McpServer::new(vec![], &engine, &ctx);

        let bad_json = server.handle_message("invalid json").unwrap();
        assert_eq!(bad_json.error.unwrap()["code"], -32700);

        let unknown = server.handle_message(r#"{"jsonrpc":"2.0","id":9,"method":"foo"}"#).unwrap();
        assert_eq!(unknown.error.unwrap()["code"], -32601);

        // Notification without id should not generate a response
        let notif = server.handle_message(r#"{"jsonrpc":"2.0","method":"unknown/notification"}"#);
        assert!(notif.is_none());

        // Ping must return success with empty object
        let ping_resp = server.handle_message(r#"{"jsonrpc":"2.0","id":10,"method":"ping"}"#).unwrap();
        assert!(ping_resp.error.is_none());
        assert_eq!(ping_resp.result.unwrap(), serde_json::json!({}));
    }

    #[test]
    fn test_mcp_stdio_loop() {
        let engine = DummyEngine;
        let ctx = DummyContext;
        let server = McpServer::new(vec![sample_doc()], &engine, &ctx);

        let input_stream = "{\"jsonrpc\":\"2.0\",\"id\":1,\"method\":\"tools/list\"}\n";
        let reader = Cursor::new(input_stream);
        let mut output = Vec::new();

        server.run_stdio_loop(reader, &mut output).expect("stdio loop should succeed");
        let output_str = String::from_utf8(output).unwrap();
        assert!(output_str.contains("test-tool"));
    }
}
