use anyhow::{Context, Result};
use asl_core_traits::ParserPort;
use asl_parser::{analyze_semantic_prefix, optimize_semantic_prefix, CommonMarkYamlParser};
use std::fs;
use std::path::Path;

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

/// Validates an ASL file and reports manifest, signature, rules and shadow status
pub fn handle_check(skill_file: &Path, parser: &CommonMarkYamlParser) -> Result<()> {
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
                anyhow::bail!("Digital signature of ASL file is invalid or corrupted.");
            }
        } else {
            println!("Signature:    ⚠️ Present, but public key missing in manifest");
        }
    } else {
        println!("Signature:    ⚠️ Unsigned");
    }

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

    if doc.rules_code.is_some() {
        println!("Semantic Rules:    ✅ Transpiled in-memory (Strict Starlark L1)");
    }
    Ok(())
}

/// Expands declarative rules or deterministic code into full Starlark representation
pub fn handle_expand(skill_file: &Path, parser: &CommonMarkYamlParser) -> Result<()> {
    let content = fs::read_to_string(skill_file)
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
        // Frontmatter deve permanecer intocado
        assert!(replaced.contains("description: Old text"));
        // Corpo deve ser substituído
        assert!(replaced.contains("---\nNew optimized text"));
        assert!(replaced.contains("```asl\npass\n```"));
    }
}
