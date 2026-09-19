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

mod prelude;
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
        extra.context.file_exists(path).map_err(|e| anyhow::anyhow!("Fuel/Ocap error: {}", e))
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

    fn asl_native_sha256(data: &str, eval: &mut Evaluator) -> anyhow::Result<String> {
        let extra = eval
            .extra
            .and_then(|e| e.downcast_ref::<StarlarkContextExtra>())
            .ok_or_else(|| anyhow::anyhow!("ASL context not configured"))?;
        extra.context.sha256(data).map_err(|e| anyhow::anyhow!("Fuel limit exceeded: {}", e))
    }

    fn asl_native_base64_encode(data: &str, eval: &mut Evaluator) -> anyhow::Result<String> {
        let extra = eval
            .extra
            .and_then(|e| e.downcast_ref::<StarlarkContextExtra>())
            .ok_or_else(|| anyhow::anyhow!("ASL context not configured"))?;
        extra.context.base64_encode(data).map_err(|e| anyhow::anyhow!("Fuel limit exceeded: {}", e))
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
}

#[starlark_module]
fn asl_pure_natives(builder: &mut GlobalsBuilder) {
    fn asl_native_chars(s: &str) -> anyhow::Result<Vec<String>> {
        Ok(s.chars().map(|c| c.to_string()).collect())
    }

    fn asl_native_is_alpha(s: &str) -> anyhow::Result<bool> {
        Ok(!s.is_empty() && s.chars().all(|c| c.is_alphabetic()))
    }

    fn asl_native_is_alnum(s: &str) -> anyhow::Result<bool> {
        Ok(!s.is_empty() && s.chars().all(|c| c.is_alphanumeric()))
    }

    fn asl_native_is_digit(s: &str) -> anyhow::Result<bool> {
        Ok(!s.is_empty() && s.chars().all(|c| c.is_ascii_digit()))
    }

    fn asl_native_is_json(s: &str) -> anyhow::Result<bool> {
        Ok(serde_json::from_str::<serde_json::Value>(s).is_ok())
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

fn build_globals(with_host: bool) -> starlark::environment::Globals {
    let mut b = GlobalsBuilder::standard().with(asl_pure_natives);
    if with_host {
        b = b.with(asl_natives);
    }
    for ext in [LibraryExtension::Json, LibraryExtension::StructType, LibraryExtension::Map, LibraryExtension::Filter] {
        ext.add(&mut b);
    }
    b.build()
}

impl EnginePort for StarlarkEngine {
    fn name(&self) -> &'static str {
        "starlark-hermetic"
    }

    fn validate(&self, code: &str, entrypoint: &str) -> Result<()> {
        let dialect = Dialect::Standard;
        let ast = AstModule::parse("ASL Code", code.to_string(), &dialect)
            .map_err(|e| AslError::StarlarkError(sanitizer::sanitize_error(&e.to_string())))?;
        let user_globals = build_globals(false);
        Module::with_temp_heap(|m| {
            let mut eval = Evaluator::new(&m);
            eval.eval_module(ast, &user_globals)
                .map_err(|e| AslError::StarlarkError(sanitizer::sanitize_error(&e.to_string())))?;
            match m.get(entrypoint) {
                Some(val) if val.get_type() == "function" => Ok(()),
                Some(_) => Err(AslError::StarlarkError(format!(
                    "Entrypoint '{}' is declared but is not a function",
                    entrypoint
                ))),
                None => Err(AslError::EntrypointNotFound(entrypoint.to_string())),
            }
        })
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
        let timeout_dur = std::time::Duration::from_millis(limits.wall_clock_timeout_ms);
        let dialect = Dialect::Standard;
        let host_globals = build_globals(true);
        let user_globals = build_globals(false);

        let context_extra = StarlarkContextExtra { context, limits };

        let host_frozen = Module::with_temp_heap(|host_module| -> Result<starlark::environment::FrozenModule> {
            {
                let mut host_eval = Evaluator::new(&host_module);
                host_eval.extra = Some(&context_extra);
                let prelude_ast = AstModule::parse("host_prelude", prelude::host_prelude_script().to_string(), &dialect)
                    .map_err(|e| AslError::StarlarkError(sanitizer::sanitize_error(&e.to_string())))?;
                host_eval.eval_module(prelude_ast, &host_globals).map_err(|e| {
                    AslError::StarlarkError(sanitizer::sanitize_error(&e.to_string()))
                })?;
            }
            host_module.freeze().map_err(|e| {
                AslError::StarlarkError(format!("Failed to freeze host module: {:?}", e))
            })
        })?;
        let ctx_frozen = host_frozen.get("asl_ctx").map_err(|e| {
            AslError::StarlarkError(format!("Failed to initialize capability context: {}", e))
        })?;

        Module::with_temp_heap(|user_module| {
            let ctx_val = unsafe {
                user_module.frozen_heap().add_reference(ctx_frozen.owner());
                starlark::values::Value::new_frozen(ctx_frozen.unchecked_frozen_value())
            };

            let mut eval = Evaluator::new(&user_module);
            eval.extra = Some(&context_extra);

            if limits.max_fuel_opcodes > 0 {
                let _ = eval.set_max_tick_count(limits.max_fuel_opcodes);
            }
            if limits.max_heap_kib > 0 {
                let _ = eval.set_max_heap_size((limits.max_heap_kib * 1024) as usize);
            }
            eval.set_check_cancelled(Box::new(move || start_time.elapsed() > timeout_dur));

            let stdlib_ast = AstModule::parse("pure_stdlib", prelude::pure_stdlib_script().to_string(), &dialect)
                .map_err(|e| AslError::StarlarkError(sanitizer::sanitize_error(&e.to_string())))?;
            eval.eval_module(stdlib_ast, &user_globals).map_err(|e| {
                AslError::StarlarkError(sanitizer::sanitize_error(&e.to_string()))
            })?;

            let user_ast = AstModule::parse("ASL Code", code.to_string(), &dialect)
                .map_err(|e| AslError::StarlarkError(sanitizer::sanitize_error(&e.to_string())))?;
            eval.eval_module(user_ast, &user_globals).map_err(|e| {
                if start_time.elapsed() > timeout_dur {
                    AslError::LimitExceeded(format!("Wall-clock timeout of {}ms exceeded", limits.wall_clock_timeout_ms))
                } else {
                    AslError::StarlarkError(sanitizer::sanitize_error(&e.to_string()))
                }
            })?;

            let entrypoint_fn = user_module.get(entrypoint).ok_or_else(|| {
                AslError::EntrypointNotFound(entrypoint.to_string())
            })?;

            let input_json_str = serde_json::to_string(input_args).map_err(AslError::Json)?;
            let input_ast = AstModule::parse(
                "input",
                format!("json.decode({})", serde_json::to_string(&input_json_str).map_err(AslError::Json)?),
                &dialect,
            ).map_err(|e| AslError::StarlarkError(sanitizer::sanitize_error(&e.to_string())))?;
            let input_val = eval.eval_module(input_ast, &user_globals).map_err(|e| {
                AslError::StarlarkError(sanitizer::sanitize_error(&e.to_string()))
            })?;

            let result_val = eval.eval_function(entrypoint_fn, &[ctx_val, input_val], &[]).map_err(|e| {
                if start_time.elapsed() > timeout_dur {
                    AslError::LimitExceeded(format!("Wall-clock timeout of {}ms exceeded", limits.wall_clock_timeout_ms))
                } else {
                    let err_str = e.to_string();
                    if err_str.contains("Fuel limit exceeded") || err_str.contains("Tick limit exceeded") {
                        AslError::LimitExceeded(err_str)
                    } else {
                        AslError::StarlarkError(sanitizer::sanitize_error(&err_str))
                    }
                }
            })?;

            let output_json_str = result_val.to_json().map_err(|e| {
                AslError::StarlarkError(format!("Failed to serialize execution result: {}", e))
            })?;
            let parsed_output: Value = serde_json::from_str(&output_json_str).map_err(AslError::Json)?;

            let duration = start_time.elapsed();
            let consumed = (context.fuel_consumed() + eval.get_total_tick_count()).max(1);

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

    struct TestCtx;
    impl CapabilityContext for TestCtx {
        fn read_file(&self, p: &str) -> Result<Option<String>> { Ok(if p == "test.txt" { Some("secret".into()) } else { None }) }
        fn file_exists(&self, p: &str) -> Result<bool> { Ok(p == "test.txt") }
        fn sha256(&self, _: &str) -> Result<String> { Ok("h".into()) }
        fn base64_encode(&self, d: &str) -> Result<String> { Ok(d.into()) }
        fn base64_decode(&self, e: &str) -> Result<String> { Ok(e.into()) }
        fn env_var(&self, _: &str) -> Result<Option<String>> { Ok(None) }
        fn http_request(&self, _: &str, _: &str, _: &[(String, String)], _: Option<&str>) -> Result<asl_core_traits::HttpResponsePayload> {
            Ok(asl_core_traits::HttpResponsePayload { status: 200, headers: vec![], body: "{}".into() })
        }
        fn check_fuel(&self) -> Result<u64> { Ok(1_000_000) }
        fn fuel_consumed(&self) -> u64 { 0 }
    }

    #[test]
    fn test_starlark_engine_creation() {
        assert_eq!(StarlarkEngine.name(), "starlark-hermetic");
    }

    #[test]
    fn test_no_ambient_authority_in_user_module() {
        let engine = StarlarkEngine;
        let limits = Limits::default();
        let ctx = TestCtx;

        // 1. Legal access via ctx argument succeeds
        let good_code = "def run(ctx, input):\n    return ctx.fs.read('test.txt')\n";
        let res = engine.execute(good_code, "run", &serde_json::json!({}), &ctx, &limits).unwrap();
        assert_eq!(res.output, "secret");

        // 2. Malicious ambient access to asl_ctx must fail (not in user module scope)
        let bad_code1 = "def run(ctx, input):\n    return asl_ctx.fs.read('test.txt')\n";
        assert!(engine.execute(bad_code1, "run", &serde_json::json!({}), &ctx, &limits).is_err());

        // 3. Malicious ambient access to _asl_http_call must fail
        let bad_code2 = "def run(ctx, input):\n    return _asl_http_call('GET', 'http://evil.com')\n";
        assert!(engine.execute(bad_code2, "run", &serde_json::json!({}), &ctx, &limits).is_err());
    }

    #[test]
    fn test_evaluator_resource_limits() {
        let dialect = Dialect::Standard;
        let globals = GlobalsBuilder::standard().build();

        // 1. Tick limit
        Module::with_temp_heap(|module| {
            let mut eval = Evaluator::new(&module);
            eval.set_max_tick_count(10).unwrap();
            let code = r#"
def loop_ticks():
    count = 0
    for x in range(100):
        count += 1
    return count
"#;
            let ast = AstModule::parse("ticks", code.to_string(), &dialect).unwrap();
            eval.eval_module(ast, &globals).unwrap();
            let func = module.get("loop_ticks").unwrap();
            let res = eval.eval_function(func, &[], &[]);
            assert!(res.is_err(), "Tick count limit must abort execution");
        });

        // 2. Cancellation / timeout
        Module::with_temp_heap(|module| {
            let mut eval = Evaluator::new(&module);
            eval.set_check_cancelled(Box::new(|| true)); // Always cancelled
            let code = r#"
def loop_cancel():
    count = 0
    for x in range(100):
        count += 1
    return count
"#;
            let ast = AstModule::parse("cancel", code.to_string(), &dialect).unwrap();
            let res = eval.eval_module(ast, &globals);
            assert!(res.is_err(), "Cancellation check must abort execution");
        });
    }
}
