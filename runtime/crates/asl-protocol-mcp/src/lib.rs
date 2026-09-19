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

pub type ContextFactory<'a> =
    Box<dyn Fn(&SkillDocument) -> Result<Box<dyn CapabilityContext>, asl_spec::AslError> + Send + Sync + 'a>;

pub struct McpServer<'a> {
    skills: Vec<SkillDocument>,
    engine: &'a dyn EnginePort,
    context: &'a dyn CapabilityContext,
    context_factory: Option<ContextFactory<'a>>,
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
            context_factory: None,
        }
    }

    pub fn with_factory(
        skills: Vec<SkillDocument>,
        engine: &'a dyn EnginePort,
        fallback_context: &'a dyn CapabilityContext,
        factory: impl Fn(&SkillDocument) -> Result<Box<dyn CapabilityContext>, asl_spec::AslError> + Send + Sync + 'a,
    ) -> Self {
        Self {
            skills,
            engine,
            context: fallback_context,
            context_factory: Some(Box::new(factory)),
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
                        "version": env!("CARGO_PKG_VERSION")
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
                        let ephemeral_ctx_box;
                        let exec_context: &dyn CapabilityContext = if let Some(ref factory) = self.context_factory {
                            match factory(skill) {
                                Ok(ctx) => {
                                    ephemeral_ctx_box = ctx;
                                    ephemeral_ctx_box.as_ref()
                                }
                                Err(e) => {
                                    return Some(JsonRpcResponse {
                                        jsonrpc: "2.0".to_string(),
                                        id: req.id,
                                        result: None,
                                        error: Some(serde_json::json!({
                                            "code": -32000,
                                            "message": format!("Capability policy violation: {}", e)
                                        })),
                                    });
                                }
                            }
                        } else {
                            self.context
                        };

                        let executor = asl_core_traits::SkillExecutor::new(self.engine, exec_context);
                        let exec_res = executor.execute(skill, None, &arguments, &skill.manifest.limits);

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
                            Err(e) => {
                                let (code, msg) = match &e {
                                    asl_spec::AslError::SchemaViolation(msg) => (-32602, format!("Input schema validation error: {}", msg)),
                                    _ => (-32000, format!("ASL execution error: {}", e)),
                                };
                                Some(JsonRpcResponse {
                                    jsonrpc: "2.0".to_string(),
                                    id: req.id,
                                    result: None,
                                    error: Some(serde_json::json!({
                                        "code": code,
                                        "message": msg,
                                    })),
                                })
                            }
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

