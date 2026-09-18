/// Desugars ASL ergonomic constructs (such as `match <expr>:` with `when <pattern>:`)
/// into deterministic, portable `if / elif / else` branching.
pub fn desugar_asl_code(source: &str) -> String {
    let mut output = Vec::new();
    let mut match_stack: Vec<MatchContext> = Vec::new();
    let mut match_counter = 0;

    for line in source.lines() {
        let trimmed = line.trim();

        // Check if line is empty or a comment
        if trimmed.is_empty() || trimmed.starts_with('#') {
            output.push(line.to_string());
            continue;
        }

        let current_indent = line.len() - line.trim_start().len();

        // Pop completed match contexts if current line is less or equally indented
        while let Some(ctx) = match_stack.last() {
            if current_indent <= ctx.base_indent && !trimmed.starts_with("when ") && trimmed != "otherwise:" && trimmed != "else:" {
                match_stack.pop();
            } else {
                break;
            }
        }

        // 1. Detect `match <expr>:`
        if trimmed.starts_with("match ") && trimmed.ends_with(':') {
            let expr = trimmed["match ".len()..trimmed.len() - 1].trim();
            match_counter += 1;
            let var_name = format!("_asl_m_{}", match_counter);
            let indent_str = " ".repeat(current_indent);

            output.push(format!("{}{} = ({})", indent_str, var_name, expr));
            match_stack.push(MatchContext {
                base_indent: current_indent,
                var_name,
                has_clause: false,
            });
            continue;
        }

        // 2. Detect `when <pattern>:`
        if trimmed.starts_with("when ") && trimmed.ends_with(':') {
            if let Some(ctx) = match_stack.last_mut() {
                let pat_raw = trimmed["when ".len()..trimmed.len() - 1].trim();
                let indent_str = " ".repeat(ctx.base_indent);
                let keyword = if !ctx.has_clause {
                    ctx.has_clause = true;
                    "if"
                } else {
                    "elif"
                };

                // Pattern with `in [...]` or comma-separated values
                let condition = if let Some(rest) = pat_raw.strip_prefix("in ") {
                    format!("{} in {}", ctx.var_name, rest.trim())
                } else if pat_raw.contains(',') {
                    format!("{} in ({})", ctx.var_name, pat_raw)
                } else if let Some(target) = pat_raw.strip_prefix("starts_with ") {
                    format!("{}.startswith({})", ctx.var_name, target.trim())
                } else if let Some(target) = pat_raw.strip_prefix("ends_with ") {
                    format!("{}.endswith({})", ctx.var_name, target.trim())
                } else {
                    format!("{} == {}", ctx.var_name, pat_raw)
                };

                output.push(format!("{}{} {}:", indent_str, keyword, condition));
                continue;
            }
        }

        // 3. Detect `otherwise:`
        if (trimmed == "otherwise:" || trimmed == "else:") && !match_stack.is_empty() {
            if let Some(ctx) = match_stack.last() {
                let indent_str = " ".repeat(ctx.base_indent);
                output.push(format!("{}else:", indent_str));
                continue;
            }
        }

        // Normal line
        output.push(line.to_string());
    }

    output.join("\n")
}

struct MatchContext {
    base_indent: usize,
    var_name: String,
    has_clause: bool,
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_desugar_arbitrary_match() {
        let code = r#"
def validate(ch):
    match ch:
        when "0":
            return "zero"
        when "1", "2":
            return "small"
        when in ["3", "4"]:
            return "medium"
        otherwise:
            return "large"
"#;
        let desugared = desugar_asl_code(code);
        assert!(desugared.contains("_asl_m_1 = (ch)"));
        assert!(desugared.contains("if _asl_m_1 == \"0\":"));
        assert!(desugared.contains("elif _asl_m_1 in (\"1\", \"2\"):"));
        assert!(desugared.contains("elif _asl_m_1 in [\"3\", \"4\"]:"));
        assert!(desugared.contains("else:"));
        assert!(!desugared.contains("match ch:"));
    }
}
