use anyhow::{Context, Result};
use asl_core_traits::ParserPort;
use asl_parser::{analyze_semantic_prefix, optimize_semantic_prefix, CommonMarkYamlParser};
use std::fs;
use std::path::Path;

/// Manipula o comando `asl analyze-prefix`
pub fn handle_analyze_prefix(skill_file: &Path) -> Result<()> {
    let content = fs::read_to_string(skill_file)
        .with_context(|| format!("Falha ao ler arquivo: {:?}", skill_file))?;

    let parser = CommonMarkYamlParser::new();
    let doc = parser
        .parse(&content)
        .with_context(|| "Erro ao analisar arquivo .skill para análise de prefixo")?;

    let report = analyze_semantic_prefix(&doc.semantic_section);

    println!("📊 Relatório de KV-Cache & Prefixo Estático (Axioma 6)");
    println!("Arquivo:                 {:?}", skill_file);
    println!("Nome da Skill:           {}", doc.manifest.name);
    println!("Caracteres Semânticos:   {}", report.total_semantic_chars);
    println!(
        "Prefixo Estático:        {} chars (~{} tokens)",
        report.static_prefix_chars, report.static_estimated_tokens
    );
    println!(
        "Hit-Rate Projetado:      {:.1}%",
        report.projected_cache_hit_rate_pct
    );

    if report.dynamic_variables.is_empty() {
        println!("Variáveis Dinâmicas:     Nenhuma (100% Invariante)");
    } else {
        println!(
            "Variáveis Dinâmicas:     {}",
            report.dynamic_variables.join(", ")
        );
    }

    if report.cache_invalidation_hazards.is_empty() {
        println!("Diagnóstico de Risco:    ✅ KV-Cache Amigável (Zero riscos detectados)");
    } else {
        println!("Diagnóstico de Risco:    ⚠️ Riscos de Invalidação Prematura Encontrados:");
        for hazard in &report.cache_invalidation_hazards {
            println!("  - {}", hazard);
        }
    }

    Ok(())
}

/// Manipula o comando `asl optimize-prefix`
pub fn handle_optimize_prefix(skill_file: &Path, in_place: bool) -> Result<()> {
    let content = fs::read_to_string(skill_file)
        .with_context(|| format!("Falha ao ler arquivo: {:?}", skill_file))?;

    let parser = CommonMarkYamlParser::new();
    let doc = parser
        .parse(&content)
        .with_context(|| "Erro ao analisar arquivo .skill para otimização de prefixo")?;

    let optimized_semantic = optimize_semantic_prefix(&doc.semantic_section);

    if in_place {
        let updated_content = replace_semantic_in_skill(&content, &doc.semantic_section, &optimized_semantic);
        fs::write(skill_file, &updated_content)
            .with_context(|| format!("Falha ao salvar arquivo otimizado: {:?}", skill_file))?;

        // Hook de Toque Zero: atualiza a sombra Markdown imediatamente
        if let Ok(new_doc) = parser.parse(&updated_content) {
            let _ = asl_parser::project_shadow_markdown(skill_file, &new_doc);
        }

        println!("✅ Prefixo estático otimizado e reescrito com sucesso em {:?}", skill_file);
    } else {
        println!("{}", optimized_semantic);
    }

    Ok(())
}

fn replace_semantic_in_skill(full_content: &str, old_semantic: &str, new_semantic: &str) -> String {
    if old_semantic.is_empty() {
        return full_content.to_string();
    }

    // Encontra o término do frontmatter delimitado pelo segundo '---'
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
