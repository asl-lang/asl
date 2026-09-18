use anyhow::{Context, Result};
use asl_core_traits::{EnginePort, GrammarCompilerPort, ParserPort};
use asl_parser::{CommonMarkYamlParser, GbnfGrammarCompiler};
use asl_security::ConfinedSecurityContext;
use asl_vm_starlark::StarlarkEngine;
use clap::{Parser, Subcommand};
use serde_json::Value;
use std::fs;
use std::path::PathBuf;

mod crypto_cmds;
mod daemon_cmds;
mod lifecycle_cmds;
mod prefix_cmds;
mod server_cmds;
mod shadow_cmds;

#[derive(Parser)]
#[command(name = "asl")]
#[command(about = "Agent Skill Language (ASL 3.0) Runtime & Tooling", long_about = None)]
#[command(version = "3.0.0")]
struct Cli {
    #[command(subcommand)]
    command: Commands,
}

#[derive(Subcommand)]
enum Commands {
    /// Executes a deterministic function from an ASL file (.skill, .tool, .asl)
    Run {
        /// Path to the ASL file (.skill, .tool, .asl)
        skill_file: PathBuf,

        /// Name of the entrypoint function to execute (optional if declared in manifest)
        #[arg(short, long)]
        entrypoint: Option<String>,

        /// Input arguments in JSON format
        #[arg(short, long, default_value = "{}")]
        input: String,

        /// Allowed host roots for intersection with requested capabilities (Host Policy)
        #[arg(long)]
        allowed_root: Vec<PathBuf>,
    },

    /// Validates and audits the integrity of an ASL file (.skill, .tool, .asl)
    Check {
        /// Path to the ASL file (.skill, .tool, .asl)
        skill_file: PathBuf,
    },

    /// Starts a Model Context Protocol (MCP) server over stdio or HTTP/SSE
    Serve {
        /// Paths to ASL files or directory containing ASL artifacts
        #[arg(default_value = ".")]
        path: PathBuf,

        /// Communication transport (stdio or http)
        #[arg(short, long, default_value = "stdio")]
        transport: String,

        /// Host bind address for HTTP server (default: 127.0.0.1)
        #[arg(long, default_value = "127.0.0.1")]
        host: String,

        /// Port for HTTP server (only used when --transport http)
        #[arg(short, long, default_value = "8080")]
        port: u16,
    },

    /// Compiles .skill JSON schema into LLM constrained sampling grammars
    CompileGrammar {
        /// Path to the .skill file
        skill_file: PathBuf,

        /// Grammar format (gbnf or regex)
        #[arg(short, long, default_value = "gbnf")]
        format: String,
    },

    /// Generates an Ed25519 cryptographic keypair for skill signing
    Keygen {
        /// Prefix or base path to save .priv and .pub key files (optional)
        #[arg(short, long)]
        out: Option<PathBuf>,
    },

    /// Digitally signs an ASL file using Ed25519
    Sign {
        /// Path to the .skill file
        skill_file: PathBuf,

        /// Ed25519 private key (direct hex string or path to key file)
        #[arg(short, long)]
        key: String,
    },

    /// Verifies the Ed25519 cryptographic signature of an ASL file
    Verify {
        /// Path to the .skill file
        skill_file: PathBuf,

        /// Ed25519 public key (optional if declared in manifest)
        #[arg(short, long)]
        pubkey: Option<String>,
    },

    /// Analyzes static prefix and projects KV-Cache hit rate (Axiom 6)
    AnalyzePrefix {
        /// Path to the .skill file
        skill_file: PathBuf,
    },

    /// Optimizes semantic prompt by consolidating static invariant blocks at the top
    OptimizePrefix {
        /// Path to the .skill file
        skill_file: PathBuf,

        /// Overwrites the file directly with the optimized version
        #[arg(short, long)]
        in_place: bool,
    },

    /// Synchronizes all Markdown (.md) shadow projections for .skill files
    #[command(alias = "sync")]
    SyncShadows {
        /// Path to file or base directory (default: '.')
        #[arg(default_value = ".")]
        path: PathBuf,
    },

    /// Monitors directory and projects shadows in real-time
    Watch {
        /// Path to directory to monitor (default: '.')
        #[arg(default_value = ".")]
        path: PathBuf,

        /// Polling interval in milliseconds (default: 500ms)
        #[arg(short, long, default_value = "500")]
        interval: u64,
    },

    /// Inspects in-memory transpiled Starlark code from declarative rules
    Expand {
        /// Path to the .skill file
        skill_file: PathBuf,
    },

    /// Manages native background zero-touch shadow projection daemon (launchd/systemd/tasks)
    Daemon {
        #[command(subcommand)]
        action: daemon_cmds::DaemonAction,
    },

    /// Interactive guided setup for configuring ASL and background services
    Setup,

    /// Updates ASL binary to the latest release from the official channel
    #[command(alias = "upgrade", alias = "self-update")]
    Update {
        /// Only check for updates without downloading
        #[arg(short, long)]
        check: bool,

        /// Force re-installation even if already on the latest version
        #[arg(short, long)]
        force: bool,
    },

    /// Uninstalls ASL and removes the binary from the system
    #[command(alias = "self-uninstall")]
    Uninstall {
        /// Automatically confirm uninstallation without prompting
        #[arg(short, long)]
        yes: bool,

        /// Purge configuration and daemon runtime files (~/.asl)
        #[arg(short, long)]
        purge: bool,
    },
}

fn main() -> Result<()> {
    let cli = Cli::parse();
    let parser = CommonMarkYamlParser::new();
    let engine = StarlarkEngine::new();
    let grammar_compiler = GbnfGrammarCompiler::new();

    match cli.command {
        Commands::Run {
            skill_file,
            entrypoint,
            input,
            allowed_root,
        } => {
            let content = fs::read_to_string(&skill_file)
                .with_context(|| format!("Failed to read file: {:?}", skill_file))?;

            let doc = parser
                .parse(&content)
                .with_context(|| "Error parsing ASL file")?;

            // Zero-Touch Hook: project or update shadow Markdown
            let _ = asl_parser::project_shadow_markdown(&skill_file, &doc);

            let ep = entrypoint
                .or_else(|| Some(doc.manifest.interface.entrypoint.clone()))
                .filter(|s| !s.is_empty())
                .unwrap_or_else(|| "run".to_string());

            let input_val: Value = serde_json::from_str(&input)
                .with_context(|| format!("Argument --input is not valid JSON: {}", input))?;

            let mut effective_caps = doc.manifest.capabilities.clone();
            if !allowed_root.is_empty() {
                let allowed_canon: Vec<PathBuf> = allowed_root
                    .iter()
                    .map(|p| fs::canonicalize(p).unwrap_or_else(|_| p.clone()))
                    .collect();
                effective_caps.fs.confined_read_roots.retain(|r| {
                    let r_path = PathBuf::from(r);
                    let r_canon = fs::canonicalize(&r_path).unwrap_or(r_path);
                    allowed_canon.iter().any(|a| r_canon.starts_with(a))
                });
            }

            let security = ConfinedSecurityContext::from_capabilities(
                &effective_caps,
                doc.manifest.limits.max_fuel_opcodes,
            );

            let result = engine
                .execute(
                    &doc.deterministic_code,
                    &ep,
                    &input_val,
                    &security,
                    &doc.manifest.limits,
                )
                .with_context(|| "Failed deterministic ASL execution")?;

            let output_str = serde_json::to_string_pretty(&result.output)?;
            println!("{}", output_str);
        }

        Commands::Check { skill_file } => {
            let content = fs::read_to_string(&skill_file)
                .with_context(|| format!("Failed to read file: {:?}", skill_file))?;

            let doc = parser
                .parse(&content)
                .with_context(|| "Validation failed: error parsing ASL file")?;

            println!("✅ ASL file validated successfully!");
            println!("Name:         {}", doc.manifest.name);
            println!("ASL Version:  {}", doc.manifest.asl_version);
            println!("Digest:       {}", doc.digest);
            println!("Entrypoint:   {}", doc.manifest.interface.entrypoint);
            println!(
                "Capabilities: FS Confined Roots={:?}, Domains={:?}",
                doc.manifest.capabilities.fs.confined_read_roots,
                doc.manifest.capabilities.net.allow_domains
            );

            if let Some(ref sig) = doc.manifest.signature {
                if let Some(ref pubkey) = doc.manifest.signer_pubkey {
                    let valid = asl_security::crypto::verify_signature(pubkey, &doc.digest, sig)
                        .unwrap_or(false);
                    if valid {
                        println!("Signature:    ✅ Valid (Ed25519)");
                        println!("Signer:       {}", pubkey);
                    } else {
                        eprintln!("Signature:    ❌ INVALID (Ed25519)");
                        anyhow::bail!(
                            "Digital signature of ASL file is invalid or corrupted."
                        );
                    }
                } else {
                    println!("Signature:    ⚠️ Present, but public key missing in manifest");
                }
            } else {
                println!("Signature:    ⚠️ Unsigned");
            }

            // Zero-Touch Hook: synchronize and report shadow projection status
            match asl_parser::project_shadow_markdown(&skill_file, &doc) {
                Ok(asl_parser::ShadowProjectResult::Created(p)) => {
                    println!("Shadow Projection: ⚡ Created at {:?}", p);
                }
                Ok(asl_parser::ShadowProjectResult::Updated(p)) => {
                    println!("Shadow Projection: ⚡ Updated at {:?}", p);
                }
                Ok(asl_parser::ShadowProjectResult::CollisionProtected(p)) => {
                    println!("Shadow Projection: ⚠️ Conflict protected at {:?}", p);
                }
                Ok(asl_parser::ShadowProjectResult::Unchanged(_)) => {
                    println!("Shadow Projection: ✅ Synchronized");
                }
                Ok(asl_parser::ShadowProjectResult::Skipped(_)) => {}
                Err(e) => {
                    eprintln!("Shadow Projection: ⚠️ Projection failed: {}", e);
                }
            }

            if doc.rules_code.is_some() {
                println!("Semantic Rules:    ✅ Transpiled in-memory (Strict Starlark L1)");
            }
        }

        Commands::Serve {
            path,
            transport,
            host,
            port,
        } => {
            server_cmds::handle_serve(&path, &transport, &host, port, &parser, &engine)?;
        }

        Commands::CompileGrammar { skill_file, format } => {
            let content = fs::read_to_string(&skill_file)
                .with_context(|| format!("Failed to read file: {:?}", skill_file))?;

            let doc = parser.parse(&content)?;

            let grammar = match format.to_lowercase().as_str() {
                "gbnf" => grammar_compiler.compile_to_gbnf(&doc.manifest.interface.input_schema)?,
                "regex" => {
                    grammar_compiler.compile_to_regex_cfg(&doc.manifest.interface.input_schema)?
                }
                other => anyhow::bail!("Unknown format: {}. Use 'gbnf' or 'regex'", other),
            };

            println!("{}", grammar);
        }

        Commands::Keygen { out } => {
            crypto_cmds::handle_keygen(out)?;
        }

        Commands::Sign { skill_file, key } => {
            crypto_cmds::handle_sign(&skill_file, &key, &parser)?;
        }

        Commands::Verify { skill_file, pubkey } => {
            crypto_cmds::handle_verify(&skill_file, pubkey, &parser)?;
        }

        Commands::AnalyzePrefix { skill_file } => {
            prefix_cmds::handle_analyze_prefix(&skill_file)?;
        }

        Commands::OptimizePrefix {
            skill_file,
            in_place,
        } => {
            prefix_cmds::handle_optimize_prefix(&skill_file, in_place)?;
        }

        Commands::SyncShadows { path } => {
            shadow_cmds::handle_sync_shadows(&path, &parser)?;
        }

        Commands::Watch { path, interval } => {
            shadow_cmds::handle_watch_shadows(&path, &parser, interval)?;
        }

        Commands::Daemon { action } => match action {
            daemon_cmds::DaemonAction::Start { watch_dir, detach } => {
                if detach {
                    let exe = std::env::current_exe()?;
                    daemon_cmds::spawn_detached_daemon(&exe, watch_dir.as_deref())?;
                } else {
                    daemon_cmds::handle_daemon_start(watch_dir.as_deref())?;
                }
            }
            daemon_cmds::DaemonAction::Install { watch_dir } => {
                daemon_cmds::handle_daemon_install(watch_dir.as_deref())?;
            }
            daemon_cmds::DaemonAction::Stop => {
                daemon_cmds::handle_daemon_stop()?;
            }
            daemon_cmds::DaemonAction::Status => {
                daemon_cmds::handle_daemon_status()?;
            }
            daemon_cmds::DaemonAction::Uninstall => {
                daemon_cmds::handle_daemon_uninstall()?;
            }
        },

        Commands::Setup => {
            daemon_cmds::handle_setup()?;
        }

        Commands::Update { check, force } => {
            lifecycle_cmds::handle_update(check, force)?;
        }

        Commands::Uninstall { yes, purge } => {
            lifecycle_cmds::handle_uninstall(yes, purge)?;
        }

        Commands::Expand { skill_file } => {
            let content = fs::read_to_string(&skill_file)
                .with_context(|| format!("Failed to read file: {:?}", skill_file))?;

            let doc = parser
                .parse(&content)
                .with_context(|| "Error parsing ASL file")?;

            if let Some(rules) = &doc.rules_code {
                println!("# --- ORIGINAL SEMANTIC RULES (asl:rules) ---");
                println!("{}\n", rules.trim());
                println!("# --- GENERATED STARLARK L1 DETERMINISTIC CODE (JIT IN-MEMORY) ---");
                println!("{}", doc.deterministic_code);
            } else {
                println!("# --- DETERMINISTIC STARLARK CODE (ORIGINAL) ---");
                println!("{}", doc.deterministic_code);
            }
        }
    }

    Ok(())
}
