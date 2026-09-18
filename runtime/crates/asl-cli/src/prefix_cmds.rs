use anyhow::{Context, Result};
use asl_core_traits::{EnginePort, ParserPort};
use asl_parser::{analyze_semantic_prefix, optimize_semantic_prefix, CommonMarkYamlParser};
use asl_security::ConfinedSecurityContext;
use asl_vm_starlark::StarlarkEngine;
use serde_json::Value;
use std::fs;
use std::path::{Path, PathBuf};

/// Handles the `asl analyze-prefix` command
pub fn handle_analyze_prefix(skill_file: &Path) -> Result<()> {
    let content = fs::read_to_string(skill_file)
        .with_context(|| format!("Failed to read file: {:?}", skill_file))?;

    let parser = CommonMarkYamlParser::new();
    let doc = parser
        .parse(&content)
        .with_context(|| "Failed to parse ASL file for prefix analysis")?;

    let report = analyze_semantic_prefix(&doc.semantic_section);

    println!("📊 KV-Cache & Static Prefix Report (Axiom 6)");
    println!("File:                    {:?}", skill_file);
    println!("Skill Name:              {}", doc.manifest.name);
    println!("Semantic Chars:          {}", report.total_semantic_chars);
    println!(
        "Static Prefix:           {} chars (~{} tokens)",
        report.static_prefix_chars, report.static_estimated_tokens
    );
    println!(
        "Projected Hit Rate:      {:.1}%",
        report.projected_cache_hit_rate_pct
    );

    if report.dynamic_variables.is_empty() {
        println!("Dynamic Variables:       None (100% Invariant)");
    } else {
        println!(
            "Dynamic Variables:       {}",
            report.dynamic_variables.join(", ")
        );
    }

    if report.cache_invalidation_hazards.is_empty() {
        println!("Risk Diagnostic:         ✅ KV-Cache Friendly (Zero hazards detected)");
    } else {
        println!("Risk Diagnostic:         ⚠️ Premature Invalidation Hazards Found:");
        for hazard in &report.cache_invalidation_hazards {
            println!("  - {}", hazard);
        }
    }

    Ok(())
}

/// Handles the `asl optimize-prefix` command
pub fn handle_optimize_prefix(skill_file: &Path, in_place: bool) -> Result<()> {
    let content = fs::read_to_string(skill_file)
        .with_context(|| format!("Failed to read file: {:?}", skill_file))?;

    let parser = CommonMarkYamlParser::new();
    let doc = parser
        .parse(&content)
        .with_context(|| "Failed to parse ASL file for prefix optimization")?;

    let optimized_semantic = optimize_semantic_prefix(&doc.semantic_section);

    if in_place {
        let updated_content = replace_semantic_in_skill(&content, &doc.semantic_section, &optimized_semantic);
        fs::write(skill_file, &updated_content)
            .with_context(|| format!("Failed to save optimized file: {:?}", skill_file))?;

        // Zero-Touch Hook: update shadow Markdown immediately
        if let Ok(new_doc) = parser.parse(&updated_content) {
            let _ = asl_parser::project_shadow_markdown(skill_file, &new_doc);
        }

        println!("✅ Static prefix successfully optimized and rewritten at {:?}", skill_file);
    } else {
        println!("{}", optimized_semantic);
    }

    Ok(())
}

/// Handles the `asl run` command with zero-touch shadow projection announcing and `--no-shadow` support
pub fn handle_run(
    skill_file: &Path,
    entrypoint: Option<String>,
    input: &str,
    allowed_root: &[PathBuf],
    no_shadow: bool,
    parser: &CommonMarkYamlParser,
    engine: &StarlarkEngine,
) -> Result<()> {
    let content = fs::read_to_string(skill_file)
        .with_context(|| format!("Failed to read file: {:?}", skill_file))?;

    let doc = parser
        .parse(&content)
        .with_context(|| "Error parsing ASL file")?;

    // Zero-Touch Hook: project or update shadow Markdown unless disabled
    if !no_shadow {
        match asl_parser::project_shadow_markdown(skill_file, &doc) {
            Ok(asl_parser::ShadowProjectResult::Created(p)) => {
                eprintln!("⚡ Shadow projected: created at {:?}", p);
            }
            Ok(asl_parser::ShadowProjectResult::Updated(p)) => {
                eprintln!("⚡ Shadow projected: updated at {:?}", p);
            }
            _ => {}
        }
    }

    let ep = entrypoint
        .or_else(|| Some(doc.manifest.interface.entrypoint.clone()))
        .filter(|s| !s.is_empty())
        .unwrap_or_else(|| "run".to_string());

    let input_val: Value = serde_json::from_str(input)
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
        effective_caps.fs.allow_write.retain(|r| {
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
    Ok(())
}

/// Validates an ASL file and reports manifest, signature, rules and shadow status
pub fn handle_check(
    skill_file: &Path,
    parser: &CommonMarkYamlParser,
    engine: &StarlarkEngine,
    dry_run: bool,
    no_shadow: bool,
) -> Result<()> {
    let content = fs::read_to_string(skill_file)
        .with_context(|| format!("Failed to read file: {:?}", skill_file))?;

    let doc = parser
        .parse(&content)
        .with_context(|| "Validation failed: error parsing ASL file")?;

    println!("✅ ASL file validated successfully!");
    println!("Name:         {}", doc.manifest.name);
    println!("ASL Version:  {}", doc.manifest.asl_version);
    println!("Digest:       {}", doc.digest);
    println!("Entrypoint:   {}", doc.manifest.interface.entrypoint);

    let fs_read = if doc.manifest.capabilities.fs.confined_read_roots.is_empty() {
        "None (sandboxed)".to_string()
    } else {
        format!("{:?}", doc.manifest.capabilities.fs.confined_read_roots)
    };
    let fs_write = if doc.manifest.capabilities.fs.allow_write.is_empty() {
        "None (read-only)".to_string()
    } else {
        format!("{:?}", doc.manifest.capabilities.fs.allow_write)
    };
    let domains = if doc.manifest.capabilities.net.allow_domains.is_empty() {
        "None (air-gapped)".to_string()
    } else {
        format!("{:?}", doc.manifest.capabilities.net.allow_domains)
    };
    let env_keys = if doc.manifest.capabilities.env.allow_keys.is_empty() {
        "None".to_string()
    } else {
        format!("{:?}", doc.manifest.capabilities.env.allow_keys)
    };

    println!("Capabilities: FS Read: {}", fs_read);
    println!("              FS Write: {}", fs_write);
    println!("              Domains: {}", domains);
    println!("              Env Keys: {}", env_keys);
    println!("Fuel Limit:   {} opcodes", doc.manifest.limits.max_fuel_opcodes);

    if let Some(ref sig) = doc.manifest.signature {
        if let Some(ref pubkey) = doc.manifest.signer_pubkey {
            let valid = asl_security::crypto::verify_signature(pubkey, &doc.digest, sig)
                .unwrap_or(false);
            if valid {
                println!("Signature:    ✅ Valid (Ed25519)");
                println!("Signer:       {}", pubkey);
            } else {
                eprintln!("Signature:    ❌ INVALID (Ed25519)");
                anyhow::bail!("Digital signature of ASL file is invalid or corrupted.");
            }
        } else {
            println!("Signature:    ⚠️ Present, but public key missing in manifest");
        }
    } else {
        println!("Signature:    ⚠️ Unsigned");
    }

    if no_shadow {
        println!("Shadow Projection: ⏩ Skipped (--no-shadow)");
    } else {
        match asl_parser::project_shadow_markdown(skill_file, &doc) {
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
    }

    if doc.rules_code.is_some() {
        println!("Semantic Rules:    ✅ Transpiled in-memory (ASL VM)");
    }

    let ep = &doc.manifest.interface.entrypoint;
    let mock_sec = ConfinedSecurityContext::from_capabilities(
        &doc.manifest.capabilities,
        doc.manifest.limits.max_fuel_opcodes,
    );

    let test_input = serde_json::json!({});
    let compile_check = engine.execute(
        &doc.deterministic_code,
        ep,
        &test_input,
        &mock_sec,
        &doc.manifest.limits,
    );

    match compile_check {
        Ok(res) => {
            println!("Compilation:  ✅ Entrypoint '{}' compiled successfully", ep);
            if dry_run {
                println!("Dry Run:      ✅ Executed with empty input -> {:?}", res.output);
            }
        }
        Err(e) => {
            if dry_run {
                eprintln!("Dry Run:      ❌ Execution failed: {}", e);
                anyhow::bail!("Dry run execution failed: {}", e);
            } else {
                let err_str = e.to_string();
                let is_input_shape_error = err_str.contains("Key not found")
                    || err_str.contains("Index out of bounds")
                    || err_str.contains("Ocap permission error")
                    || err_str.contains("key not found");
                if is_input_shape_error {
                    println!(
                        "Compilation:  ✅ Entrypoint '{}' syntax valid (runtime requires specific input schema)",
                        ep
                    );
                } else {
                    eprintln!("Compilation:  ❌ Failed: {}", e);
                    anyhow::bail!("Skill failed compilation: {}", e);
                }
            }
        }
    }

    Ok(())
}

/// Expands declarative rules or procedural ASL code into full ASL VM representation
pub fn handle_expand(skill_file: &Path, parser: &CommonMarkYamlParser) -> Result<()> {
    let content = fs::read_to_string(skill_file)
        .with_context(|| format!("Failed to read file: {:?}", skill_file))?;

    let doc = parser
        .parse(&content)
        .with_context(|| "Error parsing ASL file")?;

    if let Some(rules) = &doc.rules_code {
        println!("# --- ORIGINAL DECLARATIVE ASL RULES (```asl) ---");
        println!("{}\n", rules.trim());
        println!("# --- UNDER THE HOOD: COMPILED TO HERMETIC STARLARK (IN-MEMORY AOT) ---");
        println!("{}", doc.deterministic_code);
    } else {
        println!("# --- UNDER THE HOOD: ASL COMPILED TO HERMETIC STARLARK ---");
        println!("{}", doc.deterministic_code);
    }
    Ok(())
}

fn replace_semantic_in_skill(full_content: &str, old_semantic: &str, new_semantic: &str) -> String {
    if old_semantic.is_empty() {
        return full_content.to_string();
    }

    // Finds the end of the frontmatter delimited by the second '---'
    let mut dashes_count = 0;
    let mut split_idx = None;

    for (idx, _) in full_content.match_indices("---") {
        let line_start = full_content[..idx].rfind('\n').map(|p| p + 1).unwrap_or(0);
        let line_end = full_content[idx..].find('\n').map(|p| idx + p).unwrap_or(full_content.len());
        let trimmed = full_content[line_start..line_end].trim();
        if trimmed == "---" {
            dashes_count += 1;
            if dashes_count == 2 {
                split_idx = Some(line_end);
                break;
            }
        }
    }

    if let Some(body_start) = split_idx {
        let frontmatter = &full_content[..body_start];
        let body = &full_content[body_start..];
        let new_body = body.replacen(old_semantic, new_semantic, 1);
        format!("{}{}", frontmatter, new_body)
    } else {
        full_content.replacen(old_semantic, new_semantic, 1)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_replace_semantic_in_skill() {
        let original = "---\nname: test\ndescription: Old text\n---\nOld text\n\n```asl\npass\n```";
        let replaced = replace_semantic_in_skill(original, "Old text", "New optimized text");
        // Frontmatter must remain untouched
        assert!(replaced.contains("description: Old text"));
        // Body must be replaced
        assert!(replaced.contains("---\nNew optimized text"));
        assert!(replaced.contains("```asl\npass\n```"));
    }
}
