pub mod grammar;
pub mod prefix_analyzer;
pub mod rules;
pub mod shadow;

pub use grammar::GbnfGrammarCompiler;
pub use prefix_analyzer::*;
pub use rules::{
    parse_rules, Action, GuardClause, MatchSection, PathExpr, PatternCondition, RulesBlock,
    WhenClause,
};
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
        let raw_content = raw_content.strip_prefix('\u{feff}').unwrap_or(raw_content);
        if raw_content.trim().is_empty() {
            return Ok(SkillDocument::draft_scaffold("draft-skill"));
        }

        // 1. Extração do Frontmatter YAML delimitado por ---
        let (frontmatter_str, markdown_str) = extract_frontmatter_and_markdown(raw_content)?;

        // 2. Parse YAML into SkillManifest or infer from Markdown (ADR-0015)
        let manifest = if frontmatter_str.trim().is_empty() {
            infer_manifest_from_markdown(&markdown_str)
        } else {
            let m: SkillManifest = serde_yaml::from_str(&frontmatter_str)
                .map_err(|e| AslError::InvalidFrontmatter(e.to_string()))?;
            m
        };
        manifest.validate()?;

        // 3. Canonical digest calculation: signature fields are omitted EXCLUSIVELY from the frontmatter
        let mut hasher = Sha256::new();
        if frontmatter_str.trim().is_empty() {
            for line in raw_content.lines() {
                hasher.update(line.as_bytes());
                hasher.update(b"\n");
            }
        } else {
            let mut in_frontmatter = false;
            let mut frontmatter_ended = false;

            for line in raw_content.lines() {
                let trimmed = line.trim();
                if !in_frontmatter && !frontmatter_ended {
                    if trimmed == "---" {
                        in_frontmatter = true;
                    }
                    hasher.update(line.as_bytes());
                    hasher.update(b"\n");
                } else if in_frontmatter && !frontmatter_ended {
                    if trimmed == "---" {
                        in_frontmatter = false;
                        frontmatter_ended = true;
                        hasher.update(line.as_bytes());
                        hasher.update(b"\n");
                    } else if !trimmed.starts_with("digest:")
                        && !trimmed.starts_with("signature:")
                        && !trimmed.starts_with("signer_pubkey:")
                    {
                        hasher.update(line.as_bytes());
                        hasher.update(b"\n");
                    }
                } else {
                    // Markdown body and code: all lines included in digest
                    hasher.update(line.as_bytes());
                    hasher.update(b"\n");
                }
            }
        }
        let digest = format!("asl:sha256:{}", hex::encode(hasher.finalize()));

        // 4. Parse de Markdown com pulldown-cmark
        let parsed_blocks = parse_markdown_blocks(&markdown_str)?;

        let (deterministic_code, rules_code) = if let Some(rules_src) = parsed_blocks.rules_code {
            use asl_core_traits::RulesTranspilerPort;
            let transpiler = rules::RulesTranspiler::new();
            let transpiled = transpiler.transpile(&rules_src, &manifest)?;
            let code = if !parsed_blocks.deterministic_code.trim().is_empty() {
                format!(
                    "{}\n\n# --- CÓDIGO DETERMINÍSTICO MANUAL EMBUTIDO ---\n{}",
                    transpiled.starlark_code, parsed_blocks.deterministic_code
                )
            } else {
                transpiled.starlark_code
            };
            (code, Some(rules_src))
        } else {
            let code = if parsed_blocks.deterministic_code.trim().is_empty() {
                format!(
                    "def {}(ctx, input):\n    return input",
                    manifest.interface.entrypoint
                )
            } else {
                parsed_blocks.deterministic_code
            };
            (code, None)
        };

        Ok(SkillDocument {
            manifest,
            semantic_section: parsed_blocks.semantic_section,
            deterministic_code,
            rules_code,
            digest,
        })
    }
}

fn infer_manifest_from_markdown(markdown: &str) -> SkillManifest {
    let mut name = None;
    let mut desc = None;

    for line in markdown.lines() {
        let trimmed = line.trim();
        if trimmed.is_empty() || trimmed.starts_with("<!--") || trimmed.starts_with("#!") {
            continue;
        }
        if name.is_none() && trimmed.starts_with('#') {
            let header_text = trimmed.trim_start_matches('#').trim();
            if !header_text.is_empty() {
                let clean_name: String = header_text
                    .chars()
                    .map(|c| if c.is_ascii_alphanumeric() || c == '-' || c == '_' { c } else { '-' })
                    .collect();
                let clean_name = clean_name.trim_matches('-').to_lowercase();
                if !clean_name.is_empty() {
                    name = Some(clean_name);
                    continue;
                }
            }
        }
        if desc.is_none() && !trimmed.starts_with('#') && !trimmed.starts_with("```") {
            desc = Some(trimmed.to_string());
        }
        if name.is_some() && desc.is_some() {
            break;
        }
    }

    SkillManifest {
        asl_version: "3.0".to_string(),
        name: name.unwrap_or_else(|| "legacy-skill".to_string()),
        description: desc.unwrap_or_else(|| "Imported markdown skill".to_string()),
        interface: asl_spec::SkillInterface::default(),
        capabilities: asl_spec::SkillCapabilities::default(),
        limits: asl_spec::SkillLimits::default(),
        digest: None,
        signature: None,
        signer_pubkey: None,
        version: None,
        license: None,
    }
}

fn extract_frontmatter_and_markdown(content: &str) -> Result<(String, String)> {
    let content = content.strip_prefix('\u{feff}').unwrap_or(content);
    if content.trim().is_empty() {
        return Ok((String::new(), String::new()));
    }

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
            if trimmed.starts_with("asl_version:") {
                return Err(AslError::InvalidFrontmatter(
                    "Expected '---' frontmatter opening delimiter.".to_string(),
                ));
            }
            // Tolerant legacy markdown ingestion (ADR-0015):
            // When renaming an existing markdown file or creating a markdown-first skill
            // without YAML delimiters, ingest entire content as markdown.
            return Ok((String::new(), content.to_string()));
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
            "YAML frontmatter was not closed with '---'.".to_string(),
        ));
    }

    Ok((frontmatter_lines.join("\n"), markdown_lines.join("\n")))
}

struct ParsedBlocks {
    pub semantic_section: String,
    pub deterministic_code: String,
    pub rules_code: Option<String>,
}

fn parse_markdown_blocks(markdown_raw: &str) -> Result<ParsedBlocks> {
    let parser = Parser::new(markdown_raw).into_offset_iter();

    let mut deterministic_blocks = Vec::new();
    let mut rules_blocks = Vec::new();
    let mut current_block_kind: Option<String> = None;
    let mut semantic_section = String::new();
    let mut last_event_end = 0;

    for (event, range) in parser {
        match event {
            Event::Start(Tag::CodeBlock(CodeBlockKind::Fenced(tag))) => {
                let tag_str = tag.as_ref().trim().to_lowercase();
                if is_rules_code_tag(&tag_str) || is_deterministic_code_tag(&tag_str) {
                    current_block_kind = Some(tag_str);
                    if range.start > last_event_end {
                        semantic_section.push_str(&markdown_raw[last_event_end..range.start]);
                    }
                }
            }
            Event::Text(text) => {
                if let Some(ref tag) = current_block_kind {
                    if is_rules_code_tag(tag) {
                        rules_blocks.push(text.to_string());
                    } else if is_deterministic_code_tag(tag) {
                        deterministic_blocks.push(text.to_string());
                    }
                }
            }
            Event::End(TagEnd::CodeBlock) => {
                if current_block_kind.is_some() {
                    current_block_kind = None;
                    last_event_end = range.end;
                }
            }
            _ => {}
        }
    }

    if last_event_end < markdown_raw.len() {
        semantic_section.push_str(&markdown_raw[last_event_end..]);
    }

    let deterministic_code = deterministic_blocks.join("\n");
    let rules_code = if rules_blocks.is_empty() {
        None
    } else {
        Some(rules_blocks.join("\n"))
    };

    Ok(ParsedBlocks {
        semantic_section: semantic_section.trim().to_string(),
        deterministic_code,
        rules_code,
    })
}

fn is_rules_code_tag(tag: &str) -> bool {
    tag == "asl:rules" || tag == "rules"
}

fn is_deterministic_code_tag(tag: &str) -> bool {
    tag == "asl"
        || tag == "asl:deterministic"
        || tag == "starlark"
        || tag == "python-deterministic"
        || (tag.starts_with("asl:") && tag != "asl:rules")
}

#[cfg(test)]
mod parser_tests;
