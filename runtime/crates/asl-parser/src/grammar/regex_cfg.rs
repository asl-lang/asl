use asl_spec::Result;
use serde_json::Value;

/// Compila um JSON Schema para uma Expressão Regular DFA (para vLLM, SGLang, Outlines)
pub fn compile_schema_to_regex(schema: &Value) -> Result<String> {
    compile_regex_node(schema)
}

fn compile_regex_node(schema: &Value) -> Result<String> {
    if !schema.is_object() {
        return Ok(r#"(\{.*?\}|\[.*?\]|"[^"\\]*"|-?[0-9]+|true|false|null)"#.to_string());
    }

    // 1. Enumerações (ex: ["feat", "fix"])
    if let Some(enum_vals) = schema.get("enum").and_then(|v| v.as_array()) {
        let mut alternatives = Vec::new();
        for val in enum_vals {
            if let Some(s) = val.as_str() {
                alternatives.push(escape_regex_literal(s));
            } else {
                alternatives.push(val.to_string());
            }
        }
        if !alternatives.is_empty() {
            return Ok(format!("\"({})\"", alternatives.join("|")));
        }
    }

    // 2. Tipos de dados
    let type_str = schema
        .get("type")
        .and_then(|t| t.as_str())
        .unwrap_or("object");

    match type_str {
        "string" => Ok(r#""([^"\\]|\\.)*""#.to_string()),
        "integer" => Ok(r#"-?[0-9]+"#.to_string()),
        "number" => Ok(r#"-?[0-9]+(\.[0-9]+)?([eE][+-]?[0-9]+)?"#.to_string()),
        "boolean" => Ok(r#"(true|false)"#.to_string()),
        "null" => Ok(r#"null"#.to_string()),
        "array" => compile_regex_array(schema),
        "object" => compile_regex_object(schema),
        _ => Ok(r#"(\{.*?\}|\[.*?\]|"[^"\\]*"|-?[0-9]+|true|false|null)"#.to_string()),
    }
}

fn compile_regex_array(schema: &Value) -> Result<String> {
    let item_regex = if let Some(items) = schema.get("items") {
        compile_regex_node(items)?
    } else {
        r#"[^\]]*"#.to_string()
    };

    Ok(format!(
        r#"\[\s*({}(\s*,\s*{})*)?\s*\]"#,
        item_regex, item_regex
    ))
}

fn compile_regex_object(schema: &Value) -> Result<String> {
    let properties = match schema.get("properties").and_then(|p| p.as_object()) {
        Some(props) if !props.is_empty() => props,
        _ => return Ok(r#"\{\s*.*?\s*\}"#.to_string()),
    };

    let required_list: Vec<&str> = schema
        .get("required")
        .and_then(|r| r.as_array())
        .map(|arr| arr.iter().filter_map(|v| v.as_str()).collect())
        .unwrap_or_default();

    let mut required_props = Vec::new();
    let mut optional_props = Vec::new();

    let mut sorted_keys: Vec<&String> = properties.keys().collect();
    sorted_keys.sort();

    for key in sorted_keys {
        let prop_schema = &properties[key];
        let prop_regex = compile_regex_node(prop_schema)?;
        let escaped_key = escape_regex_literal(key);
        let kv_str = format!(r#""{}"\s*:\s*{}"#, escaped_key, prop_regex);

        if required_list.contains(&key.as_str()) {
            required_props.push(kv_str);
        } else {
            optional_props.push(kv_str);
        }
    }

    let mut inner = String::new();
    if !required_props.is_empty() {
        inner.push_str(&required_props.join(r#"\s*,\s*"#));
        for opt in optional_props {
            inner.push_str(&format!(r#"(\s*,\s*{})?"#, opt));
        }
    } else if !optional_props.is_empty() {
        let all_opts = optional_props.join("|");
        inner.push_str(&format!(r#"(({})(\s*,\s*({}))*)?"#, all_opts, all_opts));
    }

    Ok(format!(r#"\{{\s*{}\s*\}}"#, inner))
}

fn escape_regex_literal(s: &str) -> String {
    let special = r#"[\^$.|?*+(){}"#;
    let mut out = String::new();
    for c in s.chars() {
        if special.contains(c) || c == '\\' {
            out.push('\\');
        }
        out.push(c);
    }
    out
}

#[cfg(test)]
mod tests {
    use super::*;
    use serde_json::json;

    #[test]
    fn test_regex_enum() {
        let schema = json!({
            "type": "string",
            "enum": ["feat", "fix"]
        });
        let regex = compile_schema_to_regex(&schema).expect("Deve compilar enum regex");
        assert_eq!(regex, r#""(feat|fix)""#);
    }

    #[test]
    fn test_regex_object() {
        let schema = json!({
            "type": "object",
            "required": ["intent"],
            "properties": {
                "intent": { "type": "string" }
            }
        });
        let regex = compile_schema_to_regex(&schema).expect("Deve compilar objeto regex");
        assert!(regex.contains(r#""intent"\s*:\s*"([^"\\]|\\.)*""#));
    }

    #[test]
    fn test_regex_all_optional_properties() {
        let schema = json!({
            "type": "object",
            "properties": {
                "opt1": { "type": "string" },
                "opt2": { "type": "integer" }
            }
        });
        let regex = compile_schema_to_regex(&schema).expect("Deve compilar objeto com opcionais");
        assert!(regex.contains(r#"\s*,\s*"#));
    }
}
