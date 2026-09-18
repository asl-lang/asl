//! Gerador de código Strict Starlark L1 e verificador AOT para a linguagem ASL Rules.

use super::ast::*;
use super::lexer::Lexer;
use super::parser::Parser;
use asl_core_traits::{RulesTranspilerPort, SourceMapEntry, TranspilationResult};
use asl_spec::{AslError, Result, SkillManifest};
use starlark::syntax::{AstModule, Dialect};

pub struct RulesTranspiler;

impl RulesTranspiler {
    pub fn new() -> Self {
        Self
    }
}

impl Default for RulesTranspiler {
    fn default() -> Self {
        Self::new()
    }
}

impl RulesTranspilerPort for RulesTranspiler {
    fn transpile(
        &self,
        rules_source: &str,
        manifest: &SkillManifest,
    ) -> Result<TranspilationResult> {
        let lexer = Lexer::new(rules_source);
        let tokens = lexer
            .tokenize()
            .map_err(|e| AslError::RulesTranspileError(format!("Erro léxico: {}", e)))?;
        let mut parser = Parser::new(tokens);
        let block = parser
            .parse_rules_block()
            .map_err(|e| AslError::RulesTranspileError(format!("Erro sintático: {}", e)))?;

        let mut lines = Vec::new();
        let mut source_map = Vec::new();
        let mut static_invariants = Vec::new();

        // 1. Preâmbulo Universal Hermético L1
        lines.push(
            "# --- PREÂMBULO STRICT STARLARK L1 GERADO PELO ASL RULES TRANSPILER ---".to_string(),
        );
        lines.push("def _asl_get(obj, path, default=None):".to_string());
        lines.push("    curr = obj".to_string());
        lines.push("    for key in path:".to_string());
        lines.push("        if type(curr) != \"dict\" or key not in curr:".to_string());
        lines.push("            return default".to_string());
        lines.push("        curr = curr[key]".to_string());
        lines.push("    return curr if curr != None else default".to_string());
        lines.push("".to_string());
        lines.push("def _asl_contains_any(haystack, needles):".to_string());
        lines.push("    if type(haystack) != \"string\":".to_string());
        lines.push("        return False".to_string());
        lines.push("    for n in needles:".to_string());
        lines.push("        if n in haystack:".to_string());
        lines.push("            return True".to_string());
        lines.push("    return False".to_string());
        lines.push("".to_string());
        lines.push("def _asl_starts_with_any(haystack, prefixes):".to_string());
        lines.push("    if type(haystack) != \"string\":".to_string());
        lines.push("        return None".to_string());
        lines.push("    for p in prefixes:".to_string());
        lines.push("        if haystack.startswith(p):".to_string());
        lines.push("            return p".to_string());
        lines.push("    return None".to_string());
        lines.push("".to_string());
        lines.push("def _asl_ends_with_any(haystack, suffixes):".to_string());
        lines.push("    if type(haystack) != \"string\":".to_string());
        lines.push("        return None".to_string());
        lines.push("    for s in suffixes:".to_string());
        lines.push("        if haystack.endswith(s):".to_string());
        lines.push("            return s".to_string());
        lines.push("    return None".to_string());
        lines.push("".to_string());

        // 2. Assinatura da Função Entrypoint
        let entrypoint = &manifest.interface.entrypoint;
        lines.push(format!("def {}(ctx, input):", entrypoint));
        let entry_line = lines.len();
        source_map.push(SourceMapEntry {
            rules_line: 1,
            starlark_line: entry_line,
            description: format!("Assinatura entrypoint '{}'", entrypoint),
        });

        // 3. Emissão de Cláusulas Guard
        for (i, guard) in block.guards.iter().enumerate() {
            let var_name = format!("_g_{}", i);
            let path_get = render_path_get(&guard.target);
            lines.push(format!("    {} = {}", var_name, path_get));
            static_invariants.push(format!(
                "guard:{}:{}",
                guard.target.to_dotted(),
                guard.error_msg
            ));

            let condition_expr = match &guard.condition {
                GuardCondition::IsNotEmpty => format!("len(str({}).strip()) == 0", var_name),
                GuardCondition::IsEmpty => format!("len(str({}).strip()) > 0", var_name),
                GuardCondition::IsTrue => format!("{} != True", var_name),
                GuardCondition::IsFalse => format!("{} != False", var_name),
                GuardCondition::CompareOp(op, val) => {
                    format!("not ({} {} {})", var_name, op, escape_str(val))
                }
            };

            lines.push(format!("    if {}:", condition_expr));
            let rej_dict = render_rejection_dict(&guard.error_msg, manifest);
            lines.push(format!("        return {}", rej_dict));

            source_map.push(SourceMapEntry {
                rules_line: guard.line,
                starlark_line: lines.len(),
                description: format!("Cláusula guard para '{}'", guard.target.to_dotted()),
            });
        }

        // 4. Emissão de Seções Match
        for (m_idx, match_sec) in block.matches.iter().enumerate() {
            let target_var = format!("_m_val_{}", m_idx);
            let path_get = render_path_get(&match_sec.target);
            lines.push(format!("    {} = str({}).strip()", target_var, path_get));

            // Pré-avaliação de padrões com variáveis para garantir cadeia if/elif contígua
            for (w_idx, when) in match_sec.when_clauses.iter().enumerate() {
                match &when.condition {
                    PatternCondition::StartsWithAny(prefixes) => {
                        let pref_var = format!("_p_{}_{}", m_idx, w_idx);
                        let prefixes_json =
                            serde_json::to_string(prefixes).unwrap_or_else(|_| "[]".to_string());
                        lines.push(format!(
                            "    {} = _asl_starts_with_any({}, {})",
                            pref_var, target_var, prefixes_json
                        ));
                    }
                    PatternCondition::EndsWithAny(suffixes) => {
                        let suff_var = format!("_s_{}_{}", m_idx, w_idx);
                        let suffixes_json =
                            serde_json::to_string(suffixes).unwrap_or_else(|_| "[]".to_string());
                        lines.push(format!(
                            "    {} = _asl_ends_with_any({}, {})",
                            suff_var, target_var, suffixes_json
                        ));
                    }
                    _ => {}
                }
            }

            for (w_idx, when) in match_sec.when_clauses.iter().enumerate() {
                let is_first = w_idx == 0;
                let if_keyword = if is_first { "if" } else { "elif" };

                match &when.condition {
                    PatternCondition::StartsWithAny(_) => {
                        let pref_var = format!("_p_{}_{}", m_idx, w_idx);
                        lines.push(format!("    {} {} != None:", if_keyword, pref_var));
                        if let Some(alias) = &when.alias {
                            lines.push(format!("        {} = {}", alias, pref_var));
                        }
                    }
                    PatternCondition::EndsWithAny(_) => {
                        let suff_var = format!("_s_{}_{}", m_idx, w_idx);
                        lines.push(format!("    {} {} != None:", if_keyword, suff_var));
                        if let Some(alias) = &when.alias {
                            lines.push(format!("        {} = {}", alias, suff_var));
                        }
                    }
                    PatternCondition::ContainsAny(needles) => {
                        let needles_json =
                            serde_json::to_string(needles).unwrap_or_else(|_| "[]".to_string());
                        lines.push(format!(
                            "    {} _asl_contains_any({}, {}):",
                            if_keyword, target_var, needles_json
                        ));
                    }
                    PatternCondition::Equals(expr) => {
                        let expr_str = render_value_expr(expr);
                        lines.push(format!(
                            "    {} {} == {}:",
                            if_keyword, target_var, expr_str
                        ));
                    }
                    PatternCondition::MatchesRegex(pattern) => {
                        lines.push(format!(
                            "    {} _asl_matches_regex({}, {}):",
                            if_keyword,
                            target_var,
                            escape_str(pattern)
                        ));
                    }
                }

                let action_code = render_action(&when.action, manifest);
                lines.push(format!("        return {}", action_code));

                source_map.push(SourceMapEntry {
                    rules_line: when.line,
                    starlark_line: lines.len(),
                    description: format!("When ramo {}", w_idx + 1),
                });
            }

            // Otherwise no Match
            let is_last_match = m_idx + 1 == block.matches.len();
            if let Some(oth) = &match_sec.otherwise {
                if !is_last_match {
                    return Err(AslError::RulesTranspileError(
                        "Cláusula 'otherwise' em match intermediário torna matches posteriores inalcançáveis.".to_string(),
                    ));
                }
                lines.push("    else:".to_string());
                let action_code = render_action(oth, manifest);
                lines.push(format!("        return {}", action_code));
            } else if is_last_match {
                if let Some(global_oth) = &block.otherwise {
                    lines.push("    else:".to_string());
                    let action_code = render_action(global_oth, manifest);
                    lines.push(format!("        return {}", action_code));
                } else {
                    return Err(AslError::RulesTranspileError(
                        "Exaustividade violada: O bloco 'match' requer cláusula 'otherwise:'."
                            .to_string(),
                    ));
                }
            }
        }

        // Se não houve match section, emitir otherwise global se existente
        if block.matches.is_empty() {
            if let Some(oth) = &block.otherwise {
                let action_code = render_action(oth, manifest);
                lines.push(format!("    return {}", action_code));
            } else {
                let default_ret = render_default_accept(manifest);
                lines.push(format!("    return {}", default_ret));
            }
        }

        let starlark_code = lines.join("\n");

        // 5. Verificação AOT em Memória via AstModule
        let dialect = Dialect::Standard;
        AstModule::parse("rules_transpiled.star", starlark_code.clone(), &dialect).map_err(
            |e| {
                AslError::RulesTranspileError(format!(
                    "Falha na validação sintática AOT do código Starlark gerado: {}\nCódigo:\n{}",
                    e, starlark_code
                ))
            },
        )?;

        Ok(TranspilationResult {
            starlark_code,
            source_map,
            static_invariants,
        })
    }
}

fn render_path_get(path: &PathExpr) -> String {
    let segs = &path.segments;
    let segs_json = serde_json::to_string(segs).unwrap_or_else(|_| "[]".to_string());
    format!("_asl_get({}, {}, \"\")", path.root, segs_json)
}

fn render_value_expr(expr: &ValueExpr) -> String {
    match expr {
        ValueExpr::LiteralString(s) => escape_str(s),
        ValueExpr::LiteralInt(i) => i.to_string(),
        ValueExpr::LiteralFloat(f) => format!("{:.4}", f),
        ValueExpr::LiteralBool(b) => {
            if *b {
                "True".to_string()
            } else {
                "False".to_string()
            }
        }
        ValueExpr::Identifier(name) => name.clone(),
        ValueExpr::Path(path) => {
            let segs_json =
                serde_json::to_string(&path.segments).unwrap_or_else(|_| "[]".to_string());
            format!("_asl_get({}, {}, \"\")", path.root, segs_json)
        }
        ValueExpr::Concat(parts) => {
            let parts_str: Vec<String> = parts.iter().map(render_value_expr).collect();
            parts_str.join(" + ")
        }
        ValueExpr::Array(items) => {
            let items_str: Vec<String> = items.iter().map(render_value_expr).collect();
            format!("[{}]", items_str.join(", "))
        }
    }
}

fn render_action(action: &Action, manifest: &SkillManifest) -> String {
    match action {
        Action::Accept { named_args, .. } => {
            let mut pairs = Vec::new();
            for (k, v) in named_args {
                let key_str = escape_str(k);
                let val_str = render_value_expr(v);
                pairs.push(format!("{}: {}", key_str, val_str));
            }
            format!("{{{}}}", pairs.join(", "))
        }
        Action::Reject { message, .. } => render_rejection_dict(message, manifest),
    }
}

fn render_rejection_dict(msg: &str, manifest: &SkillManifest) -> String {
    let msg_escaped = escape_str(msg);
    if let Some(out_schema) = &manifest.interface.output_schema {
        if let Some(props) = out_schema.get("properties").and_then(|p| p.as_object()) {
            let mut pairs = Vec::new();
            for (key, val) in props {
                let key_escaped = escape_str(key);
                let p_type = val.get("type").and_then(|t| t.as_str()).unwrap_or("string");
                let default_val = match p_type {
                    "boolean" => "False".to_string(),
                    "array" => format!("[{}]", msg_escaped),
                    "integer" | "number" => "0".to_string(),
                    _ => {
                        if key.contains("error") || key.contains("msg") || key.contains("diag") {
                            msg_escaped.clone()
                        } else {
                            escape_str("unknown")
                        }
                    }
                };
                pairs.push(format!("{}: {}", key_escaped, default_val));
            }
            return format!("{{{}}}", pairs.join(", "));
        }
    }

    format!(
        "{{\"is_valid\": False, \"diagnostics\": [{}]}}",
        msg_escaped
    )
}

fn render_default_accept(manifest: &SkillManifest) -> String {
    if let Some(out_schema) = &manifest.interface.output_schema {
        if let Some(props) = out_schema.get("properties").and_then(|p| p.as_object()) {
            let mut pairs = Vec::new();
            for (key, val) in props {
                let key_escaped = escape_str(key);
                let p_type = val.get("type").and_then(|t| t.as_str()).unwrap_or("string");
                let default_val = match p_type {
                    "boolean" => "True".to_string(),
                    "array" => "[]".to_string(),
                    "integer" | "number" => "0".to_string(),
                    _ => escape_str(""),
                };
                pairs.push(format!("{}: {}", key_escaped, default_val));
            }
            return format!("{{{}}}", pairs.join(", "));
        }
    }
    "{\"is_valid\": True, \"diagnostics\": []}".to_string()
}

fn escape_str(s: &str) -> String {
    serde_json::to_string(s).unwrap_or_else(|_| format!("\"{}\"", s))
}
