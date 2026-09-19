use asl_core_traits::{CapabilityContext, EnginePort, HttpResponsePayload};
use asl_spec::{Limits, Result};
use asl_vm_starlark::StarlarkEngine;

struct DummyContext;
impl CapabilityContext for DummyContext {
    fn read_file(&self, _path: &str) -> Result<Option<String>> {
        Ok(None)
    }
    fn sha256(&self, data: &str) -> String {
        format!("hash-{}", data)
    }
    fn base64_encode(&self, data: &str) -> String {
        format!("b64-{}", data)
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
    ) -> Result<HttpResponsePayload> {
        Ok(HttpResponsePayload {
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

#[test]
fn test_starlark_engine_deterministic_execution() {
    let engine = StarlarkEngine::new();
    let ctx = DummyContext;
    let limits = Limits::default();

    let code = r#"
def format_commit(ctx, input):
    intent = input.get("intent", "")
    return {
        "valid": True,
        "msg": "feat: " + intent
    }
"#;

    let input_args = serde_json::json!({
        "intent": "add user authentication"
    });

    let result = engine
        .execute(code, "format_commit", &input_args, &ctx, &limits)
        .expect("Starlark execution should succeed");

    assert!(result.success);
    assert_eq!(result.output["valid"], true);
    assert_eq!(result.output["msg"], "feat: add user authentication");
}

#[test]
fn test_starlark_engine_capability_context_stdlib() {
    struct MockCtx;
    impl CapabilityContext for MockCtx {
        fn read_file(&self, path: &str) -> Result<Option<String>> {
            if path == "package.json" {
                Ok(Some(r#"{"name": "test-pkg"}"#.to_string()))
            } else {
                Ok(None)
            }
        }
        fn sha256(&self, data: &str) -> String {
            format!("sha256:{}", data)
        }
        fn base64_encode(&self, data: &str) -> String {
            format!("b64:{}", data)
        }
        fn base64_decode(&self, encoded: &str) -> Result<String> {
            Ok(encoded.to_string())
        }
        fn env_var(&self, key: &str) -> Result<Option<String>> {
            if key == "API_KEY" {
                Ok(Some("secret123".to_string()))
            } else {
                Ok(None)
            }
        }
        fn http_request(
            &self,
            _method: &str,
            _url: &str,
            _headers: &[(String, String)],
            _body: Option<&str>,
        ) -> Result<HttpResponsePayload> {
            Ok(HttpResponsePayload {
                status: 200,
                headers: vec![("Content-Type".to_string(), "application/json".to_string())],
                body: r#"{"status": "ok", "items": [1, 2]}"#.to_string(),
            })
        }
        fn check_fuel(&self) -> Result<u64> {
            Ok(1000)
        }
        fn fuel_consumed(&self) -> u64 {
            42
        }
    }

    let engine = StarlarkEngine::new();
    let ctx = MockCtx;
    let limits = Limits::default();

    let code = r#"
def inspect_system(ctx, input):
    content = ctx.fs.read(input["target_file"])
    digest = ctx.crypto.sha256(input["target_file"])
    token = ctx.env.get("API_KEY")
    b64 = ctx.crypto.base64_encode(token)
    resp = ctx.http.post("https://api.github.com/test", json={"msg": "ping"})
    data = resp.json()
    header_val = resp.headers["content-type"]
    return {
        "file_content": content,
        "digest": digest,
        "token": token,
        "b64": b64,
        "resp_status": resp.status,
        "resp_ok": data["status"] == "ok",
        "content_type": header_val,
    }
"#;

    let input_args = serde_json::json!({
        "target_file": "package.json"
    });

    let res = engine
        .execute(code, "inspect_system", &input_args, &ctx, &limits)
        .expect("Execution with capabilities should succeed");

    assert!(res.success);
    assert_eq!(res.output["file_content"], r#"{"name": "test-pkg"}"#);
    assert_eq!(res.output["digest"], "sha256:package.json");
    assert_eq!(res.output["token"], "secret123");
    assert_eq!(res.output["b64"], "b64:secret123");
    assert_eq!(res.output["resp_status"], 200);
    assert_eq!(res.output["resp_ok"], true);
    assert_eq!(res.output["content_type"], "application/json");
}

#[test]
fn test_starlark_engine_http_verbs_and_empty_json() {
    struct VerbCtx;
    impl CapabilityContext for VerbCtx {
        fn read_file(&self, _path: &str) -> Result<Option<String>> {
            Ok(None)
        }
        fn sha256(&self, data: &str) -> String {
            data.to_string()
        }
        fn base64_encode(&self, data: &str) -> String {
            data.to_string()
        }
        fn base64_decode(&self, encoded: &str) -> Result<String> {
            Ok(encoded.to_string())
        }
        fn env_var(&self, _key: &str) -> Result<Option<String>> {
            Ok(None)
        }
        fn http_request(
            &self,
            method: &str,
            _url: &str,
            _headers: &[(String, String)],
            _body: Option<&str>,
        ) -> Result<HttpResponsePayload> {
            if method == "DELETE" {
                Ok(HttpResponsePayload {
                    status: 204,
                    headers: vec![],
                    body: "".to_string(),
                })
            } else {
                Ok(HttpResponsePayload {
                    status: 200,
                    headers: vec![("X-Method".to_string(), method.to_string())],
                    body: format!(r#"{{"method": "{}"}}"#, method),
                })
            }
        }
        fn check_fuel(&self) -> Result<u64> {
            Ok(1000)
        }
        fn fuel_consumed(&self) -> u64 {
            10
        }
    }

    let engine = StarlarkEngine::new();
    let ctx = VerbCtx;
    let limits = Limits::default();

    let code = r#"
def test_verbs(ctx, input):
    r_put = ctx.http.put("https://api.github.com/put", json={"a": 1})
    r_patch = ctx.http.patch("https://api.github.com/patch", json={"b": 2})
    r_del = ctx.http.delete("https://api.github.com/delete")
    return {
        "put_method": r_put.json()["method"],
        "patch_method": r_patch.json()["method"],
        "del_status": r_del.status,
        "del_json_is_none": r_del.json() == None,
    }
"#;

    let res = engine
        .execute(code, "test_verbs", &serde_json::json!({}), &ctx, &limits)
        .expect("HTTP verbs execution should succeed");

    assert!(res.success);
    assert_eq!(res.output["put_method"], "PUT");
    assert_eq!(res.output["patch_method"], "PATCH");
    assert_eq!(res.output["del_status"], 204);
    assert_eq!(res.output["del_json_is_none"], true);
}

#[test]
fn test_ambient_authority_eradication() {
    let engine = StarlarkEngine::new();
    let ctx = DummyContext;
    let limits = Limits::default();
    let script = "def run(ctx, input):\n    return asl_native_fs_read('test.txt')";
    let res = engine.execute(script, "run", &serde_json::json!({}), &ctx, &limits);
    assert!(res.is_err(), "Calling ambient asl_native_* directly must fail");
}

