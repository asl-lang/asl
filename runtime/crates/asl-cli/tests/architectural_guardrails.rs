use asl_core_traits::{EnginePort, ParserPort};
use asl_parser::CommonMarkYamlParser;
use asl_security::ConfinedSecurityContext;
use asl_vm_starlark::StarlarkEngine;
use std::fs;
use std::path::{Path, PathBuf};

fn find_runtime_root() -> PathBuf {
    let mut curr = std::env::current_dir().unwrap_or_else(|_| PathBuf::from("."));
    for _ in 0..5 {
        if curr.join("crates").exists() && curr.join("Cargo.toml").exists() {
            return curr;
        }
        if curr.join("runtime/crates").exists() {
            return curr.join("runtime");
        }
        if !curr.pop() {
            break;
        }
    }
    PathBuf::from(".")
}

fn find_examples_dir() -> PathBuf {
    let rt = find_runtime_root();
    if rt.join("../examples").exists() {
        rt.join("../examples")
    } else if rt.join("examples").exists() {
        rt.join("examples")
    } else {
        PathBuf::from("../examples")
    }
}

#[test]
fn test_guardrail_cognitive_file_size_limit() {
    // Axioma 7: Nenhum arquivo .rs deve exceder 450 linhas (target: < 400)
    let runtime_root = find_runtime_root();
    let crates_dir = runtime_root.join("crates");
    assert!(
        crates_dir.exists(),
        "Diretório de crates deve ser encontrado: {:?}",
        crates_dir
    );

    let mut oversized_files = Vec::new();

    fn check_dir(dir: &Path, oversized: &mut Vec<(PathBuf, usize)>) {
        if let Ok(entries) = fs::read_dir(dir) {
            for entry in entries.flatten() {
                let path = entry.path();
                if path.is_dir() {
                    check_dir(&path, oversized);
                } else if path.extension().and_then(|s| s.to_str()) == Some("rs") {
                    if let Ok(content) = fs::read_to_string(&path) {
                        let lines = content.lines().count();
                        if lines > 450 {
                            oversized.push((path, lines));
                        }
                    }
                }
            }
        }
    }

    check_dir(&crates_dir, &mut oversized_files);

    assert!(
        oversized_files.is_empty(),
        "Violação do Axioma 7 (Limite Cognitivo). Arquivos com mais de 450 linhas: {:?}",
        oversized_files
    );
}

#[test]
fn test_guardrail_hexagonal_adapter_isolation() {
    // Axioma 4: Nenhum adaptador pode depender de outro adaptador
    let runtime_root = find_runtime_root();
    let crates_dir = runtime_root.join("crates");
    assert!(crates_dir.exists());

    let adapter_crates = [
        "asl-parser",
        "asl-security",
        "asl-vm-starlark",
        "asl-protocol-mcp",
    ];

    for adapter in &adapter_crates {
        let cargo_toml_path = crates_dir.join(adapter).join("Cargo.toml");
        assert!(
            cargo_toml_path.exists(),
            "Cargo.toml deve existir para {:?}",
            cargo_toml_path
        );
        let content = fs::read_to_string(&cargo_toml_path).unwrap();

        for other in &adapter_crates {
            if adapter != other {
                assert!(
                    !content.contains(other),
                    "Violação do Axioma 4 (Isolamento Hexagonal): O adaptador '{}' depende ilegalmente do adaptador '{}'",
                    adapter, other
                );
            }
        }
    }
}

#[test]
fn test_guardrail_pure_domain_spec_integrity() {
    // Axioma 2 e 4: asl-spec deve ser 100% puro (sem I/O, tokio ou starlark)
    let runtime_root = find_runtime_root();
    let spec_toml = runtime_root.join("crates/asl-spec/Cargo.toml");
    assert!(spec_toml.exists());

    let content = fs::read_to_string(&spec_toml).unwrap();
    let forbidden = ["tokio", "starlark", "clap", "asl-core-traits", "asl-parser"];

    for f in forbidden {
        assert!(
            !content.contains(f),
            "Violação de Pureza de Domínio: asl-spec depende ilegalmente de '{}'",
            f
        );
    }
}

#[test]
fn test_guardrail_canonical_skill_digests() {
    // Axioma 6 e Integridade Criptográfica: Todo arquivo .skill em examples deve ter digest válido
    let examples_dir = find_examples_dir();
    assert!(
        examples_dir.exists(),
        "Diretório de exemplos deve ser encontrado: {:?}",
        examples_dir
    );

    let parser = CommonMarkYamlParser::new();
    let entries = fs::read_dir(&examples_dir).expect("Deve ler examples/");

    let mut audited_count = 0;
    for entry in entries.flatten() {
        let path = entry.path();
        if path.extension().and_then(|s| s.to_str()) == Some("skill") {
            let content = fs::read_to_string(&path).unwrap();
            let doc = parser
                .parse(&content)
                .unwrap_or_else(|e| panic!("Falha ao analisar skill {:?}: {}", path, e));

            let declared_digest = doc
                .manifest
                .digest
                .as_deref()
                .unwrap_or_else(|| panic!("Skill {:?} não possui digest declarado", path));

            assert_eq!(
                declared_digest, doc.digest,
                "Violação de Integridade Criptográfica: Digest no arquivo {:?} ({}) diverge do hash canônico calculado ({})",
                path, declared_digest, doc.digest
            );
            audited_count += 1;
        }
    }

    assert!(
        audited_count > 0,
        "Pelo menos um arquivo .skill deve ser auditado em {:?}",
        examples_dir
    );
}

#[test]
fn test_canonical_skill_end_to_end_execution() {
    let examples_dir = find_examples_dir();
    let skill_path = examples_dir.join("git-conventional-commit.skill");
    assert!(skill_path.exists(), "Skill path must exist: {:?}", skill_path);

    let content = fs::read_to_string(&skill_path).unwrap();
    let parser = CommonMarkYamlParser::new();
    let doc = parser.parse(&content).expect("Parse must succeed");

    let engine = StarlarkEngine::new();
    let security = ConfinedSecurityContext::from_capabilities(
        &doc.manifest.capabilities,
        doc.manifest.limits.max_fuel_opcodes,
    );

    // Case 1: Valid explicit commit type
    let input_valid = serde_json::json!({
        "intent": "feat: add user authentication",
        "diff_stat": "2 files changed"
    });
    let res = engine
        .execute(
            &doc.deterministic_code,
            &doc.manifest.interface.entrypoint,
            &input_valid,
            &security,
            &doc.manifest.limits,
        )
        .expect("Execution should succeed");

    assert!(res.success);
    assert_eq!(res.output["is_valid"], true);
    assert_eq!(res.output["commit_type"], "feat");
    assert_eq!(res.output["formatted_message"], "feat: add user authentication");

    // Case 2: Inferred fix commit type
    let input_inferred = serde_json::json!({
        "intent": "corrigir bug no parser",
        "diff_stat": "1 file changed"
    });
    let res2 = engine
        .execute(
            &doc.deterministic_code,
            &doc.manifest.interface.entrypoint,
            &input_inferred,
            &security,
            &doc.manifest.limits,
        )
        .unwrap();
    assert_eq!(res2.output["commit_type"], "fix");
    assert_eq!(res2.output["formatted_message"], "fix: corrigir bug no parser");

    // Case 3: Empty intent should yield invalid
    let input_empty = serde_json::json!({
        "intent": "",
        "diff_stat": ""
    });
    let res3 = engine
        .execute(
            &doc.deterministic_code,
            &doc.manifest.interface.entrypoint,
            &input_empty,
            &security,
            &doc.manifest.limits,
        )
        .unwrap();
    assert_eq!(res3.output["is_valid"], false);
    assert_eq!(res3.output["commit_type"], "unknown");
}

#[test]
fn test_guardrail_adr_and_plan_structure() {
    let rt = find_runtime_root();
    let docs_dir = if rt.join("../docs").exists() {
        rt.join("../docs")
    } else {
        PathBuf::from("../docs")
    };
    assert!(docs_dir.exists(), "Diretório docs deve existir: {:?}", docs_dir);

    // 1. Validar docs/adrs/
    let adrs_dir = docs_dir.join("adrs");
    if adrs_dir.exists() {
        for entry in fs::read_dir(&adrs_dir).unwrap().flatten() {
            let file_name = entry.file_name().to_string_lossy().to_string();
            if file_name.ends_with(".md") && file_name != "README.md" {
                assert!(
                    file_name.chars().take(4).all(|c| c.is_ascii_digit()) && file_name.chars().nth(4) == Some('-'),
                    "ADR deve seguir o padrão NNNN-<nome>.md: {}", file_name
                );
                let content = fs::read_to_string(entry.path()).unwrap();
                assert!(content.contains("## 1. Contexto"), "ADR deve conter '## 1. Contexto': {}", file_name);
                assert!(content.contains("## 2. Proposta Detalhada"), "ADR deve conter '## 2. Proposta Detalhada': {}", file_name);
                assert!(content.contains("## 3. Alternativas"), "ADR deve conter '## 3. Alternativas': {}", file_name);
                assert!(content.contains("## 4. Consequências"), "ADR deve conter '## 4. Consequências': {}", file_name);
                assert!(content.contains("## 5. Conformidade com os 7 Axiomas"), "ADR deve conter '## 5. Conformidade': {}", file_name);
            }
        }
    }

    // 2. Validar docs/plans/
    let plans_dir = docs_dir.join("plans");
    if plans_dir.exists() {
        for entry in fs::read_dir(&plans_dir).unwrap().flatten() {
            let file_name = entry.file_name().to_string_lossy().to_string();
            if file_name.ends_with(".md") && file_name != "README.md" {
                assert!(
                    file_name.chars().take(4).all(|c| c.is_ascii_digit()) && file_name.chars().nth(4) == Some('-'),
                    "Plano deve seguir o padrão NNNN-<nome>.md: {}", file_name
                );
                let content = fs::read_to_string(entry.path()).unwrap();
                assert!(content.contains("## Fase 1:"), "Plano deve conter '## Fase 1:': {}", file_name);
                assert!(content.contains("Commit & Push") || content.contains("git push origin main"), "Plano deve conter instrução de Commit/Push: {}", file_name);
            }
        }
    }
}

#[test]
fn test_starlark_capability_context_end_to_end() {
    let runtime_root = find_runtime_root();
    let parser = CommonMarkYamlParser::new();
    let engine = StarlarkEngine::new();

    let skill_source = r#"---
asl_version: "3.0"
name: "capability-audit"
description: "Audita arquivos confinados e calcula hashes"
interface:
  entrypoint: "audit"
capabilities:
  fs:
    confined_read_roots: ["."]
limits:
  max_fuel_opcodes: 1000000
---
# Instruções Semânticas
Execute a auditoria determinística com capabilities.

```asl:deterministic
def audit(ctx, input):
    cargo_content = ctx.fs.read("Cargo.toml")
    crypto_hash = ctx.crypto.sha256("agent-skill-language")
    remaining = ctx.fuel.remaining()
    
    return {
        "has_cargo_toml": cargo_content != None and len(cargo_content) > 0,
        "crypto_hash": crypto_hash,
        "fuel_ok": remaining > 0,
    }
```
"#;

    let doc = parser.parse(skill_source).expect("Parse deve suceder");
    let mut caps = doc.manifest.capabilities.clone();
    caps.fs.confined_read_roots = vec![runtime_root.to_string_lossy().to_string()];

    let security = ConfinedSecurityContext::from_capabilities(&caps, doc.manifest.limits.max_fuel_opcodes);

    let res = engine
        .execute(
            &doc.deterministic_code,
            &doc.manifest.interface.entrypoint,
            &serde_json::json!({}),
            &security,
            &doc.manifest.limits,
        )
        .expect("Execução com stdlib de capabilities deve suceder");

    assert!(res.success);
    assert_eq!(res.output["has_cargo_toml"], true);
    assert_eq!(res.output["crypto_hash"], "3847599a5741627e350f9af8c561bf6497535001edb3a5f4cfc8281cd9c58c60");
    assert_eq!(res.output["fuel_ok"], true);
}

#[test]
fn test_guardrail_shadow_markdown_projection() {
    let examples_dir = find_examples_dir();
    let parser = CommonMarkYamlParser::new();
    let entries = fs::read_dir(&examples_dir).expect("Deve ler examples/");

    for entry in entries.flatten() {
        let path = entry.path();
        if path.extension().and_then(|s| s.to_str()) == Some("skill") {
            let content = fs::read_to_string(&path).unwrap();
            let doc = parser.parse(&content).unwrap();

            let md_path = path.with_extension("md");
            assert!(
                md_path.exists(),
                "Projeção sombra {:?} deve existir para a skill canônica {:?}",
                md_path, path
            );

            let md_content = fs::read_to_string(&md_path).unwrap();
            assert!(
                md_content.contains("<!-- ⚡ ASL AUTO-GENERATED SHADOW PROJECTION"),
                "Sombra {:?} deve conter watermark do ASL", md_path
            );
            assert!(
                md_content.contains(&format!("DIGEST: {}", doc.digest)),
                "Sombra {:?} deve conter o digest exato da skill canônica", md_path
            );
        }
    }
}

#[test]
fn test_guardrail_declarative_rules_in_memory_transpilation() {
    let examples_dir = find_examples_dir();
    let rules_skill = examples_dir.join("conventional-commit-rules.skill");
    assert!(rules_skill.exists(), "conventional-commit-rules.skill deve existir");

    let parser = CommonMarkYamlParser::new();
    let content = fs::read_to_string(&rules_skill).unwrap();
    let doc = parser.parse(&content).expect("Parsing de rules deve suceder");

    assert!(doc.rules_code.is_some(), "Documento deve reter rules_code original");
    assert!(doc.deterministic_code.contains("def validate_and_format(ctx, input):"));
    assert!(doc.deterministic_code.contains("_asl_get(input, [\"intent\"], \"\")"));

    let engine = StarlarkEngine::new();
    let security = ConfinedSecurityContext::from_capabilities(
        &doc.manifest.capabilities,
        doc.manifest.limits.max_fuel_opcodes,
    );

    let input = serde_json::json!({
        "intent": "corrigir memory leak no buffer",
        "diff_stat": "1 file changed"
    });
    let res = engine.execute(
        &doc.deterministic_code,
        &doc.manifest.interface.entrypoint,
        &input,
        &security,
        &doc.manifest.limits,
    ).expect("Execução de regras em memória deve suceder");

    assert!(res.success);
    assert_eq!(res.output["is_valid"], true);
    assert_eq!(res.output["commit_type"], "fix");
}
