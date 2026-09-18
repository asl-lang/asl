use serde::{Deserialize, Serialize};

/// Relatório quantitativo da análise estática de prefixo e KV-cache
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct PrefixAnalysisReport {
    /// Total de caracteres na seção semântica
    pub total_semantic_chars: usize,
    /// Comprimento contíguo do prefixo estático invariante inicial
    pub static_prefix_chars: usize,
    /// Estimativa de tokens totais (~4 caracteres por token)
    pub total_estimated_tokens: usize,
    /// Estimativa de tokens no prefixo estático (~4 caracteres por token)
    pub static_estimated_tokens: usize,
    /// Taxa de acerto projetada de KV-cache (0.0% a 100.0%)
    pub projected_cache_hit_rate_pct: f64,
    /// Variáveis dinâmicas detectadas (ex.: {{input}}, ${session})
    pub dynamic_variables: Vec<String>,
    /// Riscos de invalidação prematura de cache
    pub cache_invalidation_hazards: Vec<String>,
}

/// Analisa a seção semântica para mensurar invariância e potencial de reuso do KV-cache
pub fn analyze_semantic_prefix(semantic_text: &str) -> PrefixAnalysisReport {
    let trimmed = semantic_text.trim();
    let chars: Vec<char> = trimmed.chars().collect();
    let total_semantic_chars = chars.len();
    let total_estimated_tokens = total_semantic_chars.div_ceil(4);

    if total_semantic_chars == 0 {
        return PrefixAnalysisReport {
            total_semantic_chars: 0,
            static_prefix_chars: 0,
            total_estimated_tokens: 0,
            static_estimated_tokens: 0,
            projected_cache_hit_rate_pct: 100.0,
            dynamic_variables: Vec::new(),
            cache_invalidation_hazards: Vec::new(),
        };
    }

    let mut dynamic_variables = Vec::new();
    let mut first_dynamic_idx: Option<usize> = None;
    let mut hazards = Vec::new();

    // Varredura por marcadores dinâmicos: {{var}}, ${var}, ou {var}
    let n = chars.len();
    let mut i = 0;

    while i < n {
        if i + 1 < n && chars[i] == '{' && chars[i + 1] == '{' {
            let start = i;
            let mut end = start + 2;
            while end + 1 < n && !(chars[end] == '}' && chars[end + 1] == '}') {
                end += 1;
            }
            if end + 1 < n && chars[end] == '}' && chars[end + 1] == '}' {
                let var_name: String = chars[start..=end + 1].iter().collect();
                if first_dynamic_idx.is_none() {
                    first_dynamic_idx = Some(start);
                }
                dynamic_variables.push(var_name);
                i = end + 2;
                continue;
            }
        } else if i + 1 < n && chars[i] == '$' && chars[i + 1] == '{' {
            let start = i;
            let mut end = start + 2;
            while end < n && chars[end] != '}' {
                end += 1;
            }
            if end < n && chars[end] == '}' {
                let var_name: String = chars[start..=end].iter().collect();
                if first_dynamic_idx.is_none() {
                    first_dynamic_idx = Some(start);
                }
                dynamic_variables.push(var_name);
                i = end + 1;
                continue;
            }
        } else if chars[i] == '{' {
            // Check if it is a simple identifier placeholder {identifier} and not JSON
            let start = i;
            let mut end = start + 1;
            let mut is_valid_ident = true;
            while end < n && chars[end] != '}' {
                let c = chars[end];
                if !(c.is_alphanumeric() || c == '_' || c == '.' || c == '-') {
                    is_valid_ident = false;
                    break;
                }
                end += 1;
            }
            if is_valid_ident && end < n && chars[end] == '}' && end > start + 1 {
                let var_name: String = chars[start..=end].iter().collect();
                if first_dynamic_idx.is_none() {
                    first_dynamic_idx = Some(start);
                }
                dynamic_variables.push(var_name);
                i = end + 1;
                continue;
            }
        }
        i += 1;
    }

    let static_prefix_chars = first_dynamic_idx.unwrap_or(total_semantic_chars);
    let static_estimated_tokens = static_prefix_chars.div_ceil(4);
    let projected_cache_hit_rate_pct = if total_semantic_chars > 0 {
        ((static_prefix_chars as f64 / total_semantic_chars as f64) * 1000.0).round() / 10.0
    } else {
        100.0
    };

    if let Some(first_idx) = first_dynamic_idx {
        if first_idx < 150 || (first_idx as f64 / total_semantic_chars as f64) < 0.3 {
            hazards.push(format!(
                "Critical Invalidation Hazard: Dynamic variable found in the first third of the prompt (offset {} of {} characters).",
                first_idx, total_semantic_chars
            ));
        }
    }

    PrefixAnalysisReport {
        total_semantic_chars,
        static_prefix_chars,
        total_estimated_tokens,
        static_estimated_tokens,
        projected_cache_hit_rate_pct,
        dynamic_variables,
        cache_invalidation_hazards: hazards,
    }
}

/// Optimizes semantic section by reordering purely static blocks to the top
/// and consolidating dynamic variables at the suffix, maximizing KV-Cache hit rate.
pub fn optimize_semantic_prefix(semantic_text: &str) -> String {
    let trimmed = semantic_text.trim();
    if trimmed.is_empty() {
        return String::new();
    }

    let blocks: Vec<&str> = trimmed.split("\n\n").collect();
    let mut static_blocks = Vec::new();
    let mut dynamic_blocks = Vec::new();

    for block in blocks {
        let trimmed_block = block.trim();
        if trimmed_block.is_empty() {
            continue;
        }

        let report = analyze_semantic_prefix(trimmed_block);
        if report.dynamic_variables.is_empty() {
            static_blocks.push(trimmed_block);
        } else {
            dynamic_blocks.push(trimmed_block);
        }
    }

    // If already 100% static or nothing to reorder
    if dynamic_blocks.is_empty() || static_blocks.is_empty() {
        return trimmed.to_string();
    }

    let mut result = String::new();
    result.push_str(&static_blocks.join("\n\n"));
    result.push_str("\n\n## Context and Dynamic Parameters\n\n");
    result.push_str(&dynamic_blocks.join("\n\n"));

    result
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_analyze_semantic_prefix_pure_static() {
        let text = "# Commit Governance\nYou are an engineering assistant.";
        let report = analyze_semantic_prefix(text);
        assert_eq!(report.static_prefix_chars, text.len());
        assert_eq!(report.projected_cache_hit_rate_pct, 100.0);
        assert!(report.dynamic_variables.is_empty());
        assert!(report.cache_invalidation_hazards.is_empty());
    }

    #[test]
    fn test_analyze_semantic_prefix_dynamic_hazard() {
        let text = "Hello {{user_name}}!\n\nHere are permanent rules:\n1. Do not break tests.";
        let report = analyze_semantic_prefix(text);
        assert_eq!(report.dynamic_variables, vec!["{{user_name}}"]);
        assert!(report.projected_cache_hit_rate_pct < 50.0);
        assert!(!report.cache_invalidation_hazards.is_empty());
    }

    #[test]
    fn test_optimize_semantic_prefix_reordering() {
        let bad_prompt = "Dynamic instruction: {{user_query}}\n\n## System Rules\nAlways follow security standards.";
        let optimized = optimize_semantic_prefix(bad_prompt);

        assert!(optimized.starts_with("## System Rules"));
        assert!(optimized.contains("## Context and Dynamic Parameters"));
        assert!(optimized.ends_with("Dynamic instruction: {{user_query}}"));

        let report = analyze_semantic_prefix(&optimized);
        assert!(report.projected_cache_hit_rate_pct > 40.0);
    }

    #[test]
    fn test_unicode_chars_count_consistency() {
        let text = "Olá mundo! Ação de teste número 1.";
        let report = analyze_semantic_prefix(text);
        assert_eq!(report.total_semantic_chars, text.chars().count());
        assert_eq!(report.static_prefix_chars, text.chars().count());
        assert_eq!(report.projected_cache_hit_rate_pct, 100.0);
    }
}
