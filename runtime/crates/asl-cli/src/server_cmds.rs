use anyhow::{Context, Result};
use asl_core_traits::ParserPort;
use asl_parser::CommonMarkYamlParser;
use asl_protocol_mcp::McpServer;
use asl_security::ConfinedSecurityContext;
use asl_spec::SkillDocument;
use asl_vm_starlark::StarlarkEngine;
use std::fs;
use std::path::Path;

#[allow(clippy::too_many_arguments)]
pub fn handle_serve(
    path: &Path,
    transport: &str,
    host: &str,
    port: u16,
    allowed_root: &[std::path::PathBuf],
    allow_domain: &[String],
    allow_env: &[String],
    policy_path: Option<&Path>,
    permissive: bool,
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

    let mut base_policy = if permissive {
        asl_spec::HostSecurityPolicy::permissive()
    } else if let Some(p) = policy_path {
        let p_content = fs::read_to_string(p)
            .with_context(|| format!("Failed to read policy file: {:?}", p))?;
        serde_json::from_str(&p_content)
            .with_context(|| format!("Failed to parse policy file as JSON: {:?}", p))?
    } else {
        asl_spec::HostSecurityPolicy::default()
    };

    for r in allowed_root {
        base_policy = base_policy.with_allowed_fs_read_root(r.to_string_lossy().to_string());
        base_policy = base_policy.with_allowed_fs_write_root(r.to_string_lossy().to_string());
    }
    for d in allow_domain {
        base_policy = base_policy.with_allowed_domain(d);
    }
    for e in allow_env {
        base_policy = base_policy.with_allowed_env_key(e);
    }

    let fallback_security = ConfinedSecurityContext::from_capabilities(&asl_spec::SkillCapabilities::default(), 1_000_000);

    let policy_for_factory = base_policy;
    let mcp_server = McpServer::with_factory(skills.clone(), engine, &fallback_security, move |skill| {
        let effective_caps = policy_for_factory.intersect(&skill.manifest.capabilities)?;
        let effective_limits = policy_for_factory.effective_limits(&skill.manifest.limits);
        Ok(Box::new(
            ConfinedSecurityContext::from_capabilities(
                &effective_caps,
                effective_limits.max_fuel_opcodes,
            )
            .with_timeout_ms(effective_limits.wall_clock_timeout_ms),
        ) as Box<dyn asl_core_traits::CapabilityContext>)
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
