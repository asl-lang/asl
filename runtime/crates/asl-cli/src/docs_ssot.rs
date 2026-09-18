//! Single Source of Truth (SSOT) embedded provider for ASL documentation.
//! Adheres strictly to Axiom 2 (Zero Runtime Dependencies) and Axiom 7 (< 450 lines).

use anyhow::Result;
use serde::Deserialize;

pub const COMMANDS_JSON: &str = include_str!("../../../../docs/spec/commands.json");
pub const TOPICS_JSON: &str = include_str!("../../../../docs/spec/topics.json");

#[allow(dead_code)]
#[derive(Debug, Clone, Deserialize)]
pub struct CommandsSpec {
    pub version: String,
    pub commands: Vec<CommandDoc>,
}

#[allow(dead_code)]
#[derive(Debug, Clone, Deserialize)]
pub struct CommandDoc {
    pub id: String,
    pub name: String,
    #[serde(default)]
    pub aliases: Vec<String>,
    pub category: String,
    pub summary: String,
    pub description: String,
    #[serde(default)]
    pub arguments: Vec<ArgDoc>,
    #[serde(default)]
    pub flags: Vec<FlagDoc>,
    #[serde(default)]
    pub examples: Vec<ExampleDoc>,
    #[serde(default)]
    pub ai_primer: Option<AiPrimerDoc>,
}

#[allow(dead_code)]
#[derive(Debug, Clone, Deserialize)]
pub struct ArgDoc {
    pub name: String,
    pub r#type: String,
    pub required: bool,
    #[serde(default)]
    pub default: Option<String>,
    pub description: String,
}

#[allow(dead_code)]
#[derive(Debug, Clone, Deserialize)]
pub struct FlagDoc {
    pub name: String,
    #[serde(default)]
    pub short: Option<String>,
    pub long: String,
    pub r#type: String,
    pub required: bool,
    #[serde(default)]
    pub default: Option<String>,
    pub description: String,
}

#[allow(dead_code)]
#[derive(Debug, Clone, Deserialize)]
pub struct ExampleDoc {
    pub cmd: String,
    pub desc: String,
}

#[allow(dead_code)]
#[derive(Debug, Clone, Deserialize)]
pub struct AiPrimerDoc {
    pub usage: String,
    pub token_estimate: u32,
}

#[allow(dead_code)]
#[derive(Debug, Clone, Deserialize)]
pub struct TopicsSpec {
    pub version: String,
    pub topics: Vec<TopicDoc>,
}

#[allow(dead_code)]
#[derive(Debug, Clone, Deserialize)]
pub struct TopicDoc {
    pub id: String,
    pub title: String,
    pub summary: String,
    #[serde(default)]
    pub ai_primer: Option<String>,
    pub content: String,
}

#[allow(dead_code)]
pub fn get_commands_spec() -> Result<CommandsSpec> {
    serde_json::from_str(COMMANDS_JSON).map_err(Into::into)
}

#[allow(dead_code)]
pub fn get_topics_spec() -> Result<TopicsSpec> {
    serde_json::from_str(TOPICS_JSON).map_err(Into::into)
}

pub fn find_topic_content(topic_id: &str) -> Result<Option<String>> {
    let spec = get_topics_spec()?;
    for t in spec.topics {
        if t.id.eq_ignore_ascii_case(topic_id) {
            return Ok(Some(t.content));
        }
    }
    Ok(None)
}

pub fn get_ai_primer() -> &'static str {
    r#"# ASL 3.0 Primer for AI Agents
Execute: `asl run <file>` or `./<file>` | ASL VM Hermetic Sandbox | Zero overhead
File Triad: .skill (modular, auto-shadows to .md), .tool (MCP tool), .asl (native root)
Universal Code Tag: All code blocks use strictly ```asl

Structure of .skill / .tool:
```yaml
#!/usr/bin/env -S asl run
---
asl_version: "3.0"
name: "my-skill"
description: "Clear description for LLM dispatch"
interface:
  entrypoint: "run"
  protocol: "mcp-tool-v1"
capabilities:
  fs: ["./data"]
limits:
  max_fuel_opcodes: 1000000
---
# Semantic Instructions (for LLMs)
Instructions and prompting guidelines go here.

```asl
match input.action:
  when "query":
    return {"status": "ok", "term": input.term}
  otherwise:
    return {"error": "unknown action"}
```
"#
}

#[cfg(test)]
mod tests {
    use super::*;
    use clap::CommandFactory;

    #[test]
    fn test_embedded_specs_validity() {
        let commands = get_commands_spec().expect("commands.json should deserialize");
        assert!(!commands.commands.is_empty(), "commands should not be empty");

        let topics = get_topics_spec().expect("topics.json should deserialize");
        assert!(!topics.topics.is_empty(), "topics should not be empty");
    }

    #[test]
    fn test_zero_drift_clap_vs_ssot() {
        let cmd = crate::Cli::command();
        let spec = get_commands_spec().expect("commands.json should deserialize");

        // Verify every Clap subcommand exists in SSOT spec
        for subcmd in cmd.get_subcommands() {
            let subcmd_name = subcmd.get_name();
            // Skip auto-generated help
            if subcmd_name == "help" {
                continue;
            }

            let found = spec.commands.iter().any(|c| {
                c.name == subcmd_name
                    || c.id == subcmd_name
                    || c.aliases.iter().any(|a| a == subcmd_name)
            });

            assert!(
                found,
                "Drift detected: Clap subcommand '{}' is missing from docs/spec/commands.json!",
                subcmd_name
            );
        }

        // Verify every SSOT command exists in Clap
        for c in &spec.commands {
            let exists_in_clap = cmd.get_subcommands().any(|sc| {
                sc.get_name() == c.name
                    || sc.get_name() == c.id
                    || sc.get_all_aliases().any(|a| a == c.name || a == c.id)
            });

            assert!(
                exists_in_clap,
                "Drift detected: SSOT command '{}' is not registered in Clap runtime/crates/asl-cli/src/main.rs!",
                c.name
            );
        }
    }
}
