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
        let stripped = strip_inline_comment(trimmed);

        // Pop completed match contexts if current line is less or equally indented
        while let Some(ctx) = match_stack.last() {
            if current_indent <= ctx.base_indent {
                match_stack.pop();
            } else {
                break;
            }
        }

        // 1. Detect `match <expr>:`
        if stripped.starts_with("match ") && stripped.ends_with(':') {
            let expr = stripped["match ".len()..stripped.len() - 1].trim();
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
        if stripped.starts_with("when ") && stripped.ends_with(':') {
            if let Some(ctx) = match_stack.last_mut() {
                let pat_raw = stripped["when ".len()..stripped.len() - 1].trim();
                let indent_str = " ".repeat(ctx.base_indent);

                if pat_raw == "_" {
                    output.push(format!("{}else:", indent_str));
                    continue;
                }

                let keyword = if !ctx.has_clause {
                    ctx.has_clause = true;
                    "if"
                } else {
                    "elif"
                };

                let condition = if let Some(rest) = pat_raw.strip_prefix("in ") {
                    format!("{} in {}", ctx.var_name, rest.trim())
                } else if let Some(items) = split_top_level_commas(pat_raw) {
                    format!("{} in ({})", ctx.var_name, items.join(", "))
                } else if let Some(target) = pat_raw.strip_prefix("starts_with(")
                    .and_then(|s| s.strip_suffix(')'))
                    .or_else(|| pat_raw.strip_prefix("starts_with "))
                {
                    format!("{}.startswith({})", ctx.var_name, target.trim())
                } else if let Some(target) = pat_raw.strip_prefix("ends_with(")
                    .and_then(|s| s.strip_suffix(')'))
                    .or_else(|| pat_raw.strip_prefix("ends_with "))
                {
                    format!("{}.endswith({})", ctx.var_name, target.trim())
                } else if let Some(target) = pat_raw.strip_prefix("contains(")
                    .and_then(|s| s.strip_suffix(')'))
                    .or_else(|| pat_raw.strip_prefix("contains "))
                {
                    format!("{} in {}", target.trim(), ctx.var_name)
                } else if let Some(target) = pat_raw.strip_prefix("matches(")
                    .and_then(|s| s.strip_suffix(')'))
                    .or_else(|| pat_raw.strip_prefix("matches "))
                {
                    format!("_asl_matches_regex({}, {})", ctx.var_name, target.trim())
                } else if pat_raw == "is empty" {
                    format!("len({}) == 0", ctx.var_name)
                } else if pat_raw == "is not empty" {
                    format!("len({}) > 0", ctx.var_name)
                } else {
                    format!("{} == {}", ctx.var_name, pat_raw)
                };

                output.push(format!("{}{} {}:", indent_str, keyword, condition));
                continue;
            }
        }

        // 3. Detect `otherwise:` or `else:`
        if (stripped == "otherwise:" || stripped == "else:") && !match_stack.is_empty() {
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

fn strip_inline_comment(line: &str) -> &str {
    let mut in_single = false;
    let mut in_double = false;
    let mut escaped = false;

    for (i, c) in line.char_indices() {
        if escaped {
            escaped = false;
            continue;
        }
        if c == '\\' {
            escaped = true;
            continue;
        }
        if c == '"' && !in_single {
            in_double = !in_double;
        } else if c == '\'' && !in_double {
            in_single = !in_single;
        } else if c == '#' && !in_single && !in_double {
            return line[..i].trim_end();
        }
    }
    line.trim_end()
}

fn split_top_level_commas(pattern: &str) -> Option<Vec<String>> {
    let mut in_single = false;
    let mut in_double = false;
    let mut depth_paren: usize = 0;
    let mut depth_bracket: usize = 0;
    let mut depth_brace: usize = 0;
    let mut escaped = false;
    let mut items = Vec::new();
    let mut current = String::new();
    let mut has_top_comma = false;

    for c in pattern.chars() {
        if escaped {
            current.push(c);
            escaped = false;
            continue;
        }
        if c == '\\' {
            current.push(c);
            escaped = true;
            continue;
        }
        if c == '"' && !in_single {
            in_double = !in_double;
        } else if c == '\'' && !in_double {
            in_single = !in_single;
        } else if !in_single && !in_double {
            match c {
                '(' => depth_paren += 1,
                ')' => depth_paren = depth_paren.saturating_sub(1),
                '[' => depth_bracket += 1,
                ']' => depth_bracket = depth_bracket.saturating_sub(1),
                '{' => depth_brace += 1,
                '}' => depth_brace = depth_brace.saturating_sub(1),
                ',' if depth_paren == 0 && depth_bracket == 0 && depth_brace == 0 => {
                    has_top_comma = true;
                    items.push(current.trim().to_string());
                    current.clear();
                    continue;
                }
                _ => {}
            }
        }
        current.push(c);
    }

    if has_top_comma {
        if !current.trim().is_empty() {
            items.push(current.trim().to_string());
        }
        Some(items)
    } else {
        None
    }
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
    match ch:  # check char
        when "0":
            return "zero"
        when "1", "2":
            return "small"
        when in ["3", "4"]:
            return "medium"
        when contains("special"):
            return "special"
        otherwise:  # fallback
            return "large"
"#;
        let desugared = desugar_asl_code(code);
        assert!(desugared.contains("_asl_m_1 = (ch)"));
        assert!(desugared.contains("if _asl_m_1 == \"0\":"));
        assert!(desugared.contains("elif _asl_m_1 in (\"1\", \"2\"):"));
        assert!(desugared.contains("elif _asl_m_1 in [\"3\", \"4\"]:"));
        assert!(desugared.contains("elif \"special\" in _asl_m_1:"));
        assert!(desugared.contains("else:"));
        assert!(!desugared.contains("match ch:"));
    }

    #[test]
    fn test_desugar_nested_match() {
        let code = r#"
def nested(a, b):
    match a:
        when 1:
            match b:
                when 10:
                    return "1-10"
                otherwise:
                    return "1-other"
        when 2:
            return "2"
        when _:
            return "other"
"#;
        let desugared = desugar_asl_code(code);
        assert!(desugared.contains("_asl_m_1 = (a)"));
        assert!(desugared.contains("if _asl_m_1 == 1:"));
        assert!(desugared.contains("_asl_m_2 = (b)"));
        assert!(desugared.contains("if _asl_m_2 == 10:"));
        assert!(desugared.contains("elif _asl_m_1 == 2:"));
    }

    #[test]
    fn test_desugar_commas_inside_quotes_and_brackets() {
        let code = r#"
def check(val):
    match val:
        when "hello, world":
            return 1
        when [1, 2]:
            return 2
        when "a", "b":
            return 3
"#;
        let desugared = desugar_asl_code(code);
        assert!(desugared.contains("if _asl_m_1 == \"hello, world\":"));
        assert!(desugared.contains("elif _asl_m_1 == [1, 2]:"));
        assert!(desugared.contains("elif _asl_m_1 in (\"a\", \"b\"):"));
    }
}
