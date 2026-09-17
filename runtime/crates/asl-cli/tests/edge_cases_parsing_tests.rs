use asl_core_traits::ParserPort;
use asl_parser::CommonMarkYamlParser;
use asl_spec::{is_asl_file, is_shadow_eligible, AslError, ASL_EXTENSIONS};
use std::path::Path;

fn build_document(asl_ver: &str, name: &str, entrypoint: &str, body: &str) -> String {
    format!(
        "---\nasl_version: \"{}\"\nname: \"{}\"\ninterface:\n  entrypoint: \"{}\"\n---\n{}",
        asl_ver, name, entrypoint, body
    )
}

#[test]
fn test_empty_and_whitespace_rejected_for_all_formats() {
    let parser = CommonMarkYamlParser::new();
    let empty_inputs = ["", "   ", "\n\n\t  \n  ", "# Apenas comentário solto\n"];

    for ext in ASL_EXTENSIONS {
        for input in empty_inputs {
            let res = parser.parse(input);
            assert!(
                res.is_err(),
                "Extensão .{} deveria rejeitar conteúdo vazio ou sem frontmatter",
                ext
            );
            match res.unwrap_err() {
                AslError::InvalidFrontmatter(_) => {}
                other => panic!("Esperado InvalidFrontmatter, obtido: {:?}", other),
            }
        }
    }
}

#[test]
fn test_unclosed_and_missing_frontmatter_delimiters() {
    let parser = CommonMarkYamlParser::new();

    // 1. Apenas delimitador inicial sem fechamento
    let unclosed = "---\nasl_version: \"3.0\"\nname: \"teste\"\ninterface:\n  entrypoint: \"run\"\n";
    let res = parser.parse(unclosed);
    assert!(res.is_err());
    assert!(matches!(res.unwrap_err(), AslError::InvalidFrontmatter(_)));

    // 2. Sem delimitador inicial
    let no_open = "asl_version: \"3.0\"\nname: \"teste\"\n---\n";
    let res = parser.parse(no_open);
    assert!(res.is_err());
    assert!(matches!(res.unwrap_err(), AslError::InvalidFrontmatter(_)));
}

#[test]
fn test_html_comments_preceding_frontmatter_supported() {
    let parser = CommonMarkYamlParser::new();
    let content = r#"<!--
  Metadados ou licença em comentário HTML antes do YAML
-->
---
asl_version: "3.0"
name: "html-comment-skill"
interface:
  entrypoint: "run"
---
# Seção Semântica
```asl:deterministic
def run(ctx, input):
    return {"ok": True}
```
"#;
    let doc = parser.parse(content).expect("Deve tolerar comentários HTML no topo");
    assert_eq!(doc.manifest.name, "html-comment-skill");
    assert_eq!(doc.manifest.interface.entrypoint, "run");
}

#[test]
fn test_malformed_yaml_syntax_rejected() {
    let parser = CommonMarkYamlParser::new();
    let bad_yaml = "---\nname: [unclosed array\nasl_version: \"3.0\"\n---\n# Corpo";
    let res = parser.parse(bad_yaml);
    assert!(res.is_err());
    assert!(matches!(res.unwrap_err(), AslError::InvalidFrontmatter(_)));
}

#[test]
fn test_missing_mandatory_manifest_fields() {
    let parser = CommonMarkYamlParser::new();

    // Nome vazio
    let doc_no_name = build_document("3.0", "", "run", "```asl:deterministic\ndef run(c, i): pass\n```");
    assert!(matches!(parser.parse(&doc_no_name).unwrap_err(), AslError::InvalidFrontmatter(_)));

    // Versão ASL incompatível (ex: 2.0 ou 1.0)
    let doc_bad_ver = build_document("2.0", "valid-name", "run", "");
    assert!(matches!(parser.parse(&doc_bad_ver).unwrap_err(), AslError::InvalidFrontmatter(_)));

    // Entrypoint vazio
    let doc_no_ep = build_document("3.0", "valid-name", "", "");
    assert!(matches!(parser.parse(&doc_no_ep).unwrap_err(), AslError::InvalidFrontmatter(_)));

    // Entrypoint com caracteres ilegais
    let doc_invalid_ep = build_document("3.0", "valid-name", "1invalid_ident", "");
    assert!(matches!(parser.parse(&doc_invalid_ep).unwrap_err(), AslError::InvalidFrontmatter(_)));

    let doc_spaces_ep = build_document("3.0", "valid-name", "invalid ident", "");
    assert!(matches!(parser.parse(&doc_spaces_ep).unwrap_err(), AslError::InvalidFrontmatter(_)));
}

#[test]
fn test_unicode_and_emojis_in_manifest_and_content() {
    let parser = CommonMarkYamlParser::new();
    let content = r#"---
asl_version: "3.0"
name: "validador-segurança-🚀"
description: "Verificação de acentuação (ç, ã, é) e emojis 🛡️ 日本語"
interface:
  entrypoint: "executar_verificacao"
---
# Seção Semântica com Emojis 💎
Critério de ativação: quando usuário solicitar análise de texto em português ou japonês (日本語).

```asl:deterministic
def executar_verificacao(ctx, input):
    return {"mensagem": "Olá mundo! 🚀", "status": "sucesso"}
```
"#;
    let doc = parser.parse(content).expect("Deve suportar Unicode e Emojis");
    assert_eq!(doc.manifest.name, "validador-segurança-🚀");
    assert!(doc.manifest.description.contains("🛡️ 日本語"));
    assert!(doc.semantic_section.contains("💎"));
    assert!(doc.deterministic_code.contains("Olá mundo! 🚀"));
}

#[test]
fn test_multiple_code_blocks_and_foreign_languages_ignored() {
    let parser = CommonMarkYamlParser::new();
    let content = r#"---
asl_version: "3.0"
name: "multi-code-blocks"
interface:
  entrypoint: "run"
---
# Exemplos com outras linguagens
Aqui está um script em Python que NÃO deve ser executado:
```python
print("Eu sou python e devo ser ignorado pelo parser ASL")
```

E aqui um script Bash:
```bash
echo "Eu sou bash"
```

Apenas o bloco asl:deterministic deve ser considerado:
```asl:deterministic
def run(ctx, input):
    return {"real": True}
```
"#;
    let doc = parser.parse(content).expect("Deve ignorar linguagens estranhas");
    assert!(!doc.deterministic_code.contains("print(\"Eu sou python"));
    assert!(!doc.deterministic_code.contains("echo \"Eu sou bash\""));
    assert!(doc.deterministic_code.contains("def run(ctx, input):"));
}

#[test]
fn test_document_with_no_code_block_provides_default_fallback() {
    let parser = CommonMarkYamlParser::new();
    let content = r#"---
asl_version: "3.0"
name: "no-code-block"
interface:
  entrypoint: "run"
---
# Documento Declarativo Puro
Apenas instruções semânticas para LLMs, sem código executável.
"#;
    let doc = parser.parse(content).expect("Deve aceitar documento sem bloco de código");
    assert!(doc.deterministic_code.contains("def run(ctx, input):"));
    assert!(doc.deterministic_code.contains("return input"));
}

#[test]
fn test_filenames_with_multiple_dots_and_extensions() {
    // Casos complexos de nomes de arquivos
    let filenames = [
        ("my.complex.service.v1.0.tool", true, false),
        ("app.config.rules.skill", true, true),
        ("agent.core.pipeline.asl", true, false),
        ("TEST.CANONICAL.TOOL", true, false),
        ("RELEASE.SKILL", true, true),
        ("MODULE.ASL", true, false),
    ];

    for (fname, expected_asl, expected_shadow) in filenames {
        let p = Path::new(fname);
        assert_eq!(
            is_asl_file(p),
            expected_asl,
            "Falha em is_asl_file para {}",
            fname
        );
        assert_eq!(
            is_shadow_eligible(p),
            expected_shadow,
            "Falha em is_shadow_eligible para {}",
            fname
        );
    }
}

#[test]
fn test_unsupported_extensions_strictly_rejected() {
    let rejected = [
        "skill.agent",
        "prompt.prompt",
        "guardrail.guard",
        "persona.persona",
        "chain.chain",
        "rules.rules",
        "script.py",
        "script.sh",
        "doc.md",
        "data.json",
        "notes.txt",
        "no_extension",
    ];

    for fname in rejected {
        let p = Path::new(fname);
        assert!(
            !is_asl_file(p),
            "Extensão de {} NÃO deve ser aceita como arquivo ASL",
            fname
        );
        assert!(
            !is_shadow_eligible(p),
            "Extensão de {} NÃO deve ser elegível para sombra",
            fname
        );
    }
}
