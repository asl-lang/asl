use anyhow::{Context, Result};
use asl_core_traits::{GrammarCompilerPort, ParserPort};
use asl_parser::{CommonMarkYamlParser, GbnfGrammarCompiler};
use asl_vm_starlark::StarlarkEngine;
use clap::{Parser, Subcommand};
use std::fs;
use std::path::PathBuf;

mod crypto_cmds;
mod daemon_cmds;
mod docs_cmds;
mod docs_rosetta;
mod docs_ssot;
mod lifecycle_cmds;
mod prefix_cmds;
mod repl_cmds;
mod server_cmds;
mod shadow_cmds;

#[derive(Parser)]
#[command(name = "asl")]
#[command(about = "Agent Skill Language (ASL) Runtime & Tooling", long_about = None)]
#[command(version)]
pub(crate) struct Cli {
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

        /// Allowed network domains for Host Policy
        #[arg(long)]
        allow_domain: Vec<String>,

        /// Allowed environment keys for Host Policy
        #[arg(long)]
        allow_env: Vec<String>,

        /// Path to host security policy file (JSON or YAML)
        #[arg(long)]
        policy: Option<PathBuf>,

        /// Enables permissive host policy for testing/dev (grants all capabilities)
        #[arg(long)]
        permissive: bool,

        /// Skips automatic Markdown shadow projection
        #[arg(long)]
        no_shadow: bool,
    },

    /// Validates and audits the integrity and executability of an ASL file (.skill, .tool, .asl)
    Check {
        /// Path to the ASL file (.skill, .tool, .asl)
        skill_file: PathBuf,

        /// Performs a dry-run execution against mock capability context
        #[arg(long)]
        dry_run: bool,

        /// Skips automatic Markdown shadow projection
        #[arg(long)]
        no_shadow: bool,
    },

    /// Interactive ASL evaluation REPL (Read-Eval-Print Loop)
    Repl {
        /// Optional path to an ASL file (.skill, .tool, .asl) to preload
        #[arg(short, long)]
        skill: Option<PathBuf>,
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

        /// Allowed host roots for intersection with requested capabilities
        #[arg(long)]
        allowed_root: Vec<PathBuf>,

        /// Allowed network domains for Host Policy
        #[arg(long)]
        allow_domain: Vec<String>,

        /// Allowed environment keys for Host Policy
        #[arg(long)]
        allow_env: Vec<String>,

        /// Path to host security policy file (JSON or YAML)
        #[arg(long)]
        policy: Option<PathBuf>,

        /// Enables permissive host policy for testing/dev
        #[arg(long)]
        permissive: bool,
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

    /// Shows language reference, syntax cheat sheets, rules, and AI primers
    #[command(alias = "learn", alias = "syntax", alias = "guide", alias = "cheat")]
    Docs {
        /// Documentation topic (overview, syntax, rules, triad, mcp, examples)
        #[arg(default_value = "overview")]
        topic: String,

        /// Format output as an ultra-compact primer for AI prompt ingestion
        #[arg(long)]
        ai: bool,

        /// Output structured documentation in JSON format
        #[arg(long)]
        json: bool,
    },

    /// Emits canonical, ready-to-use templates for skills, tools, and rules
    Template {
        /// Target template type (skill, tool, rules, asl)
        #[arg(default_value = "skill")]
        target_type: String,
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
            allow_domain,
            allow_env,
            policy,
            permissive,
            no_shadow,
        } => {
            prefix_cmds::handle_run(
                &skill_file,
                entrypoint,
                &input,
                &allowed_root,
                &allow_domain,
                &allow_env,
                policy.as_deref(),
                permissive,
                no_shadow,
                &parser,
                &engine,
            )?;
        }

        Commands::Check {
            skill_file,
            dry_run,
            no_shadow,
        } => {
            prefix_cmds::handle_check(&skill_file, &parser, &engine, dry_run, no_shadow)?;
        }

        Commands::Repl { skill } => {
            repl_cmds::handle_repl(skill.as_deref(), &parser, &engine)?;
        }

        Commands::Serve {
            path,
            transport,
            host,
            port,
            allowed_root,
            allow_domain,
            allow_env,
            policy,
            permissive,
        } => {
            server_cmds::handle_serve(
                &path,
                &transport,
                &host,
                port,
                &allowed_root,
                &allow_domain,
                &allow_env,
                policy.as_deref(),
                permissive,
                &parser,
                &engine,
            )?;
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
            prefix_cmds::handle_expand(&skill_file, &parser)?;
        }

        Commands::Docs { topic, ai, json } => {
            docs_cmds::handle_docs(&topic, ai, json)?;
        }

        Commands::Template { target_type } => {
            docs_cmds::handle_template(&target_type)?;
        }
    }

    Ok(())
}
