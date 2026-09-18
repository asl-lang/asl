use super::*;

#[test]
fn test_parser_valid_skill() {
    let raw = r#"---
asl_version: "3.0"
name: "test-skill"
description: "A test skill"
interface:
  entrypoint: "run"
---
# Semantic Instructions
Execute deterministic function.

```asl:deterministic
def run(ctx, input):
    return {"status": "ok"}
```
"#;

    let parser = CommonMarkYamlParser::new();
    let doc = parser.parse(raw).expect("Parsing should succeed");
    assert_eq!(doc.manifest.name, "test-skill");
    assert_eq!(doc.manifest.asl_version, "3.0");
    assert!(doc.deterministic_code.contains("def run(ctx, input)"));
    assert!(doc.semantic_section.contains("Semantic Instructions"));
    assert!(doc.digest.starts_with("asl:sha256:"));
}

#[test]
fn test_greenfield_zero_byte_empty_file_ingestion() {
    let parser = CommonMarkYamlParser::new();
    let doc = parser.parse("").expect("0-byte empty file must parse as draft scaffold");
    assert_eq!(doc.manifest.asl_version, "3.0");
    assert_eq!(doc.manifest.name, "draft-skill");
    assert_eq!(doc.manifest.interface.entrypoint, "run");
    assert!(doc.deterministic_code.contains("def run(ctx, input):"));
    assert!(doc.digest.starts_with("asl:sha256:"));

    let doc_spaces = parser.parse("   \n\t\n  ").expect("Whitespace-only file must parse as draft scaffold");
    assert_eq!(doc_spaces.manifest.name, "draft-skill");
}

#[test]
fn test_legacy_skill_missing_asl_version_and_interface() {
    let raw = r#"---
name: "legacy-agent-skill"
description: "From Claude or Cursor without asl_version"
---
# Prompt
Legacy prompt instructions.
"#;

    let parser = CommonMarkYamlParser::new();
    let doc = parser.parse(raw).expect("Legacy skill missing asl_version must use default 3.0");
    assert_eq!(doc.manifest.asl_version, "3.0");
    assert_eq!(doc.manifest.name, "legacy-agent-skill");
    assert_eq!(doc.manifest.interface.entrypoint, "run");
}

#[test]
fn test_digest_invariance_with_signature_and_digest_fields() {
    let raw1 = r#"---
asl_version: "3.0"
name: "signed-skill"
interface:
  entrypoint: "run"
---
# Semantic

```asl
def run(ctx, input):
    return {}
```
"#;

    let raw2 = r#"---
asl_version: "3.0"
name: "signed-skill"
interface:
  entrypoint: "run"
digest: "asl:sha256:dummy"
signature: "asl:ed25519:dummy_sig"
signer_pubkey: "asl:ed25519:pub:dummy_pub"
---
# Semantic

```asl
def run(ctx, input):
    return {}
```
"#;

    let parser = CommonMarkYamlParser::new();
    let doc1 = parser.parse(raw1).unwrap();
    let doc2 = parser.parse(raw2).unwrap();
    assert_eq!(doc1.digest, doc2.digest);
}

#[test]
fn test_pure_semantic_skill_without_code_block() {
    let raw = r#"---
asl_version: "3.0"
name: "pure-prompt"
interface:
  entrypoint: "run"
---
# Pure Prompt Instructions
Technical documentation writer instructions.
"#;
    let parser = CommonMarkYamlParser::new();
    let doc = parser
        .parse(raw)
        .expect("Pure semantic skill must be valid");
    assert_eq!(doc.manifest.name, "pure-prompt");
    assert!(doc.deterministic_code.contains("def run(ctx, input)"));
    assert!(doc.semantic_section.contains("Pure Prompt Instructions"));
}

#[test]
fn test_rules_skill_parsing_and_in_memory_transpilation() {
    let raw = r#"---
asl_version: "3.0"
name: "rules-skill"
interface:
  entrypoint: "validate"
---
# Semantic Instructions
Declarative rules execution.

```asl:rules
guard:
  input.text is not empty else reject("Empty text")

match input.text:
  when starts_with "hello":
    accept(status="greeting")
  otherwise:
    accept(status="normal")
```
"#;
    let parser = CommonMarkYamlParser::new();
    let doc = parser
        .parse(raw)
        .expect("Skill with rules must parse and transpile in-memory");
    assert_eq!(doc.manifest.name, "rules-skill");
    assert!(doc.rules_code.is_some());
    assert!(doc.deterministic_code.contains("def validate(ctx, input):"));
    assert!(doc
        .deterministic_code
        .contains("_asl_get(input, [\"text\"], \"\")"));
}

#[test]
fn test_pure_semantic_skill_with_custom_entrypoint() {
    let raw = r#"---
asl_version: "3.0"
name: "custom-ep-skill"
interface:
  entrypoint: "process_query"
---
# Prompt
Semantic prompt only.
"#;
    let parser = CommonMarkYamlParser::new();
    let doc = parser
        .parse(raw)
        .expect("Skill without code and custom ep must be valid");
    assert_eq!(doc.manifest.interface.entrypoint, "process_query");
    assert!(doc
        .deterministic_code
        .contains("def process_query(ctx, input):"));
}

#[test]
fn test_digest_includes_markdown_lines_starting_with_digest_or_signature() {
    let base = r#"---
asl_version: "3.0"
name: "markdown-test"
interface:
  entrypoint: "run"
---
# Semantic Section
Normal line.
"#;

    let tampered = r#"---
asl_version: "3.0"
name: "markdown-test"
interface:
  entrypoint: "run"
---
# Semantic Section
Normal line.
digest: malicious alteration
signature: fake signature
"#;
    let parser = CommonMarkYamlParser::new();
    let doc_base = parser.parse(base).unwrap();
    let doc_tampered = parser.parse(tampered).unwrap();
    assert_ne!(doc_base.digest, doc_tampered.digest, "Lines in markdown body MUST alter digest even if starting with digest: or signature:");
}

#[test]
fn test_tolerant_raw_markdown_ingestion_without_frontmatter() {
    let raw_md = r#"# My Awesome Custom Skill
This is an imported markdown skill without YAML frontmatter.

Execute this command carefully.
"#;
    let parser = CommonMarkYamlParser::new();
    let doc = parser
        .parse(raw_md)
        .expect("Raw markdown without frontmatter must parse tolerantly per ADR-0015");
    assert_eq!(doc.manifest.asl_version, "3.0");
    assert_eq!(doc.manifest.name, "my-awesome-custom-skill");
    assert_eq!(
        doc.manifest.description,
        "This is an imported markdown skill without YAML frontmatter."
    );
    assert_eq!(doc.manifest.interface.entrypoint, "run");
    assert!(doc.deterministic_code.contains("def run(ctx, input):"));
    assert!(doc.digest.starts_with("asl:sha256:"));
}

#[test]
fn test_utf8_bom_and_crlf_cross_platform_handling() {
    let parser = CommonMarkYamlParser::new();

    // 1. Skill with UTF-8 BOM (\u{feff}) and Windows CRLF (\r\n)
    let bom_crlf_content = "\u{feff}---\r\nasl_version: \"3.0\"\r\nname: \"windows-bom-skill\"\r\ninterface:\r\n  entrypoint: \"run\"\r\n---\r\n# Windows Instructions\r\nHello from Windows.\r\n";
    let doc = parser.parse(bom_crlf_content).expect("UTF-8 BOM and CRLF must parse flawlessly");
    assert_eq!(doc.manifest.name, "windows-bom-skill");
    assert_eq!(doc.manifest.asl_version, "3.0");
    assert!(doc.semantic_section.contains("Hello from Windows."));

    // 2. 0-byte file with BOM
    let bom_empty = "\u{feff}";
    let doc_empty = parser.parse(bom_empty).expect("BOM-only file must parse as draft scaffold");
    assert_eq!(doc_empty.manifest.name, "draft-skill");
}

#[test]
fn test_unclosed_frontmatter_transitioning_to_markdown() {
    let content = r#"---
asl_version: "3.0"
name: "unclosed-fm-skill"
description: "Skill with unclosed frontmatter"
interface:
  entrypoint: "run"

# Skill Instructions
This skill tests unclosed frontmatter gracefully transitioning to markdown.

```asl
def run(ctx, input):
  return {"status": "ok"}
```
"#;
    let parser = CommonMarkYamlParser::new();
    let doc = parser.parse(content).expect("Unclosed frontmatter before header must parse tolerantly");
    assert_eq!(doc.manifest.name, "unclosed-fm-skill");
    assert_eq!(doc.manifest.description, "Skill with unclosed frontmatter");
    assert!(doc.semantic_section.contains("# Skill Instructions"));
    assert!(doc.deterministic_code.contains("def run(ctx, input):"));
}



