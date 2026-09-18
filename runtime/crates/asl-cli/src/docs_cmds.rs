use anyhow::{bail, Result};
use crate::docs_rosetta::*;

/// Handles `asl docs` / `asl learn` / `asl syntax`
pub fn handle_docs(topic: &str, ai: bool, json: bool) -> Result<()> {
    if ai {
        println!("{}", get_ai_primer());
        return Ok(());
    }

    if json {
        println!("{}", get_json_docs(topic)?);
        return Ok(());
    }

    match topic.to_lowercase().as_str() {
        "syntax" => println!("{}", get_syntax_docs()),
        "rules" => println!("{}", get_rules_docs()),
        "triad" => println!("{}", get_triad_docs()),
        "practices" | "best-practices" | "manual" => println!("{}", get_practices_docs()),
        "mcp" => println!("{}", get_mcp_docs()),
        "examples" => println!("{}", get_examples_docs()),
        "migration" | "rosetta" => println!("{}", get_migration_docs()),
        "bash" | "sh" | "shell" => println!("{}", get_bash_migration_docs()),
        "python" | "py" => println!("{}", get_python_migration_docs()),
        "starlark" => println!("{}", get_starlark_migration_docs()),
        "overview" | "all" | "" => println!("{}", get_overview_docs()),
        other => {
            eprintln!("⚠️  Unknown topic '{}'. Showing language overview:\n", other);
            println!("{}", get_overview_docs());
        }
    }

    Ok(())
}

/// Handles `asl template <type>`
pub fn handle_template(target_type: &str) -> Result<()> {
    match target_type.to_lowercase().as_str() {
        "skill" => print!("{}", get_skill_template()),
        "tool" => print!("{}", get_tool_template()),
        "rules" => print!("{}", get_rules_template()),
        "asl" => print!("{}", get_asl_template()),
        other => bail!("Unknown template '{}'. Use: skill, tool, rules, or asl.", other),
    }
    Ok(())
}

fn get_ai_primer() -> &'static str {
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

fn get_overview_docs() -> &'static str {
    r#"========================================================================
⚡ Agent Skill Language (ASL 3.0) - Language Reference & Cheat Sheet
========================================================================

ASL is an open specification where programming feels just like writing
Markdown. Your document IS your program — combining plain prose for AI
agents and humans with concise ```asl code blocks. Under the hood, ASL
transpiles AOT into hermetic, fuel-metered Starlark in RAM.

🚀 QUICK START:
  asl template skill > my-skill.skill    # Generate canonical skill template
  chmod +x my-skill.skill               # Make executable
  ./my-skill.skill                      # Run directly via universal shebang!
  asl run my-skill.skill                # Run via hermetic runtime

📚 TOPICS (run 'asl docs <topic>' or 'asl learn <topic>'):
  asl docs syntax      - YAML frontmatter, schema, universal ```asl tag
  asl docs rules       - Declarative semantic rules DSL (match/when/guard)
  asl docs migration   - ASL Rosetta Stone (Migrate from Bash/Python/Starlark)
  asl docs bash        - How to replace Bash scripts with pure ASL
  asl docs python      - How to replace Python automations with pure ASL
  asl docs triad       - The canonical file triad (.skill, .tool, .asl)
  asl docs practices   - Best practices & on-demand workflow (no daemon)
  asl docs mcp         - Exposing skills as Model Context Protocol tools
  asl docs examples    - Real-world, copy-pasteable runnable code
  asl docs --ai        - Ultra-dense primer optimized for AI agent prompts
  asl template <type>  - Output starter templates (skill, tool, rules, asl)"#
}

fn get_syntax_docs() -> &'static str {
    r#"### ASL 3.0 Syntax Reference

1. Universal Shebang (Line 1):
   #!/usr/bin/env -S asl run

2. YAML Frontmatter (Delimited by ---):
   name: string                  (Required: lowercase alphanumeric, hyphens)
   asl_version: "3.0"            (Required)
   description: string           (Optional but recommended for LLM discovery)
   interface:
     entrypoint: string          (Default: "run")
     protocol: string            (Optional: "mcp-tool-v1")
     input_schema: object        (Optional JSON Schema for inputs)
     output_schema: object       (Optional JSON Schema for outputs)
   capabilities:
     fs: [string]                (Confined sandbox directory paths)
     domains: [string]           (Allowed network domains)
   limits:
     max_fuel_opcodes: integer   (Default: 1,000,000)
     max_heap_kib: integer       (Default: 8,192 KiB)
     wall_clock_timeout_ms: int  (Default: 1,000 ms)

3. Universal Code Block Tag:
   ```asl                        The single unified tag for all ASL code.
                                 Accepts declarative rules or functions."#
}

fn get_rules_docs() -> &'static str {
    r#"### Declarative Semantic Rules DSL in ASL

Declarative rules allow writing readable, deterministic guards and actions:

Syntax:
match <expression>:
  when <pattern>:
    [guard <boolean_condition>]
    return <result_expression>

Supported Patterns:
  • String literals:  when "deploy":
  • Integers / Bool:  when 200: / when True:
  • Wildcard:         when _:
  • Substring:        when contains "needle":
  • Prefix / Suffix:  when starts_with "prefix": / ends_with ".json":

Example:
```asl
match input.command:
  when "format":
    guard input.text != ""
    return {"formatted": input.text.strip()}
  when "validate":
    return {"valid": True}
  otherwise:
    return {"error": "unsupported command"}
```"#
}

fn get_triad_docs() -> &'static str {
    r#"### The ASL Canonical File Triad

1. .skill (Modular Agent Skill)
   • Intended for AI agent behaviors, user pair programming, workflows.
   • Features automatic Markdown (.md) shadow projection by the daemon.
   • Self-describing shebang: #!/usr/bin/env -S asl run.

2. .tool (Deterministic MCP Tool)
   • Intended for direct tool calls, MCP servers, API endpoints.
   • No shadow projection (operates cleanly and directly).
   • Self-describing shebang: #!/usr/bin/env -S asl run.

3. .asl (Native Core Module)
   • Intended for shared domain logic, libraries, and core rules.
   • No shadow projection.
   • Self-describing shebang: #!/usr/bin/env -S asl run."#
}

fn get_practices_docs() -> &'static str {
    r#"### ASL Best Practices & On-Demand Workflow (No Daemon)

If you prefer NOT to run a background daemon, follow these canonical practices:

1. Always Place Shebang on Line 1:
   #!/usr/bin/env -S asl run
   • Instructs AI agents and tools how to execute the file upon inspection.
   • Allows direct terminal execution: `./my-skill.skill` (after chmod +x).

2. Dual-Consumer Discovery without MCP:
   If your AI editor (Claude, Cursor) reads Markdown (.md):
   • Run `asl sync .` on-demand to project all .md shadows in < 5ms.
   • Or use `asl check <file>` to validate and update the shadow upon save.
   • The .md points canonically to the .skill via asl_canonical_source.

3. Ephemeral Foreground Watcher:
   During active development sessions, run a temporary watcher in your terminal:
   $ asl watch .
   • Automatically projects .md shadows live as you edit.
   • Terminate anytime with Ctrl+C (zero background residue or daemons).

4. Self-Describing Header Directive:
   For environments where shebang is not executed directly:
   # Execution: asl run ./<file>.skill
   # Runtime: https://github.com/asl-lang/asl"#
}

fn get_mcp_docs() -> &'static str {
    r#"### Model Context Protocol (MCP) Integration

ASL skills and tools can be served natively as MCP servers:

1. Stdio Transport (Default for Claude Desktop / Cursor):
   asl serve ./my-skill.skill --transport stdio

2. HTTP / Server-Sent Events (SSE) Transport:
   asl serve ./my-skill.skill --transport sse --host 127.0.0.1 --port 3000

Declaring an MCP Tool Interface:
---
name: "calculator"
asl_version: "3.0"
interface:
  protocol: "mcp-tool-v1"
  entrypoint: "run"
  input_schema:
    type: "object"
    properties:
      a: { type: "number" }
      b: { type: "number" }
    required: ["a", "b"]
---"#
}

fn get_examples_docs() -> &'static str {
    r#"### Canonical ASL Examples

1. Hello World Skill:
```yaml
#!/usr/bin/env -S asl run
---
asl_version: "3.0"
name: "hello-world"
description: "Greets the user"
---
# Hello World
Say hello deterministically.

```asl
def run(ctx, input):
    name = input.get("name", "World")
    return {"message": "Hello, " + name + "!"}
```
```

2. Semantic Guard Skill:
```yaml
#!/usr/bin/env -S asl run
---
asl_version: "3.0"
name: "input-guard"
description: "Guards against malicious shell injection"
---
# Guard Instructions
Validates prompt parameters.

```asl
match input.text:
  when contains ";":
    return {"allowed": False, "reason": "Shell delimiter detected"}
  otherwise:
    return {"allowed": True}
```
```"#
}

fn get_skill_template() -> &'static str {
    r#"#!/usr/bin/env -S asl run
---
asl_version: "3.0"
name: "custom-skill"
description: "A hermetic agent skill"
interface:
  entrypoint: "run"
---
# Instructions
Describe semantic behavior for AI agents here.

```asl
def run(ctx, input):
    return {"status": "ok", "echo": input}
```
"#
}

fn get_tool_template() -> &'static str {
    r#"#!/usr/bin/env -S asl run
---
asl_version: "3.0"
name: "custom-tool"
description: "A deterministic MCP tool"
interface:
  protocol: "mcp-tool-v1"
  entrypoint: "run"
  input_schema:
    type: "object"
    properties:
      query: { type: "string" }
    required: ["query"]
---
# Tool Documentation
Explain tool usage here.

```asl
def run(ctx, input):
    return {"processed": input.get("query", "")}
```
"#
}

fn get_rules_template() -> &'static str {
    r#"#!/usr/bin/env -S asl run
---
asl_version: "3.0"
name: "rules-skill"
description: "Skill governed by declarative semantic rules"
---
# Rules Instructions
Applies deterministic policy matchers.

```asl
match input.action:
  when "ping":
    return {"status": "pong"}
  otherwise:
    return {"error": "unknown action"}
```
"#
}

fn get_asl_template() -> &'static str {
    r#"#!/usr/bin/env -S asl run
---
asl_version: "3.0"
name: "core-module"
description: "Native ASL module"
---
# Core Module
Shared logic and functions.

```asl
def run(ctx, input):
    return {"module": "ready"}
```
"#
}

fn get_json_docs(topic: &str) -> Result<String> {
    let val = serde_json::json!({
        "asl_version": "3.0",
        "topic": topic,
        "runtime": "asl_vm_hermetic",
        "shebang": "#!/usr/bin/env -S asl run",
        "code_tag": "asl",
        "triad": [".skill", ".tool", ".asl"],
        "available_topics": [
            "overview", "syntax", "rules", "migration", "bash",
            "python", "starlark", "triad", "mcp", "examples", "ai"
        ]
    });
    serde_json::to_string_pretty(&val).map_err(Into::into)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_handle_docs_all_topics() {
        for topic in [
            "overview", "syntax", "rules", "triad", "practices",
            "best-practices", "mcp", "examples", "migration", "rosetta",
            "bash", "python", "starlark", "all", "unknown"
        ] {
            assert!(handle_docs(topic, false, false).is_ok());
        }
        assert!(handle_docs("all", true, false).is_ok());
        assert!(handle_docs("all", false, true).is_ok());
    }

    #[test]
    fn test_handle_template_all_types() {
        for t in ["skill", "tool", "rules", "asl"] {
            assert!(handle_template(t).is_ok());
        }
        assert!(handle_template("invalid").is_err());
    }
}
