use asl_core_traits::{CapabilityContext, EnginePort};
use asl_spec::{AslError, ExecutionResult, Limits, Result};
use serde_json::Value;
use starlark::environment::{GlobalsBuilder, LibraryExtension, Module};
use starlark::eval::Evaluator;
use starlark::starlark_module;
use starlark::syntax::{AstModule, Dialect};
use starlark::values::ProvidesStaticType;
use std::collections::HashMap;
use std::time::Instant;

#[derive(ProvidesStaticType)]
struct StarlarkContextExtra<'a> {
    context: &'a dyn CapabilityContext,
    limits: &'a Limits,
}

use starlark::values::none::{NoneOr, NoneType};

mod sanitizer;

#[starlark_module]
fn asl_natives(builder: &mut GlobalsBuilder) {
    fn asl_native_fs_read(path: &str, eval: &mut Evaluator) -> anyhow::Result<NoneOr<String>> {
        let extra = eval
            .extra
            .and_then(|e| e.downcast_ref::<StarlarkContextExtra>())
            .ok_or_else(|| anyhow::anyhow!("ASL context not configured"))?;
        match extra.context.read_file(path) {
            Ok(Some(content)) => Ok(NoneOr::Other(content)),
            Ok(None) => Ok(NoneOr::None),
            Err(e) => Err(anyhow::anyhow!("Ocap permission error: {}", e)),
        }
    }

    fn asl_native_fs_write(path: &str, content: &str, eval: &mut Evaluator) -> anyhow::Result<NoneType> {
        let extra = eval
            .extra
            .and_then(|e| e.downcast_ref::<StarlarkContextExtra>())
            .ok_or_else(|| anyhow::anyhow!("ASL context not configured"))?;
        extra
            .context
            .write_file(path, content)
            .map_err(|e| anyhow::anyhow!("Ocap permission error: {}", e))?;
        Ok(NoneType)
    }

    fn asl_native_fs_exists(path: &str, eval: &mut Evaluator) -> anyhow::Result<bool> {
        let extra = eval
            .extra
            .and_then(|e| e.downcast_ref::<StarlarkContextExtra>())
            .ok_or_else(|| anyhow::anyhow!("ASL context not configured"))?;
        Ok(extra.context.file_exists(path))
    }

    fn asl_native_fs_list(path: &str, eval: &mut Evaluator) -> anyhow::Result<Vec<String>> {
        let extra = eval
            .extra
            .and_then(|e| e.downcast_ref::<StarlarkContextExtra>())
            .ok_or_else(|| anyhow::anyhow!("ASL context not configured"))?;
        extra
            .context
            .list_dir(path)
            .map_err(|e| anyhow::anyhow!("Ocap permission error: {}", e))
    }

    fn asl_native_chars(s: &str) -> anyhow::Result<Vec<String>> {
        Ok(s.chars().map(|c| c.to_string()).collect())
    }

    fn asl_native_sha256(data: &str, eval: &mut Evaluator) -> anyhow::Result<String> {
        let extra = eval
            .extra
            .and_then(|e| e.downcast_ref::<StarlarkContextExtra>())
            .ok_or_else(|| anyhow::anyhow!("ASL context not configured"))?;
        Ok(extra.context.sha256(data))
    }

    fn asl_native_base64_encode(data: &str, eval: &mut Evaluator) -> anyhow::Result<String> {
        let extra = eval
            .extra
            .and_then(|e| e.downcast_ref::<StarlarkContextExtra>())
            .ok_or_else(|| anyhow::anyhow!("ASL context not configured"))?;
        Ok(extra.context.base64_encode(data))
    }

    fn asl_native_base64_decode(encoded: &str, eval: &mut Evaluator) -> anyhow::Result<String> {
        let extra = eval
            .extra
            .and_then(|e| e.downcast_ref::<StarlarkContextExtra>())
            .ok_or_else(|| anyhow::anyhow!("ASL context not configured"))?;
        extra
            .context
            .base64_decode(encoded)
            .map_err(|e| anyhow::anyhow!("Base64 decode error: {}", e))
    }

    fn asl_native_env_get(key: &str, eval: &mut Evaluator) -> anyhow::Result<NoneOr<String>> {
        let extra = eval
            .extra
            .and_then(|e| e.downcast_ref::<StarlarkContextExtra>())
            .ok_or_else(|| anyhow::anyhow!("ASL context not configured"))?;
        match extra.context.env_var(key) {
            Ok(Some(v)) => Ok(NoneOr::Other(v)),
            Ok(None) => Ok(NoneOr::None),
            Err(e) => Err(anyhow::anyhow!("Ocap permission error: {}", e)),
        }
    }

    fn asl_native_http_request(
        method: &str,
        url: &str,
        headers_json: &str,
        body: NoneOr<&str>,
        eval: &mut Evaluator,
    ) -> anyhow::Result<String> {
        let extra = eval
            .extra
            .and_then(|e| e.downcast_ref::<StarlarkContextExtra>())
            .ok_or_else(|| anyhow::anyhow!("ASL context not configured"))?;

        let parsed_headers: HashMap<String, String> =
            serde_json::from_str(headers_json).unwrap_or_default();
        let header_vec: Vec<(String, String)> = parsed_headers.into_iter().collect();
        let body_opt = match body {
            NoneOr::Other(b) => Some(b),
            NoneOr::None => None,
        };

        match extra
            .context
            .http_request(method, url, &header_vec, body_opt)
        {
            Ok(payload) => {
                let mut header_map = HashMap::new();
                for (k, v) in payload.headers {
                    header_map.insert(k.to_lowercase(), v.clone());
                    header_map.insert(k, v);
                }
                let json_res = serde_json::json!({
                    "status": payload.status,
                    "body": payload.body,
                    "headers": header_map,
                });
                Ok(json_res.to_string())
            }
            Err(e) => Err(anyhow::anyhow!("HTTP error: {}", e)),
        }
    }

    fn asl_native_fuel_consumed(eval: &mut Evaluator) -> anyhow::Result<u64> {
        let extra = eval
            .extra
            .and_then(|e| e.downcast_ref::<StarlarkContextExtra>())
            .ok_or_else(|| anyhow::anyhow!("ASL context not configured"))?;
        Ok(extra.context.fuel_consumed())
    }

    fn asl_native_fuel_remaining(eval: &mut Evaluator) -> anyhow::Result<u64> {
        let extra = eval
            .extra
            .and_then(|e| e.downcast_ref::<StarlarkContextExtra>())
            .ok_or_else(|| anyhow::anyhow!("ASL context not configured"))?;
        Ok(extra
            .limits
            .max_fuel_opcodes
            .saturating_sub(extra.context.fuel_consumed()))
    }

    fn _asl_matches_regex(haystack: &str, pattern: &str) -> anyhow::Result<bool> {
        let re = regex::Regex::new(pattern)
            .map_err(|e| anyhow::anyhow!("Invalid regular expression '{}': {}", pattern, e))?;
        Ok(re.is_match(haystack))
    }
}

pub struct StarlarkEngine;

impl StarlarkEngine {
    pub fn new() -> Self {
        Self
    }
}

impl Default for StarlarkEngine {
    fn default() -> Self {
        Self::new()
    }
}

impl EnginePort for StarlarkEngine {
    fn name(&self) -> &'static str {
        "starlark-hermetic"
    }

    fn execute(
        &self,
        code: &str,
        entrypoint: &str,
        input_args: &Value,
        context: &dyn CapabilityContext,
        limits: &Limits,
    ) -> Result<ExecutionResult> {
        let start_time = Instant::now();

        let dialect = Dialect::Standard;
        let mut globals_builder = GlobalsBuilder::standard().with(asl_natives);
        for ext in [
            LibraryExtension::Json,
            LibraryExtension::StructType,
            LibraryExtension::Map,
            LibraryExtension::Filter,
        ] {
            ext.add(&mut globals_builder);
        }
        let globals = globals_builder.build();

        let input_json_str = serde_json::to_string(input_args).map_err(AslError::Json)?;

        let invocation_script = format!(
            r#"
{code}

# Helper functions for Sandboxed HTTP & Environment Capabilities
def _asl_make_resp(raw_resp):
    _status = raw_resp["status"]
    _body = raw_resp["body"]
    _headers = raw_resp["headers"]
    def _json_decode_body():
        if _body == None or _body.strip() == "":
            return None
        return json.decode(_body)
    return struct(
        status = _status,
        text = _body,
        headers = _headers,
        json = _json_decode_body,
    )

def _asl_http_call(method, url, headers=None, json_data=None, data=None):
    body_str = None
    if json_data != None:
        body_str = json.encode(json_data)
        if headers == None:
            headers = {{"Content-Type": "application/json"}}
        elif "Content-Type" not in headers and "content-type" not in headers:
            headers["Content-Type"] = "application/json"
    elif data != None:
        body_str = data

    headers_json = json.encode(headers if headers != None else {{}})
    raw_str = asl_native_http_request(method, url, headers_json, body_str)
    return _asl_make_resp(json.decode(raw_str))

def _asl_http_get(url, headers=None):
    return _asl_http_call("GET", url, headers=headers)

def _asl_http_post(url, headers=None, json=None, data=None):
    return _asl_http_call("POST", url, headers=headers, json_data=json, data=data)

def _asl_http_put(url, headers=None, json=None, data=None):
    return _asl_http_call("PUT", url, headers=headers, json_data=json, data=data)

def _asl_http_patch(url, headers=None, json=None, data=None):
    return _asl_http_call("PATCH", url, headers=headers, json_data=json, data=data)

def _asl_http_delete(url, headers=None):
    return _asl_http_call("DELETE", url, headers=headers)

def _asl_env_get(key, default=None):
    val = asl_native_env_get(key)
    return val if val != None else default

# Ergonomic string and safe conversion standard library
def chars(s):
    if s == None:
        return []
    return asl_native_chars(str(s))

def is_digit(s):
    if s == None:
        return False
    st = str(s)
    return len(st) > 0 and st.isdigit()

def is_int(s):
    if s == None:
        return False
    st = str(s).strip()
    if len(st) == 0:
        return False
    if st.startswith("-"):
        st = st[1:]
    return len(st) > 0 and st.isdigit()

def to_int(s, default=None):
    if is_int(s):
        return int(str(s).strip())
    return default

def to_float(s, default=None):
    if s == None:
        return default
    st = str(s).strip()
    parts = st.split(".")
    if len(parts) == 1 and is_int(parts[0]):
        return float(st)
    if len(parts) == 2 and (is_int(parts[0]) or parts[0] == "" or parts[0] == "-") and parts[1].isdigit():
        return float(st)
    return default

# Deterministic wrapper with Capability injection
asl_raw_input = json.decode({input_json:?})
asl_ctx = struct(
    fs = struct(
        read = asl_native_fs_read,
        write = asl_native_fs_write,
        exists = asl_native_fs_exists,
        list = asl_native_fs_list,
    ),
    crypto = struct(
        sha256 = asl_native_sha256,
        base64_encode = asl_native_base64_encode,
        base64_decode = asl_native_base64_decode,
    ),
    env = struct(get = _asl_env_get),
    http = struct(
        get = _asl_http_get,
        post = _asl_http_post,
        put = _asl_http_put,
        patch = _asl_http_patch,
        delete = _asl_http_delete,
        call = _asl_http_call,
    ),
    fuel = struct(consumed = asl_native_fuel_consumed, remaining = asl_native_fuel_remaining),
)
asl_result = {entrypoint}(asl_ctx, asl_raw_input)
asl_output_json = json.encode(asl_result)
"#,
            code = code,
            input_json = input_json_str,
            entrypoint = entrypoint
        );

        let ast = AstModule::parse("ASL Code", invocation_script, &dialect)
            .map_err(|e| AslError::StarlarkError(sanitizer::sanitize_error(&e.to_string())))?;

        let context_extra = StarlarkContextExtra { context, limits };

        Module::with_temp_heap(|module| {
            let mut eval = Evaluator::new(&module);
            eval.extra = Some(&context_extra);
            eval.eval_module(ast, &globals).map_err(|e| {
                AslError::StarlarkError(sanitizer::sanitize_error(&e.to_string()))
            })?;

            let output_val = module
                .get("asl_output_json")
                .ok_or_else(|| AslError::EntrypointNotFound(entrypoint.to_string()))?;

            let output_str = output_val.unpack_str().ok_or_else(|| {
                AslError::StarlarkError("asl_output_json output is not a string".to_string())
            })?;

            let parsed_output: Value = serde_json::from_str(output_str).map_err(AslError::Json)?;

            let duration = start_time.elapsed();
            let consumed = context.fuel_consumed().max(1);

            Ok(ExecutionResult {
                success: true,
                output: parsed_output,
                fuel_consumed: consumed,
                execution_time_ns: duration.as_nanos() as u64,
                diagnostics: Vec::new(),
            })
        })
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_starlark_engine_creation() {
        let engine = StarlarkEngine;
        assert_eq!(engine.name(), "starlark-hermetic");
    }
}
