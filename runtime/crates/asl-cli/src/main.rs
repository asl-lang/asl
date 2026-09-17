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

            let empty_caps = asl_spec::SkillCapabilities::default();
            let security = ConfinedSecurityContext::from_capabilities(&empty_caps, 1_000_000);

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
    }

    Ok(())
}

fn load_skills_recursive(dir: &Path, parser: &CommonMarkYamlParser, acc: &mut Vec<SkillDocument>) {
    if let Ok(entries) = fs::read_dir(dir) {
        for entry in entries.flatten() {
            let p = entry.path();
            if p.is_dir() {
                load_skills_recursive(&p, parser, acc);
            } else if p.extension().and_then(|e| e.to_str()) == Some("skill") {
                if let Ok(content) = fs::read_to_string(&p) {
                    if let Ok(doc) = parser.parse(&content) {
                        acc.push(doc);
                    }
                }
            }
        }
    }
}
