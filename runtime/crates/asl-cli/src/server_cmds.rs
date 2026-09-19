use anyhow::{Context, Result};
use asl_core_traits::ParserPort;
use asl_parser::CommonMarkYamlParser;
use asl_protocol_mcp::McpServer;
use asl_security::ConfinedSecurityContext;
use asl_spec::SkillDocument;
use asl_vm_starlark::StarlarkEngine;
use std::fs;
use std::path::Path;

pub fn handle_serve(
    path: &Path,
    transport: &str,
    host: &str,
    port: u16,
    parser: &CommonMarkYamlParser,
    engine: &StarlarkEngine,
) -> Result<()> {
    let mut skills = Vec::new();

    if path.is_file() {
        if let Ok(c) = fs::read_to_string(path) {
            if let Ok(doc) = parser.parse(&c) {
                skills.push(doc);
            }
        }
    } else if path.is_dir() {
        load_skills_recursive(path, parser, &mut skills);
    }

    let root_str = if path.is_dir() {
        path.to_string_lossy().to_string()
    } else {
        path.parent()
            .unwrap_or_else(|| Path::new("."))
            .to_string_lossy()
            .to_string()
    };
    let mut server_caps = asl_spec::SkillCapabilities::default();
    server_caps.fs.confined_read_roots.push(root_str.clone());
    let fallback_security = ConfinedSecurityContext::from_capabilities(&server_caps, 1_000_000);

    let root_for_factory = root_str;
    let r_str = root_for_factory.clone();
    let mcp_server = McpServer::with_factory(skills.clone(), engine, &fallback_security, move |skill| {
        let mut caps = skill.manifest.capabilities.clone();
        if !caps.fs.confined_read_roots.contains(&r_str) {
            caps.fs.confined_read_roots.push(r_str.clone());
        }
        Box::new(
            ConfinedSecurityContext::from_capabilities(
                &caps,
                skill.manifest.limits.max_fuel_opcodes,
            )
            .with_timeout_ms(skill.manifest.limits.wall_clock_timeout_ms),
        )
    });

    if transport.to_lowercase() == "http" {
        eprintln!(
            "[ASL MCP Server] Started over HTTP/SSE on http://{}:{} with {} skill(s) loaded",
            host,
            port,
            skills.len()
        );
        let http_server =
            asl_protocol_http::McpHttpServer::with_host(mcp_server, host.to_string(), port);
        let running = std::sync::Arc::new(std::sync::atomic::AtomicBool::new(true));
        http_server
            .run(running)
            .with_context(|| "Error in MCP HTTP server")?;
    } else {
        eprintln!(
            "[ASL MCP Server] Started over stdio with {} skill(s) loaded",
            skills.len()
        );
        let stdin = std::io::stdin();
        let stdout = std::io::stdout();

        mcp_server
            .run_stdio_loop(stdin.lock(), stdout.lock())
            .with_context(|| "Error in MCP stdio message loop")?;
    }

    Ok(())
}

fn load_skills_recursive(dir: &Path, parser: &CommonMarkYamlParser, acc: &mut Vec<SkillDocument>) {
    if let Ok(entries) = fs::read_dir(dir) {
        for entry in entries.flatten() {
            let p = entry.path();
            if p.is_dir() {
                load_skills_recursive(&p, parser, acc);
            } else if asl_spec::is_asl_file(&p) && !asl_parser::is_ignored_path(&p) {
                if let Ok(content) = fs::read_to_string(&p) {
                    if let Ok(doc) = parser.parse(&content) {
                        acc.push(doc);
                    }
                }
            }
        }
    }
}
