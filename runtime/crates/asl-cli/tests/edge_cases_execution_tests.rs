use asl_core_traits::{EnginePort, ParserPort};
use asl_parser::CommonMarkYamlParser;
use asl_security::ConfinedSecurityContext;
use asl_spec::{AslError, Limits, ASL_EXTENSIONS};
use asl_vm_starlark::StarlarkEngine;
use serde_json::json;

fn make_asl_source(name: &str, entrypoint: &str, code_block: &str) -> String {
    format!(
        r#"---
asl_version: "3.0"
name: "{}"
interface:
  entrypoint: "{}"
  input_schema:
    type: "object"
    properties:
      val: {{ type: "integer" }}
---
# Documento de Teste
{}"#,
        name, entrypoint, code_block
    )
}

#[test]
fn test_missing_entrypoint_in_code_fails_safely_across_triad() {
    let parser = CommonMarkYamlParser::new();
    let engine = StarlarkEngine::new();
    let limits = Limits::default();
    let ctx =
        ConfinedSecurityContext::from_capabilities(&Default::default(), limits.max_fuel_opcodes);

    for ext in ASL_EXTENSIONS {
        let raw = make_asl_source(
            &format!("ep-missing-{}", ext),
            "expected_func",
            "```asl:deterministic\ndef other_func(ctx, input):\n    return {\"status\": \"ok\"}\n```",
        );
        let doc = parser.parse(&raw).expect("Parsing deve suceder");
        let res = engine.execute(
            &doc.deterministic_code,
            &doc.manifest.interface.entrypoint,
            &json!({"val": 1}),
            &ctx,
            &limits,
        );

        assert!(
            res.is_err(),
            "Execução com entrypoint ausente deveria falhar para .{}",
            ext
        );
        match res.unwrap_err() {
            AslError::StarlarkError(msg) => {
                assert!(
                    msg.contains("expected_func")
                        || msg.contains("Variable referenced before assignment"),
                    "Mensagem deve referenciar o entrypoint ausente: {}",
                    msg
                );
            }
            AslError::EntrypointNotFound(ep) => {
                assert_eq!(ep, "expected_func");
            }
            other => panic!(
                "Expected Starlark error or EntrypointNotFound, got: {:?}",
                other
            ),
        }
    }
}

#[test]
fn test_runtime_division_by_zero_and_exceptions_across_triad() {
    let parser = CommonMarkYamlParser::new();
    let engine = StarlarkEngine::new();
    let limits = Limits::default();
    let ctx =
        ConfinedSecurityContext::from_capabilities(&Default::default(), limits.max_fuel_opcodes);

    for ext in ASL_EXTENSIONS {
        let raw = make_asl_source(
            &format!("div-zero-{}", ext),
            "divide",
            "```asl:deterministic\ndef divide(ctx, input):\n    x = 100 // 0\n    return {\"res\": x}\n```",
        );
        let doc = parser.parse(&raw).expect("Parsing deve suceder");
        let res = engine.execute(
            &doc.deterministic_code,
            &doc.manifest.interface.entrypoint,
            &json!({}),
            &ctx,
            &limits,
        );

        assert!(res.is_err(), "Divisão por zero deve falhar em .{}", ext);
        match res.unwrap_err() {
            AslError::StarlarkError(msg) => {
                assert!(
                    msg.to_lowercase().contains("zero") || msg.to_lowercase().contains("division"),
                    "Mensagem deve reportar divisão por zero: {}",
                    msg
                );
            }
            other => panic!("Expected StarlarkError, got: {:?}", other),
        }
    }
}

#[test]
fn test_rules_transpiler_syntax_error_rejected() {
    let parser = CommonMarkYamlParser::new();
    let bad_rules = r#"---
asl_version: "3.0"
name: "bad-rules"
interface:
  entrypoint: "evaluate"
---
# Regras Inválidas
```asl:rules
RULE 123_invalid_name:
  WHEN ???:
    RETURN reject("Sintaxe corrompida")
```
"#;
    let res = parser.parse(bad_rules);
    assert!(
        res.is_err(),
        "Regras com sintaxe inválida devem ser rejeitadas"
    );
    assert!(matches!(res.unwrap_err(), AslError::RulesTranspileError(_)));
}

#[test]
fn test_rules_exhaustiveness_enforcement_in_match() {
    let parser = CommonMarkYamlParser::new();
    // Bloco match sem 'otherwise:' obrigatório deve falhar no transpilador
    let incomplete_match = r#"---
asl_version: "3.0"
name: "incomplete-match"
interface:
  entrypoint: "check_intent"
---
# Match Incompleto
```asl:rules
match input.action:
  when starts_with "save":
    accept(saved=true)
  when starts_with "delete":
    accept(deleted=true)
```
"#;
    let res = parser.parse(incomplete_match);
    assert!(
        res.is_err(),
        "Match sem otherwise deve violar exaustividade"
    );
    match res.unwrap_err() {
        AslError::RulesTranspileError(msg) => {
            assert!(
                msg.contains("Exaustividade violada") || msg.contains("otherwise"),
                "Mensagem deve apontar violação de exaustividade: {}",
                msg
            );
        }
        other => panic!("Expected RulesTranspileError, got: {:?}", other),
    }
}

#[test]
fn test_rules_execution_across_triad() {
    let parser = CommonMarkYamlParser::new();
    let engine = StarlarkEngine::new();
    let limits = Limits::default();
    let ctx =
        ConfinedSecurityContext::from_capabilities(&Default::default(), limits.max_fuel_opcodes);

    let rules_source = r#"---
asl_version: "3.0"
name: "classifier"
interface:
  entrypoint: "classify"
---
# Classificador de Acesso
```asl:rules
guard:
  input.role is not empty else reject("Role obrigatório")

match input.role:
  when starts_with "admin":
    accept(allowed=true, level=10)
  when starts_with "guest":
    accept(allowed=false, level=0)
  otherwise:
    accept(allowed=false, level=1)
```
"#;

    let doc = parser
        .parse(rules_source)
        .expect("Regras válidas devem transpilar");
    assert!(doc.rules_code.is_some());

    // 1. Cenário admin
    let res_admin = engine
        .execute(
            &doc.deterministic_code,
            "classify",
            &json!({"role": "admin"}),
            &ctx,
            &limits,
        )
        .expect("Execução admin deve funcionar");
    assert!(res_admin.success);
    assert_eq!(res_admin.output["allowed"], true);
    assert_eq!(res_admin.output["level"], 10);

    // 2. Cenário guest
    let res_guest = engine
        .execute(
            &doc.deterministic_code,
            "classify",
            &json!({"role": "guest"}),
            &ctx,
            &limits,
        )
        .expect("Execução guest deve funcionar");
    assert!(res_guest.success);
    assert_eq!(res_guest.output["allowed"], false);
    assert_eq!(res_guest.output["level"], 0);

    // 3. Cenário fallback (otherwise)
    let res_other = engine
        .execute(
            &doc.deterministic_code,
            "classify",
            &json!({"role": "developer"}),
            &ctx,
            &limits,
        )
        .expect("Execução otherwise deve funcionar");
    assert!(res_other.success);
    assert_eq!(res_other.output["allowed"], false);
    assert_eq!(res_other.output["level"], 1);
}

#[test]
fn test_hybrid_rules_with_manual_helper_functions() {
    let parser = CommonMarkYamlParser::new();
    let engine = StarlarkEngine::new();
    let limits = Limits::default();
    let ctx =
        ConfinedSecurityContext::from_capabilities(&Default::default(), limits.max_fuel_opcodes);

    let hybrid_source = r#"---
asl_version: "3.0"
name: "hybrid-engine"
interface:
  entrypoint: "process_hybrid"
---
# Documento Híbrido com Regras e Código Manual

```asl:rules
guard:
  input.tag is not empty else reject("Tag obrigatória")

match input.tag:
  when starts_with "custom":
    accept(processed="custom: " + input.tag, is_custom=true)
  otherwise:
    accept(processed="standard", is_custom=false)
```

```asl:deterministic
def audit_helper(tag):
    return "audited:" + tag
```
"#;

    let doc = parser
        .parse(hybrid_source)
        .expect("Documento híbrido deve compilar");
    assert!(doc.deterministic_code.contains("audit_helper"));
    assert!(doc.rules_code.is_some());

    let res = engine
        .execute(
            &doc.deterministic_code,
            "process_hybrid",
            &json!({"tag": "custom"}),
            &ctx,
            &limits,
        )
        .expect("Execução híbrida deve suceder chamando o router gerado");

    assert!(res.success);
    assert_eq!(res.output["processed"], "custom: custom");
    assert_eq!(res.output["is_custom"], true);
}

#[test]
fn test_rules_matches_regex_execution() {
    let parser = CommonMarkYamlParser::new();
    let engine = StarlarkEngine::new();
    let limits = Limits::default();
    let ctx =
        ConfinedSecurityContext::from_capabilities(&Default::default(), limits.max_fuel_opcodes);

    let regex_source = r#"---
asl_version: "3.0"
name: "regex-validator"
interface:
  entrypoint: "validate_number"
---
# Validador de Números por Regex

```asl:rules
match input.code:
  when matches("^[0-9]{4}$"):
    accept(is_numeric=true, code=input.code)
  otherwise:
    reject("Código deve conter exatamente 4 dígitos numéricos")
```
"#;

    let doc = parser
        .parse(regex_source)
        .expect("Documento com regex deve compilar");

    // Caso 1: 4 dígitos numéricos -> Aceito
    let res_valid = engine
        .execute(
            &doc.deterministic_code,
            "validate_number",
            &json!({"code": "1234"}),
            &ctx,
            &limits,
        )
        .expect("Execução com entrada válida deve suceder");
    assert_eq!(res_valid.output["is_numeric"], true);
    assert_eq!(res_valid.output["code"], "1234");

    // Caso 2: Letras -> Rejeitado pelo regex (não aceito incondicionalmente!)
    let res_invalid = engine
        .execute(
            &doc.deterministic_code,
            "validate_number",
            &json!({"code": "abcd"}),
            &ctx,
            &limits,
        )
        .expect("Execução deve retornar dicionário de rejeição");
    assert_eq!(res_invalid.output["is_valid"], false);
    assert!(res_invalid.output["diagnostics"][0]
        .as_str()
        .unwrap()
        .contains("4 dígitos"));
}

#[test]
fn test_rules_multiple_sequential_matches() {
    let parser = CommonMarkYamlParser::new();
    let engine = StarlarkEngine::new();
    let limits = Limits::default();
    let ctx =
        ConfinedSecurityContext::from_capabilities(&Default::default(), limits.max_fuel_opcodes);

    let multi_match_source = r#"---
asl_version: "3.0"
name: "multi-matcher"
interface:
  entrypoint: "route"
---
# Múltiplos Matches Sequenciais

```asl:rules
match input.category:
  when starts_with "admin":
    accept(role="administrator")

match input.tag:
  when starts_with "vip":
    accept(tier="vip_member")
  otherwise:
    accept(tier="standard_user")
```
"#;

    let doc = parser
        .parse(multi_match_source)
        .expect("Múltiplos match sequenciais devem compilar");

    // Caso 1: Primeiro match casa
    let res1 = engine
        .execute(
            &doc.deterministic_code,
            "route",
            &json!({"category": "admin_ops", "tag": "none"}),
            &ctx,
            &limits,
        )
        .expect("Execução deve suceder");
    assert_eq!(res1.output["role"], "administrator");

    // Caso 2: Primeiro match não casa, cai no segundo match que casa
    let res2 = engine
        .execute(
            &doc.deterministic_code,
            "route",
            &json!({"category": "user", "tag": "vip_guest"}),
            &ctx,
            &limits,
        )
        .expect("Execução deve suceder");
    assert_eq!(res2.output["tier"], "vip_member");

    // Caso 3: Primeiro e segundo matches não casam, cai no otherwise do segundo match
    let res3 = engine
        .execute(
            &doc.deterministic_code,
            "route",
            &json!({"category": "user", "tag": "guest"}),
            &ctx,
            &limits,
        )
        .expect("Execução deve suceder");
    assert_eq!(res3.output["tier"], "standard_user");
}
