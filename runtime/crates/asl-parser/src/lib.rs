pub mod grammar;
pub use grammar::GbnfGrammarCompiler;

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
            if !trimmed.starts_with("digest:") {
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

        // 3. Parse de Markdown com pulldown-cmark
        let (semantic_section, deterministic_code) = parse_markdown_blocks(&markdown_str)?;

        if deterministic_code.trim().is_empty() {
            return Err(AslError::MissingDeterministicBlock);
        }

        Ok(SkillDocument {
            manifest,
            semantic_section,
            deterministic_code,
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
    let parser = Parser::new(markdown_raw);

    let mut semantic_section = String::new();
    let mut code_blocks = Vec::new();

    let mut in_target_code_block = false;
    let mut current_code = String::new();

    for event in parser {
        match event {
            Event::Start(Tag::CodeBlock(CodeBlockKind::Fenced(lang))) => {
                let tag_str = lang.trim().to_lowercase();
                if is_asl_code_tag(&tag_str) {
                    in_target_code_block = true;
                    current_code.clear();
                } else {
                    semantic_section.push_str(&format!("\n```{}\n", lang));
                }
            }
            Event::End(TagEnd::CodeBlock) => {
                if in_target_code_block {
                    code_blocks.push(current_code.clone());
                    in_target_code_block = false;
                } else {
                    semantic_section.push_str("```\n");
                }
            }
            Event::Text(text) => {
                if in_target_code_block {
                    current_code.push_str(&text);
                } else {
                    semantic_section.push_str(&text);
                }
            }
            Event::Code(code) => {
                if !in_target_code_block {
                    semantic_section.push('`');
                    semantic_section.push_str(&code);
                    semantic_section.push('`');
                }
            }
            Event::SoftBreak | Event::HardBreak => {
                if in_target_code_block {
                    current_code.push('\n');
                } else {
                    semantic_section.push('\n');
                }
            }
            _ => {}
        }
    }

    let code = code_blocks.join("\n\n");
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
}
