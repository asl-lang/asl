use anyhow::{Context, Result};
use asl_core_traits::{EnginePort, ParserPort};
use asl_parser::{desugar_asl_code, CommonMarkYamlParser};
use asl_security::ConfinedSecurityContext;
use asl_spec::{Limits, SkillCapabilities};
use asl_vm_starlark::StarlarkEngine;
use std::fs;
use std::io::{self, BufRead, Write};
use std::path::Path;

/// Handles the interactive `asl repl` command
pub fn handle_repl(
    skill_file: Option<&Path>,
    parser: &CommonMarkYamlParser,
    engine: &StarlarkEngine,
) -> Result<()> {
    println!("⚡ Agent Skill Language (ASL 3.0) Interactive REPL");
    println!("Type ASL / Python expressions or statements.");
    println!("Special commands: :exit, :help, :clear");

    let mut preloaded_code = String::new();
    let mut limits = Limits {
        max_fuel_opcodes: 5_000_000,
        ..Default::default()
    };

    if let Some(path) = skill_file {
        let content = fs::read_to_string(path)
            .with_context(|| format!("Failed to read preload skill: {:?}", path))?;
        let doc = parser.parse(&content)?;
        preloaded_code = doc.deterministic_code;
        limits = doc.manifest.limits;
        println!(
            "Loaded skill: {} (entrypoint: {})",
            doc.manifest.name, doc.manifest.interface.entrypoint
        );
    }

    let stdin = io::stdin();
    let mut reader = stdin.lock();
    let mut line_buf = String::new();
    let mut session_code = preloaded_code;

    loop {
        print!("asl> ");
        io::stdout().flush()?;
        line_buf.clear();

        if reader.read_line(&mut line_buf)? == 0 {
            println!("\nGoodbye!");
            break;
        }

        let trimmed = line_buf.trim();
        if trimmed.is_empty() {
            continue;
        }

        match trimmed {
            ":exit" | ":q" | "exit" | "quit" => {
                println!("Goodbye!");
                break;
            }
            ":help" => {
                println!("ASL REPL Help:");
                println!("  chars(\"abc\")              - Iterate string as characters");
                println!("  to_int(\"42\")               - Safe integer conversion");
                println!("  to_float(\"3.14\")           - Safe float conversion");
                println!("  ctx.fs.read(path)          - Read file contents");
                println!("  ctx.fs.write(path, data)   - Write file contents");
                println!("  ctx.fs.list(path)          - List directory entries");
                println!("  ctx.crypto.sha256(s)       - SHA-256 hash");
                println!("  ctx.crypto.base64_encode(s)- Base64 encode");
                println!("  ctx.env.get(\"VAR\")         - Read environment variable");
                println!("  :exit, :q                  - Exit REPL");
                continue;
            }
            ":clear" => {
                session_code.clear();
                println!("Session cleared.");
                continue;
            }
            _ => {}
        }

        let desugared = desugar_asl_code(trimmed);

        // Try evaluating as an expression first
        let expr_code = format!(
            "{}\ndef __repl_expr__(ctx, input):\n    return ({})\n",
            session_code, desugared
        );

        let mut permissive_caps = SkillCapabilities::default();
        permissive_caps.fs.confined_read_roots = vec![".".to_string(), "~".to_string()];
        permissive_caps.fs.allow_write = vec![".".to_string(), "~".to_string()];
        permissive_caps.env.allow_keys = vec!["*".to_string()];
        permissive_caps.net.allow_domains = vec!["*".to_string()];

        let sec =
            ConfinedSecurityContext::from_capabilities(&permissive_caps, limits.max_fuel_opcodes);
        let dummy_input = serde_json::json!({});

        match engine.execute(&expr_code, "__repl_expr__", &dummy_input, &sec, &limits) {
            Ok(res) => {
                if !res.output.is_null() {
                    let out_str = serde_json::to_string_pretty(&res.output)?;
                    println!("=> {}", out_str);
                }
            }
            Err(_) => {
                // If expression returned error, try statement execution
                let stmt_code = format!(
                    "{}\n{}\ndef __repl_stmt__(ctx, input):\n    return True\n",
                    session_code, desugared
                );
                match engine.execute(&stmt_code, "__repl_stmt__", &dummy_input, &sec, &limits) {
                    Ok(_) => {
                        session_code.push('\n');
                        session_code.push_str(&desugared);
                    }
                    Err(e) => {
                        eprintln!("Error: {}", e);
                    }
                }
            }
        }
    }

    Ok(())
}
