use asl_core_traits::{CapabilityContext, EnginePort};
use asl_spec::{AslError, ExecutionResult, Limits, Result};
use serde_json::Value;
use std::time::Instant;
use wasmi::{Config, Engine, Linker, Module, Store, Val, ValType};

pub struct WasmEngine {
    engine: Engine,
}

impl WasmEngine {
    pub fn new() -> Self {
        let mut config = Config::default();
        config.consume_fuel(true);
        Self {
            engine: Engine::new(&config),
        }
    }
}

impl Default for WasmEngine {
    fn default() -> Self {
        Self::new()
    }
}

impl EnginePort for WasmEngine {
    fn name(&self) -> &'static str {
        "wasm-component"
    }

    fn execute(
        &self,
        code: &str,
        entrypoint: &str,
        input_args: &Value,
        _context: &dyn CapabilityContext,
        limits: &Limits,
    ) -> Result<ExecutionResult> {
        let start = Instant::now();
        let wasm_bytes = parse_wasm_source(code)?;

        let module = Module::new(&self.engine, &wasm_bytes[..])
            .map_err(|e| AslError::WasmError(format!("Erro ao compilar módulo WASM: {}", e)))?;

        let mut store = Store::new(&self.engine, ());
        store
            .set_fuel(limits.max_fuel_opcodes)
            .map_err(|e| AslError::WasmError(format!("Falha ao alocar fuel WASM: {}", e)))?;

        let linker = <Linker<()>>::new(&self.engine);
        let instance = linker
            .instantiate_and_start(&mut store, &module)
            .map_err(|e| AslError::WasmError(format!("Falha ao instanciar módulo WASM: {}", e)))?;

        let func = instance
            .get_func(&store, entrypoint)
            .ok_or_else(|| AslError::EntrypointNotFound(entrypoint.to_string()))?;

        let func_ty = func.ty(&store);
        let param_types = func_ty.params();
        let mut args: Vec<Val> = Vec::new();

        if !param_types.is_empty() {
            for (i, p_ty) in param_types.iter().enumerate() {
                let val = if let Some(arr) = input_args.as_array() {
                    arr.get(i)
                } else if let Some(obj) = input_args.as_object() {
                    let key = format!("arg{}", i);
                    obj.get(&key).or_else(|| obj.values().nth(i))
                } else if i == 0 {
                    Some(input_args)
                } else {
                    None
                };
                args.push(json_val_to_wasm_val(val, p_ty));
            }
        }

        let mut results = vec![Val::I32(0); func_ty.results().len()];

        func.call(&mut store, &args, &mut results)
            .map_err(|e| AslError::WasmError(format!("Erro na execução WASM: {}", e)))?;

        let remaining_fuel = store.get_fuel().unwrap_or(0);
        let fuel_consumed = limits.max_fuel_opcodes.saturating_sub(remaining_fuel);
        let execution_time_ns = start.elapsed().as_nanos() as u64;

        let output = if results.is_empty() {
            serde_json::json!({ "status": "ok" })
        } else if results.len() == 1 {
            let out_val = wasm_val_to_json(&results[0]);
            serde_json::json!({ "result": out_val })
        } else {
            let out_arr: Vec<Value> = results.iter().map(wasm_val_to_json).collect();
            serde_json::json!({ "results": out_arr })
        };

        Ok(ExecutionResult {
            success: true,
            output,
            fuel_consumed,
            execution_time_ns,
            diagnostics: Vec::new(),
        })
    }
}

fn parse_wasm_source(code: &str) -> Result<Vec<u8>> {
    let trimmed = code.trim();
    if trimmed.starts_with('(') || trimmed.contains("(module") {
        wat::parse_str(trimmed)
            .map_err(|e| AslError::WasmError(format!("Falha ao parsear WAT: {}", e)))
    } else if let Ok(bytes) = hex::decode(trimmed.strip_prefix("0x").unwrap_or(trimmed)) {
        Ok(bytes)
    } else {
        Err(AslError::WasmError(
            "Formato WASM não reconhecido: deve ser WAT ou bytecode hexadecimal.".to_string(),
        ))
    }
}

fn json_val_to_wasm_val(val: Option<&Value>, ty: &ValType) -> Val {
    match ty {
        ValType::I32 => {
            let n = val.and_then(|v| v.as_i64()).unwrap_or(0) as i32;
            Val::I32(n)
        }
        ValType::I64 => {
            let n = val.and_then(|v| v.as_i64()).unwrap_or(0);
            Val::I64(n)
        }
        ValType::F32 => {
            let n = val.and_then(|v| v.as_f64()).unwrap_or(0.0) as f32;
            Val::F32(n.into())
        }
        ValType::F64 => {
            let n = val.and_then(|v| v.as_f64()).unwrap_or(0.0);
            Val::F64(n.into())
        }
        _ => Val::I32(0),
    }
}

fn wasm_val_to_json(val: &Val) -> Value {
    match val {
        Val::I32(n) => serde_json::json!(n),
        Val::I64(n) => serde_json::json!(n),
        Val::F32(f) => serde_json::json!(f.to_float()),
        Val::F64(f) => serde_json::json!(f.to_float()),
        _ => Value::Null,
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use asl_spec::SkillLimits;

    struct DummyContext;
    impl CapabilityContext for DummyContext {
        fn read_file(&self, _path: &str) -> Result<Option<String>> {
            Ok(None)
        }
        fn sha256(&self, _data: &str) -> String {
            "".to_string()
        }
        fn check_fuel(&self) -> Result<u64> {
            Ok(1_000_000)
        }
        fn fuel_consumed(&self) -> u64 {
            0
        }
    }

    #[test]
    fn test_wasm_wat_addition() {
        let wat_code = r#"
            (module
                (func (export "add") (param i32 i32) (result i32)
                    local.get 0
                    local.get 1
                    i32.add
                )
            )
        "#;

        let engine = WasmEngine::new();
        let limits = SkillLimits::default();
        let ctx = DummyContext;
        let args = serde_json::json!([15, 27]);

        let res = engine
            .execute(wat_code, "add", &args, &ctx, &limits)
            .expect("Execução WASM deve suceder");

        assert_eq!(res.output["result"], 42);
        assert!(res.fuel_consumed > 0);
        assert!(res.execution_time_ns > 0);
    }

    #[test]
    fn test_wasm_named_object_arguments() {
        let wat_code = r#"
            (module
                (func (export "mul") (param i32 i32) (result i32)
                    local.get 0
                    local.get 1
                    i32.mul
                )
            )
        "#;

        let engine = WasmEngine::new();
        let limits = SkillLimits::default();
        let ctx = DummyContext;
        let args = serde_json::json!({"x": 6, "y": 7});

        let res = engine
            .execute(wat_code, "mul", &args, &ctx, &limits)
            .expect("Execução com objeto nomeado deve suceder");

        assert_eq!(res.output["result"], 42);
    }
}
