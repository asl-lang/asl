use asl_core_traits::{CapabilityContext, EnginePort};
use asl_spec::{AslError, ExecutionResult, Limits, Result};
use serde_json::Value;
use starlark::environment::{GlobalsBuilder, LibraryExtension, Module};
use starlark::eval::Evaluator;
use starlark::starlark_module;
use starlark::syntax::{AstModule, Dialect};
use starlark::values::ProvidesStaticType;
use std::time::Instant;

#[derive(ProvidesStaticType)]
struct StarlarkContextExtra<'a> {
    context: &'a dyn CapabilityContext,
    limits: &'a Limits,
}

use starlark::values::none::NoneOr;

#[starlark_module]
fn asl_natives(builder: &mut GlobalsBuilder) {
    fn asl_native_fs_read(path: &str, eval: &mut Evaluator) -> anyhow::Result<NoneOr<String>> {
        let extra = eval
            .extra
            .and_then(|e| e.downcast_ref::<StarlarkContextExtra>())
            .ok_or_else(|| anyhow::anyhow!("Contexto ASL não configurado"))?;
        match extra.context.read_file(path) {
            Ok(Some(content)) => Ok(NoneOr::Other(content)),
            Ok(None) => Ok(NoneOr::None),
            Err(e) => Err(anyhow::anyhow!("Erro de permissão ocap: {}", e)),
        }
    }

    fn asl_native_sha256(data: &str, eval: &mut Evaluator) -> anyhow::Result<String> {
        let extra = eval
            .extra
            .and_then(|e| e.downcast_ref::<StarlarkContextExtra>())
            .ok_or_else(|| anyhow::anyhow!("Contexto ASL não configurado"))?;
        Ok(extra.context.sha256(data))
    }

    fn asl_native_fuel_consumed(eval: &mut Evaluator) -> anyhow::Result<u64> {
        let extra = eval
            .extra
            .and_then(|e| e.downcast_ref::<StarlarkContextExtra>())
            .ok_or_else(|| anyhow::anyhow!("Contexto ASL não configurado"))?;
        Ok(extra.context.fuel_consumed())
    }

    fn asl_native_fuel_remaining(eval: &mut Evaluator) -> anyhow::Result<u64> {
        let extra = eval
            .extra
            .and_then(|e| e.downcast_ref::<StarlarkContextExtra>())
            .ok_or_else(|| anyhow::anyhow!("Contexto ASL não configurado"))?;
        Ok(extra.limits.max_fuel_opcodes.saturating_sub(extra.context.fuel_consumed()))
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

# Wrapper determinístico com injeção de Capabilities (ASL 3.0)
asl_raw_input = json.decode({input_json:?})
asl_ctx = struct(
    fs = struct(read = asl_native_fs_read),
    crypto = struct(sha256 = asl_native_sha256),
    fuel = struct(consumed = asl_native_fuel_consumed, remaining = asl_native_fuel_remaining),
)
asl_result = {entrypoint}(asl_ctx, asl_raw_input)
asl_output_json = json.encode(asl_result)
"#,
            code = code,
            input_json = input_json_str,
            entrypoint = entrypoint
        );

        let ast = AstModule::parse("asl_skill.star", invocation_script, &dialect)
            .map_err(|e| AslError::StarlarkError(format!("Erro de sintaxe no Starlark: {}", e)))?;

        let context_extra = StarlarkContextExtra { context, limits };

        Module::with_temp_heap(|module| {
            let mut eval = Evaluator::new(&module);
            eval.extra = Some(&context_extra);
            eval.eval_module(ast, &globals).map_err(|e| {
                AslError::StarlarkError(format!("Erro na execução Starlark: {}", e))
            })?;

            let output_val = module
                .get("asl_output_json")
                .ok_or_else(|| AslError::EntrypointNotFound(entrypoint.to_string()))?;

            let output_str = output_val
                .unpack_str()
                .ok_or_else(|| {
                    AslError::StarlarkError("Saída de asl_output_json não é string".to_string())
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
    use asl_core_traits::CapabilityContext;

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
            .expect("Execução Starlark deve suceder");

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
    consumed = ctx.fuel.consumed()
    remaining = ctx.fuel.remaining()
    return {
        "file_content": content,
        "digest": digest,
        "consumed": consumed,
        "has_remaining": remaining > 0,
    }
"#;

        let input_args = serde_json::json!({
            "target_file": "package.json"
        });

        let res = engine
            .execute(code, "inspect_system", &input_args, &ctx, &limits)
            .expect("Execução com capabilities deve suceder");

        assert!(res.success);
        assert_eq!(res.output["file_content"], r#"{"name": "test-pkg"}"#);
        assert_eq!(res.output["digest"], "sha256:package.json");
        assert_eq!(res.output["consumed"], 42);
        assert_eq!(res.output["has_remaining"], true);
        assert_eq!(res.fuel_consumed, 42);
    }
}

