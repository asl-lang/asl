use asl_core_traits::{EnginePort, ParserPort};
use asl_parser::CommonMarkYamlParser;
use asl_security::ConfinedSecurityContext;
use asl_spec::SkillDocument;
use asl_vm_starlark::StarlarkEngine;
use serde_json::Value;
use std::ffi::{CStr, CString};
use std::os::raw::c_char;
use std::panic::{catch_unwind, AssertUnwindSafe};
use std::time::Instant;

/// Identificador opaco de instância do runtime ASL
#[allow(non_camel_case_types)]
pub struct asl_runtime_t {
    parser: CommonMarkYamlParser,
    engine: StarlarkEngine,
}

/// Identificador opaco de documento de skill carregada
#[allow(non_camel_case_types)]
pub struct asl_skill_t {
    doc: SkillDocument,
}

/// Resultado retornado pela execução in-process
#[repr(C)]
#[allow(non_camel_case_types)]
pub struct asl_exec_result_t {
    pub success: u8,
    pub json_output: *const c_char,
    pub fuel_consumed: u64,
    pub execution_time_ns: u64,
}

/// Inicializa uma nova instância isolada do runtime ASL.
#[no_mangle]
pub extern "C" fn asl_runtime_init() -> *mut asl_runtime_t {
    let result = catch_unwind(AssertUnwindSafe(|| {
        Box::into_raw(Box::new(asl_runtime_t {
            parser: CommonMarkYamlParser::new(),
            engine: StarlarkEngine::new(),
        }))
    }));
    result.unwrap_or(std::ptr::null_mut())
}

/// Libera a instância do runtime ASL.
///
/// # Safety
/// O ponteiro `rt` deve ser nulo ou ter sido obtido previamente por `asl_runtime_init`.
#[no_mangle]
pub unsafe extern "C" fn asl_runtime_free(rt: *mut asl_runtime_t) {
    if !rt.is_null() {
        let _ = catch_unwind(AssertUnwindSafe(|| {
            drop(Box::from_raw(rt));
        }));
    }
}

/// Faz parse e validação de um documento .skill em memória.
///
/// # Safety
/// `skill_source` deve ser um ponteiro válido para string C terminada em nulo (UTF-8).
#[no_mangle]
pub unsafe extern "C" fn asl_skill_load(
    rt: *mut asl_runtime_t,
    skill_source: *const c_char,
) -> *mut asl_skill_t {
    let result = catch_unwind(AssertUnwindSafe(|| {
        if rt.is_null() || skill_source.is_null() {
            return std::ptr::null_mut();
        }
        let rt_ref = &*rt;
        let src_str = match CStr::from_ptr(skill_source).to_str() {
            Ok(s) => s,
            Err(_) => return std::ptr::null_mut(),
        };

        match rt_ref.parser.parse(src_str) {
            Ok(doc) => Box::into_raw(Box::new(asl_skill_t { doc })),
            Err(_) => std::ptr::null_mut(),
        }
    }));
    result.unwrap_or(std::ptr::null_mut())
}

/// Libera a skill compilada.
///
/// # Safety
/// O ponteiro `skill` deve ser nulo ou ter sido obtido por `asl_skill_load`.
#[no_mangle]
pub unsafe extern "C" fn asl_skill_free(skill: *mut asl_skill_t) {
    if !skill.is_null() {
        let _ = catch_unwind(AssertUnwindSafe(|| {
            drop(Box::from_raw(skill));
        }));
    }
}

/// Executa um entrypoint da skill com argumentos JSON e barreira de pânico.
///
/// # Safety
/// Os ponteiros passados devem ser válidos ou nulos. Strings devem ser terminadas em nulo (UTF-8).
#[no_mangle]
pub unsafe extern "C" fn asl_skill_execute(
    rt: *mut asl_runtime_t,
    skill: *mut asl_skill_t,
    entrypoint: *const c_char,
    json_args: *const c_char,
) -> *mut asl_exec_result_t {
    let result = catch_unwind(AssertUnwindSafe(|| {
        let start = Instant::now();

        if rt.is_null() || skill.is_null() {
            return make_error_result("Null Runtime or Skill supplied for execution.", 0);
        }

        let rt_ref = &*rt;
        let skill_ref = &*skill;

        let ep = if entrypoint.is_null() {
            skill_ref.doc.manifest.interface.entrypoint.clone()
        } else {
            match CStr::from_ptr(entrypoint).to_str() {
                Ok(s) if !s.is_empty() => s.to_string(),
                _ => skill_ref.doc.manifest.interface.entrypoint.clone(),
            }
        };

        let args_val: Value = if json_args.is_null() {
            Value::Object(serde_json::Map::new())
        } else {
            let str_res = CStr::from_ptr(json_args).to_str();
            match str_res {
                Ok(s) => match serde_json::from_str(s) {
                    Ok(v) => v,
                    Err(e) => {
                        return make_error_result(&format!("JSON arguments invalid: {}", e), 0);
                    }
                },
                Err(e) => {
                    return make_error_result(&format!("JSON arguments not valid UTF-8: {}", e), 0);
                }
            }
        };

        let security = ConfinedSecurityContext::from_capabilities(
            &skill_ref.doc.manifest.capabilities,
            skill_ref.doc.manifest.limits.max_fuel_opcodes,
        );

        match rt_ref.engine.execute(
            &skill_ref.doc.deterministic_code,
            &ep,
            &args_val,
            &security,
            &skill_ref.doc.manifest.limits,
        ) {
            Ok(exec_res) => {
                let duration_ns = start.elapsed().as_nanos() as u64;
                let json_text = serde_json::to_string(&exec_res.output)
                    .unwrap_or_else(|_| "{}".to_string());
                let sanitized = json_text.replace('\0', "\\u0000");
                let c_json = CString::new(sanitized).unwrap_or_else(|_| CString::new("{}").unwrap());
                Box::into_raw(Box::new(asl_exec_result_t {
                    success: 1,
                    json_output: c_json.into_raw(),
                    fuel_consumed: exec_res.fuel_consumed,
                    execution_time_ns: duration_ns,
                }))
            }
            Err(err) => {
                let duration_ns = start.elapsed().as_nanos() as u64;
                make_error_result(&err.to_string(), duration_ns)
            }
        }
    }));

    result.unwrap_or_else(|_| make_error_result("Panic caught during in-process execution.", 0))
}

/// Libera o resultado da execução e a string JSON associada.
///
/// # Safety
/// O ponteiro `res` deve ser nulo ou ter sido obtido por `asl_skill_execute`.
#[no_mangle]
pub unsafe extern "C" fn asl_exec_result_free(res: *mut asl_exec_result_t) {
    if !res.is_null() {
        let _ = catch_unwind(AssertUnwindSafe(|| {
            let b = Box::from_raw(res);
            if !b.json_output.is_null() {
                drop(CString::from_raw(b.json_output as *mut c_char));
            }
        }));
    }
}

fn make_error_result(msg: &str, duration_ns: u64) -> *mut asl_exec_result_t {
    let err_json = serde_json::json!({ "error": msg }).to_string();
    let sanitized = err_json.replace('\0', "\\u0000");
    let c_err = CString::new(sanitized).unwrap_or_else(|_| CString::new("{\"error\":\"unknown\"}").unwrap());
    Box::into_raw(Box::new(asl_exec_result_t {
        success: 0,
        json_output: c_err.into_raw(),
        fuel_consumed: 0,
        execution_time_ns: duration_ns,
    }))
}

#[cfg(test)]
mod tests {
    use super::*;

    const SAMPLE_SKILL: &str = r#"---
asl_version: "3.0"
name: "ffi-test-skill"
description: "Skill for C-ABI FFI validation"
interface:
  entrypoint: "calc"
---
# Semantic Section
Executes deterministic calculation in-process.

```asl
def calc(ctx, input):
    a = input.get("a", 0)
    b = input.get("b", 0)
    return {"sum": a + b, "product": a * b}
```
"#;

    #[test]
    fn test_ffi_lifecycle_and_execution() {
        unsafe {
            let rt = asl_runtime_init();
            assert!(!rt.is_null());

            let c_src = CString::new(SAMPLE_SKILL).unwrap();
            let skill = asl_skill_load(rt, c_src.as_ptr());
            assert!(!skill.is_null());

            let c_ep = CString::new("calc").unwrap();
            let c_args = CString::new(r#"{"a": 20, "b": 22}"#).unwrap();

            let res = asl_skill_execute(rt, skill, c_ep.as_ptr(), c_args.as_ptr());
            assert!(!res.is_null());
            assert_eq!((*res).success, 1);
            assert!((*res).execution_time_ns > 0);

            let out_str = CStr::from_ptr((*res).json_output).to_str().unwrap();
            let val: Value = serde_json::from_str(out_str).unwrap();
            assert_eq!(val["sum"], 42);
            assert_eq!(val["product"], 440);

            asl_exec_result_free(res);
            asl_skill_free(skill);
            asl_runtime_free(rt);
        }
    }

    #[test]
    fn test_ffi_null_safety() {
        unsafe {
            let res = asl_skill_execute(std::ptr::null_mut(), std::ptr::null_mut(), std::ptr::null(), std::ptr::null());
            assert!(!res.is_null());
            assert_eq!((*res).success, 0);
            asl_exec_result_free(res);

            asl_runtime_free(std::ptr::null_mut());
            asl_skill_free(std::ptr::null_mut());
            asl_exec_result_free(std::ptr::null_mut());
        }
    }
}
