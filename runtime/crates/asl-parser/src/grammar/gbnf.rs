use asl_spec::Result;
use serde_json::Value;

/// Compila um JSON Schema para uma gramática GBNF (GGML BNF para llama.cpp / Ollama)
pub fn compile_schema_to_gbnf(schema: &Value) -> Result<String> {
    let mut rules: Vec<(String, String)> = Vec::new();
    let mut counter = 0;

    let root_expr = compile_node(schema, "root_val", &mut rules, &mut counter)?;

    let mut out = String::new();
    out.push_str(&format!("root ::= {}\n\n", root_expr));

    for (name, expr) in &rules {
        out.push_str(&format!("{} ::= {}\n", name, expr));
    }

    if !rules.is_empty() {
        out.push('\n');
    }

    // Terminais primitivos da gramática GBNF
    out.push_str("ws ::= [ \\t\\n\\r]*\n");
    out.push_str("string ::= \"\\\"\" ([^\"\\\\] | \"\\\\\" ([\"\\\\/bfnrt] | \"u\" [0-9a-fA-F] [0-9a-fA-F] [0-9a-fA-F] [0-9a-fA-F]))* \"\\\"\"\n");
    out.push_str("number ::= \"-\"? [0-9]+ (\".\" [0-9]+)? ([eE] [-+]? [0-9]+)?\n");
    out.push_str("integer ::= \"-\"? [0-9]+\n");
    out.push_str("boolean ::= \"true\" | \"false\"\n");
    out.push_str("null ::= \"null\"\n");
    out.push_str("any_value ::= any_object | any_array | string | number | boolean | null\n");
    out.push_str("any_object ::= \"{\" ws (string ws \":\" ws any_value (\",\" ws string ws \":\" ws any_value)*)? \"}\" ws\n");
    out.push_str("any_array ::= \"[\" ws (any_value (\",\" ws any_value)*)? \"]\" ws\n");

    Ok(out)
}

fn compile_node(
    schema: &Value,
    name_hint: &str,
    rules: &mut Vec<(String, String)>,
    counter: &mut usize,
) -> Result<String> {
    if !schema.is_object() {
        return Ok("any_value".to_string());
    }

    // 1. Tratamento de Enumeração (ex: ["feat", "fix"])
    if let Some(enum_vals) = schema.get("enum").and_then(|v| v.as_array()) {
        let mut alternatives = Vec::new();
        for val in enum_vals {
            if let Some(s) = val.as_str() {
                let escaped = escape_gbnf_string(s);
                alternatives.push(format!("\"\\\"{}\\\"\"", escaped));
            } else {
                alternatives.push(format!("\"{}\"", val));
            }
        }
        if alternatives.is_empty() {
            return Ok("any_value".to_string());
        }
        let rule_name = alloc_rule_name(name_hint, counter);
        rules.push((rule_name.clone(), alternatives.join(" | ")));
        return Ok(rule_name);
    }

    // 2. Extração do tipo
    let type_str = schema
        .get("type")
        .and_then(|t| t.as_str())
        .unwrap_or("object");

    match type_str {
        "string" => Ok("string".to_string()),
        "integer" => Ok("integer".to_string()),
        "number" => Ok("number".to_string()),
        "boolean" => Ok("boolean".to_string()),
        "null" => Ok("null".to_string()),
        "array" => compile_array(schema, name_hint, rules, counter),
        "object" => compile_object(schema, name_hint, rules, counter),
        _ => Ok("any_value".to_string()),
    }
}

fn compile_array(
    schema: &Value,
    name_hint: &str,
    rules: &mut Vec<(String, String)>,
    counter: &mut usize,
) -> Result<String> {
    let item_expr = if let Some(items) = schema.get("items") {
        compile_node(items, &format!("{}_item", name_hint), rules, counter)?
    } else {
        "any_value".to_string()
    };

    let rule_name = alloc_rule_name(name_hint, counter);
    let rule_def = format!("\"[\" ws ({} (\",\" ws {})*)? \"]\" ws", item_expr, item_expr);
    rules.push((rule_name.clone(), rule_def));
    Ok(rule_name)
}

fn compile_object(
    schema: &Value,
    name_hint: &str,
    rules: &mut Vec<(String, String)>,
    counter: &mut usize,
) -> Result<String> {
    let properties = match schema.get("properties").and_then(|p| p.as_object()) {
        Some(props) if !props.is_empty() => props,
        _ => return Ok("any_object".to_string()),
    };

    let required_list: Vec<&str> = schema
        .get("required")
        .and_then(|r| r.as_array())
        .map(|arr| arr.iter().filter_map(|v| v.as_str()).collect())
        .unwrap_or_default();

    let mut required_props = Vec::new();
    let mut optional_props = Vec::new();

    // Ordenação canônica estável
    let mut sorted_keys: Vec<&String> = properties.keys().collect();
    sorted_keys.sort();

    for key in sorted_keys {
        let prop_schema = &properties[key];
        let prop_rule = compile_node(
            prop_schema,
            &format!("{}_{}", name_hint, key),
            rules,
            counter,
        )?;

        let escaped_key = escape_gbnf_string(key);
        let kv_segment = format!("\"\\\"{}\\\"\" ws \":\" ws {}", escaped_key, prop_rule);

        if required_list.contains(&key.as_str()) {
            required_props.push(kv_segment);
        } else {
            optional_props.push(kv_segment);
        }
    }

    let mut parts = Vec::new();
    parts.push("\"{\" ws".to_string());

    if !required_props.is_empty() {
        let req_joined = required_props.join(" \",\" ws ");
        parts.push(req_joined);

        for opt in optional_props {
            parts.push(format!("(\",\" ws {})?", opt));
        }
    } else if !optional_props.is_empty() {
        // Objeto com apenas propriedades opcionais
        let first = &optional_props[0];
        let mut opt_expr = format!("({}", first);
        for opt in &optional_props[1..] {
            opt_expr.push_str(&format!(" | (\",\" ws {})", opt));
        }
        opt_expr.push_str(")?");
        parts.push(opt_expr);
    }

    parts.push("\"}\" ws".to_string());

    let rule_name = alloc_rule_name(name_hint, counter);
    let rule_def = parts.join(" ");
    rules.push((rule_name.clone(), rule_def));
    Ok(rule_name)
}

fn alloc_rule_name(hint: &str, counter: &mut usize) -> String {
    *counter += 1;
    let sanitized = hint
        .chars()
        .map(|c| if c.is_ascii_alphanumeric() { c } else { '_' })
        .collect::<String>();
    format!("{}_{}", sanitized, counter)
}

fn escape_gbnf_string(s: &str) -> String {
    s.replace('\\', "\\\\").replace('"', "\\\"")
}

#[cfg(test)]
mod tests {
    use super::*;
    use serde_json::json;

    #[test]
    fn test_gbnf_enum() {
        let schema = json!({
            "type": "string",
            "enum": ["feat", "fix", "docs"]
        });
        let gbnf = compile_schema_to_gbnf(&schema).expect("Deve compilar enum");
        assert!(gbnf.contains("\"\\\"feat\\\"\" | \"\\\"fix\\\"\" | \"\\\"docs\\\"\""));
        assert!(gbnf.contains("root ::="));
    }

    #[test]
    fn test_gbnf_object_required_properties() {
        let schema = json!({
            "type": "object",
            "required": ["intent", "diff_stat"],
            "properties": {
                "intent": { "type": "string" },
                "diff_stat": { "type": "string" }
            }
        });
        let gbnf = compile_schema_to_gbnf(&schema).expect("Deve compilar objeto");
        assert!(gbnf.contains("\"\\\"diff_stat\\\"\" ws \":\" ws string"));
        assert!(gbnf.contains("\"\\\"intent\\\"\" ws \":\" ws string"));
    }

    #[test]
    fn test_gbnf_array_items() {
        let schema = json!({
            "type": "array",
            "items": { "type": "integer" }
        });
        let gbnf = compile_schema_to_gbnf(&schema).expect("Deve compilar array");
        assert!(gbnf.contains("\"[\" ws (integer (\",\" ws integer)*)? \"]\" ws"));
    }
}
