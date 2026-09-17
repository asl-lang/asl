pub mod grammar;
pub mod prefix_analyzer;
pub mod rules;
pub mod shadow;

pub use grammar::GbnfGrammarCompiler;
pub use prefix_analyzer::*;
pub use rules::{parse_rules, Action, GuardClause, MatchSection, PathExpr, PatternCondition, RulesBlock, WhenClause};
pub use shadow::*;

use asl_core_traits::ParserPort;
use asl_spec::{AslError, Result, SkillDocument, SkillManifest};
use pulldown_cmark::{CodeBlockKind, Event, Parser, Tag, TagEnd};
use sha2::{Digest, Sha256};

pub struct CommonMarkYamlParser;

impl CommonMarkYamlParser {
    pub fn new() -> Self {
        Self
    }
}

impl Default for CommonMarkYamlParser {
    fn default() -> Self {
        Self::new()
    }
}

impl ParserPort for CommonMarkYamlParser {
    fn parse(&self, raw_content: &str) -> Result<SkillDocument> {
        let mut hasher = Sha256::new();
        for line in raw_content.lines() {
            let trimmed = line.trim();
            if !trimmed.starts_with("digest:")
                && !trimmed.starts_with("signature:")
                && !trimmed.starts_with("signer_pubkey:")
            {
                hasher.update(line.as_bytes());
                hasher.update(b"\n");
            }
        }
        let digest = format!("asl:sha256:{}", hex::encode(hasher.finalize()));

        // 1. Extração do Frontmatter YAML delimitado por ---
        let (frontmatter_str, markdown_str) = extract_frontmatter_and_markdown(raw_content)?;

        // 2. Parse do YAML em SkillManifest
        let manifest: SkillManifest = serde_yaml::from_str(&frontmatter_str)
            .map_err(|e| AslError::InvalidFrontmatter(e.to_string()))?;
        manifest.validate()?;

        // 3. Parse de Markdown com pulldown-cmark
        let (semantic_section, deterministic_code) = parse_markdown_blocks(&markdown_str)?;

        let deterministic_code = if deterministic_code.trim().is_empty() {
            "def run(ctx, input):\n    return input".to_string()
        } else {
            deterministic_code
        };

        Ok(SkillDocument {
            manifest,
            semantic_section,
            deterministic_code,
            rules_code: None,
            digest,
        })
    }
}


fn extract_frontmatter_and_markdown(content: &str) -> Result<(String, String)> {
    let mut in_comment = false;
    let mut frontmatter_started = false;
    let mut frontmatter_lines = Vec::new();
    let mut markdown_lines = Vec::new();
    let mut frontmatter_ended = false;

    for line in content.lines() {
        let trimmed = line.trim();

        if !frontmatter_started {
            if trimmed.starts_with("<!--") {
                in_comment = true;
            }
            if in_comment {
                if trimmed.ends_with("-->") {
                    in_comment = false;
                }
                continue;
            }
            if trimmed.starts_with("#!") || trimmed.is_empty() {
                continue;
            }
            if trimmed == "---" {
                frontmatter_started = true;
                continue;
            }
            return Err(AslError::InvalidFrontmatter(
                "Esperado '---' delimitando início do YAML frontmatter.".to_string(),
            ));
        } else if !frontmatter_ended {
            if trimmed == "---" {
                frontmatter_ended = true;
                continue;
            }
            frontmatter_lines.push(line);
        } else {
            markdown_lines.push(line);
        }
    }

    if !frontmatter_ended {
        return Err(AslError::InvalidFrontmatter(
            "Frontmatter YAML não foi fechado com '---'.".to_string(),
        ));
    }

    Ok((frontmatter_lines.join("\n"), markdown_lines.join("\n")))
}

fn parse_markdown_blocks(markdown_raw: &str) -> Result<(String, String)> {
    let parser = Parser::new(markdown_raw).into_offset_iter();

    let mut code_blocks = Vec::new();
    let mut code_ranges = Vec::new();

    let mut in_target_code_block = false;
    let mut current_code = String::new();
    let mut current_start = 0;

    for (event, range) in parser {
        match event {
            Event::Start(Tag::CodeBlock(CodeBlockKind::Fenced(lang))) => {
                let tag_str = lang.trim().to_lowercase();
                if is_asl_code_tag(&tag_str) {
                    in_target_code_block = true;
                    current_code.clear();
                    current_start = range.start;
                }
            }
            Event::End(TagEnd::CodeBlock) => {
                if in_target_code_block {
                    code_blocks.push(current_code.clone());
                    code_ranges.push(current_start..range.end);
                    in_target_code_block = false;
                }
            }
            Event::Text(text) if in_target_code_block => {
                current_code.push_str(&text);
            }
            Event::SoftBreak | Event::HardBreak if in_target_code_block => {
                current_code.push('\n');
            }
            _ => {}
        }
    }

    let code = code_blocks.join("\n\n");

    let mut semantic_section = String::new();
    let mut last_idx = 0;
    for r in &code_ranges {
        if r.start > last_idx {
            semantic_section.push_str(&markdown_raw[last_idx..r.start]);
        }
        last_idx = r.end;
    }
    if last_idx < markdown_raw.len() {
        semantic_section.push_str(&markdown_raw[last_idx..]);
    }

    Ok((semantic_section.trim().to_string(), code.trim().to_string()))
}

fn is_asl_code_tag(tag: &str) -> bool {
    tag == "asl"
        || tag == "asl:deterministic"
        || tag == "starlark"
        || tag == "python-deterministic"
        || tag.starts_with("asl:")
}

#[cfg(test)]
mod tests {
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
# Instruções Semânticas
Execute a função determinística.

```asl:deterministic
def run(ctx, input):
    return {"status": "ok"}
```
"#;

        let parser = CommonMarkYamlParser::new();
        let doc = parser.parse(raw).expect("Parsing deve suceder");
        assert_eq!(doc.manifest.name, "test-skill");
        assert_eq!(doc.manifest.asl_version, "3.0");
        assert!(doc.deterministic_code.contains("def run(ctx, input)"));
        assert!(doc.semantic_section.contains("Instruções Semânticas"));
        assert!(doc.digest.starts_with("asl:sha256:"));
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
# Instruções Puras de Prompt
Você é um redator de documentação técnica.
"#;
        let parser = CommonMarkYamlParser::new();
        let doc = parser.parse(raw).expect("Skill puramente semântica deve ser válida");
        assert_eq!(doc.manifest.name, "pure-prompt");
        assert!(doc.deterministic_code.contains("def run(ctx, input)"));
        assert!(doc.semantic_section.contains("Instruções Puras de Prompt"));
    }
}
