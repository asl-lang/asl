use anyhow::{Context, Result};
use asl_core_traits::{EnginePort, GrammarCompilerPort, ParserPort};
use asl_parser::{CommonMarkYamlParser, GbnfGrammarCompiler};
use asl_protocol_mcp::McpServer;
use asl_security::ConfinedSecurityContext;
use asl_spec::SkillDocument;
use asl_vm_starlark::StarlarkEngine;
use clap::{Parser, Subcommand};
use serde_json::Value;
use std::fs;
use std::path::{Path, PathBuf};

mod crypto_cmds;
mod prefix_cmds;
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
    /// Executa uma função determinística de um arquivo .skill
    Run {
        /// Caminho para o arquivo .skill
        skill_file: PathBuf,

        /// Nome da função de entrada a executar (opcional se definida no manifesto)
        #[arg(short, long)]
        entrypoint: Option<String>,

        /// Argumentos de entrada em formato JSON
        #[arg(short, long, default_value = "{}")]
        input: String,
    },

    /// Valida e audita a integridade de um arquivo .skill
    Check {
        /// Caminho para o arquivo .skill
        skill_file: PathBuf,
    },

    /// Inicia um servidor Model Context Protocol (MCP) sobre stdio ou HTTP/SSE
    Serve {
        /// Caminhos para arquivos .skill ou diretório contendo .skill
        #[arg(default_value = ".")]
        path: PathBuf,

        /// Transporte de comunicação (stdio ou http)
        #[arg(short, long, default_value = "stdio")]
        transport: String,

        /// Porta para o servidor HTTP (utilizado apenas quando --transport http)
        #[arg(short, long, default_value = "8080")]
        port: u16,
    },

    /// Compila o esquema JSON do .skill em gramáticas de amostragem para LLMs
    CompileGrammar {
        /// Caminho para o arquivo .skill
        skill_file: PathBuf,

        /// Formato da gramática (gbnf ou regex)
        #[arg(short, long, default_value = "gbnf")]
        format: String,
    },

    /// Gera um par de chaves criptográficas Ed25519 para assinatura de skills
    Keygen {
        /// Prefixo ou caminho base para salvar os arquivos .priv e .pub (opcional)
        #[arg(short, long)]
        out: Option<PathBuf>,
    },

    /// Assina digitalmente um arquivo .skill usando Ed25519
    Sign {
        /// Caminho para o arquivo .skill
        skill_file: PathBuf,

        /// Chave privada Ed25519 (hexadecimal direto ou caminho para arquivo de chave)
        #[arg(short, long)]
        key: String,
    },

    /// Verifica a assinatura criptográfica Ed25519 de um arquivo .skill
    Verify {
        /// Caminho para o arquivo .skill
        skill_file: PathBuf,

        /// Chave pública Ed25519 (opcional se contida no manifesto do arquivo)
        #[arg(short, long)]
        pubkey: Option<String>,
    },

    /// Analisa o prefixo estático e projeta a taxa de acerto do KV-Cache (Axioma 6)
    AnalyzePrefix {
        /// Caminho para o arquivo .skill
        skill_file: PathBuf,
    },

    /// Otimiza o prompt semântico consolidando blocos estáticos no topo do arquivo
    OptimizePrefix {
        /// Caminho para o arquivo .skill
        skill_file: PathBuf,

        /// Sobrescreve o arquivo diretamente com a versão otimizada
        #[arg(short, long)]
        in_place: bool,
    },

    /// Sincroniza todas as projeções sombra de Markdown (.md) para arquivos .skill
    SyncShadows {
        /// Caminho para arquivo ou diretório base (padrão: '.')
        #[arg(default_value = ".")]
        path: PathBuf,
    },

    /// Monitora o diretório e projeta as sombras em tempo real
    Watch {
        /// Caminho para o diretório a ser monitorado (padrão: '.')
        #[arg(default_value = ".")]
        path: PathBuf,

        /// Intervalo de sondagem em milissegundos (padrão: 500ms)
        #[arg(short, long, default_value = "500")]
        interval: u64,
    },

    /// Inspeciona o código Starlark transpilado em memória a partir de regras declarativas
    Expand {
        /// Caminho para o arquivo .skill
        skill_file: PathBuf,
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
        } => {
            let content = fs::read_to_string(&skill_file)
                .with_context(|| format!("Falha ao ler arquivo: {:?}", skill_file))?;

            let doc = parser
                .parse(&content)
                .with_context(|| "Erro ao analisar o arquivo .skill")?;

            // Hook de Toque Zero: projeta ou atualiza sombra Markdown
            let _ = asl_parser::project_shadow_markdown(&skill_file, &doc);

            let ep = entrypoint
                .or_else(|| Some(doc.manifest.interface.entrypoint.clone()))
                .filter(|s| !s.is_empty())
                .unwrap_or_else(|| "run".to_string());

            let input_val: Value = serde_json::from_str(&input)
                .with_context(|| format!("Argumento --input não é um JSON válido: {}", input))?;

            let security = ConfinedSecurityContext::from_capabilities(
                &doc.manifest.capabilities,
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
                .with_context(|| "Falha na execução determinística do ASL")?;

            let output_str = serde_json::to_string_pretty(&result.output)?;
            println!("{}", output_str);
        }

        Commands::Check { skill_file } => {
            let content = fs::read_to_string(&skill_file)
                .with_context(|| format!("Falha ao ler arquivo: {:?}", skill_file))?;

            let doc = parser
                .parse(&content)
                .with_context(|| "Validação falhou: erro ao analisar .skill")?;

            println!("✅ Arquivo .skill validado com sucesso!");
            println!("Nome:        {}", doc.manifest.name);
            println!("Versão ASL:  {}", doc.manifest.asl_version);
            println!("Digest:      {}", doc.digest);
            println!("Entrypoint:  {}", doc.manifest.interface.entrypoint);
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
                        println!("Assinatura:  ✅ Válida (Ed25519)");
                        println!("Signatário:  {}", pubkey);
                    } else {
                        eprintln!("Assinatura:  ❌ INVÁLIDA (Ed25519)");
                        anyhow::bail!("Assinatura digital do arquivo .skill é inválida ou foi corrompida.");
                    }
                } else {
                    println!("Assinatura:  ⚠️ Presente, mas chave pública ausente no manifesto");
                }
            } else {
                println!("Assinatura:  ⚠️ Não assinado");
            }

            // Hook de Toque Zero: sincroniza e relata status da projeção sombra
            match asl_parser::project_shadow_markdown(&skill_file, &doc) {
                Ok(asl_parser::ShadowProjectResult::Created(p)) => {
                    println!("Projeção Sombra: ⚡ Criada em {:?}", p);
                }
                Ok(asl_parser::ShadowProjectResult::Updated(p)) => {
                    println!("Projeção Sombra: ⚡ Atualizada em {:?}", p);
                }
                Ok(asl_parser::ShadowProjectResult::CollisionProtected(p)) => {
                    println!("Projeção Sombra: ⚠️ Conflito protegido em {:?}", p);
                }
                Ok(asl_parser::ShadowProjectResult::Unchanged(_)) => {
                    println!("Projeção Sombra: ✅ Sincronizada");
                }
                Ok(asl_parser::ShadowProjectResult::Skipped(_)) => {}
                Err(e) => {
                    eprintln!("Projeção Sombra: ⚠️ Falha ao projetar: {}", e);
                }
            }

            if doc.rules_code.is_some() {
                println!("Regras Semânticas: ✅ Transpiladas em memória (Strict Starlark L1)");
            }
        }

        Commands::Serve {
            path,
            transport,
            port,
        } => {
            let mut skills = Vec::new();

            if path.is_file() {
                if let Ok(c) = fs::read_to_string(&path) {
                    if let Ok(doc) = parser.parse(&c) {
                        skills.push(doc);
                    }
                }
            } else if path.is_dir() {
                load_skills_recursive(&path, &parser, &mut skills);
            }

            let mut server_caps = asl_spec::SkillCapabilities::default();
            let root_str = if path.is_dir() {
                path.to_string_lossy().to_string()
            } else {
                path.parent().unwrap_or_else(|| Path::new(".")).to_string_lossy().to_string()
            };
            server_caps.fs.confined_read_roots.push(root_str);
            let security = ConfinedSecurityContext::from_capabilities(&server_caps, 1_000_000);

            if transport.to_lowercase() == "http" {
                eprintln!(
                    "[ASL MCP Server] Iniciado sobre HTTP/SSE em http://0.0.0.0:{} com {} skill(s) carregada(s)",
                    port,
                    skills.len()
                );
                let mcp_server = McpServer::new(skills, &engine, &security);
                let http_server = asl_protocol_http::McpHttpServer::new(mcp_server, port);
                let running = std::sync::Arc::new(std::sync::atomic::AtomicBool::new(true));
                http_server
                    .run(running)
                    .with_context(|| "Erro no servidor HTTP do MCP")?;
            } else {
                eprintln!(
                    "[ASL MCP Server] Iniciado sobre stdio com {} skill(s) carregada(s)",
                    skills.len()
                );
                let server = McpServer::new(skills, &engine, &security);
                let stdin = std::io::stdin();
                let stdout = std::io::stdout();

                server
                    .run_stdio_loop(stdin.lock(), stdout.lock())
                    .with_context(|| "Erro no loop de mensagens stdio do MCP")?;
            }
        }

        Commands::CompileGrammar { skill_file, format } => {
            let content = fs::read_to_string(&skill_file)
                .with_context(|| format!("Falha ao ler arquivo: {:?}", skill_file))?;

            let doc = parser.parse(&content)?;

            let grammar = match format.to_lowercase().as_str() {
                "gbnf" => grammar_compiler.compile_to_gbnf(&doc.manifest.interface.input_schema)?,
                "regex" => {
                    grammar_compiler.compile_to_regex_cfg(&doc.manifest.interface.input_schema)?
                }
                other => anyhow::bail!("Formato desconhecido: {}. Use 'gbnf' ou 'regex'", other),
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

        Commands::Expand { skill_file } => {
            let content = fs::read_to_string(&skill_file)
                .with_context(|| format!("Falha ao ler arquivo: {:?}", skill_file))?;

            let doc = parser
                .parse(&content)
                .with_context(|| "Erro ao analisar o arquivo .skill")?;

            if let Some(rules) = &doc.rules_code {
                println!("# --- REGRAS SEMÂNTICAS ORIGINAIS (asl:rules) ---");
                println!("{}\n", rules.trim());
                println!("# --- CÓDIGO DETERMINÍSTICO STARLARK L1 GERADO (JIT IN-MEMORY) ---");
                println!("{}", doc.deterministic_code);
            } else {
                println!("# --- CÓDIGO DETERMINÍSTICO STARLARK (ORIGINAL) ---");
                println!("{}", doc.deterministic_code);
            }
        }
    }

    Ok(())
}

fn load_skills_recursive(dir: &Path, parser: &CommonMarkYamlParser, acc: &mut Vec<SkillDocument>) {
    if let Ok(entries) = fs::read_dir(dir) {
        for entry in entries.flatten() {
            let p = entry.path();
            if p.is_dir() {
                load_skills_recursive(&p, parser, acc);
            } else if p.extension().and_then(|e| e.to_str()) == Some("skill")
                && !asl_parser::is_ignored_path(&p)
            {
                if let Ok(content) = fs::read_to_string(&p) {
                    if let Ok(doc) = parser.parse(&content) {
                        acc.push(doc);
                    }
                }
            }
        }
    }
}
