use asl_core_traits::{CapabilityContext, EnginePort};
use asl_protocol_mcp::McpServer;
use asl_spec::{ExecutionResult, Limits, Result, SkillDocument, SkillInterface, SkillManifest};
use serde_json::Value;
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
    fn sha256(&self, data: &str) -> Result<String> {
        Ok(format!("hash-{}", data))
    }
    fn base64_encode(&self, data: &str) -> Result<String> {
        Ok(data.to_string())
    }
    fn base64_decode(&self, encoded: &str) -> Result<String> {
        Ok(encoded.to_string())
    }
    fn env_var(&self, _key: &str) -> Result<Option<String>> {
        Ok(None)
    }
    fn http_request(
        &self,
        _method: &str,
        _url: &str,
        _headers: &[(String, String)],
        _body: Option<&str>,
    ) -> Result<asl_core_traits::HttpResponsePayload> {
        Ok(asl_core_traits::HttpResponsePayload {
            status: 200,
            headers: vec![],
            body: "{}".to_string(),
        })
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
                input_schema: serde_json::json!({
                    "type": "object",
                    "properties": {
                        "msg": { "type": "string" }
                    }
                }),
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
fn test_mcp_input_schema_validation_rejection() {
    let engine = DummyEngine;
    let ctx = DummyContext;
    let server = McpServer::new(vec![sample_doc()], &engine, &ctx);

    // tools/call with invalid schema type ("msg" is expected to be a string, pass number)
    let call_msg = r#"{"jsonrpc":"2.0","id":5,"method":"tools/call","params":{"name":"test-tool","arguments":{"msg": 12345}}}"#;
    let call_resp = server.handle_message(call_msg).unwrap();
    assert!(call_resp.error.is_some());
    let err = call_resp.error.unwrap();
    assert_eq!(err["code"], -32602);
    assert!(err["message"].as_str().unwrap().contains("Input schema validation error"));
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
