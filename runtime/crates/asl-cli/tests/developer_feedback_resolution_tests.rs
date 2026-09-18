use asl_core_traits::{EnginePort, ParserPort};
use asl_parser::CommonMarkYamlParser;
use asl_security::MockSecurityContext;
use asl_vm_starlark::StarlarkEngine;
use serde_json::json;
use std::fs;
use std::path::PathBuf;

fn create_temp_dir(name: &str) -> PathBuf {
    let dir = std::env::temp_dir().join(format!("asl_test_{}_{}", name, std::process::id()));
    let _ = fs::remove_dir_all(&dir);
    fs::create_dir_all(&dir).unwrap();
    dir
}

#[test]
fn test_problem_1_and_2_runtime_matches_docs_and_clean_errors() {
    let parser = CommonMarkYamlParser::new();
    let engine = StarlarkEngine::new();

    // 1. Validating that ctx.fs.write, read, exists, list, ctx.env.get, ctx.crypto.base64_encode all exist and work
    let skill_yaml = r#"---
asl_version: "3.0"
name: "ocap-docs-skill"
description: "Demonstrates all capabilities promised in docs"
interface:
  entrypoint: "test_all_caps"
capabilities:
  fs:
    roots: ["."]
    write: ["."]
  env:
    keys: ["API_SECRET"]
---
# OCAP Verification
Test that all APIs work as documented.

```asl
def test_all_caps(ctx, input):
    # ctx.env
    secret = ctx.env.get("API_SECRET", "default_val")

    # ctx.crypto
    encoded = ctx.crypto.base64_encode("hello world")
    decoded = ctx.crypto.base64_decode(encoded)
    hashed = ctx.crypto.sha256("hello world")

    # ctx.fs write and read
    ctx.fs.write("test_output.txt", "asl runtime content")
    read_back = ctx.fs.read("test_output.txt")
    exists = ctx.fs.exists("test_output.txt")
    listed = ctx.fs.list(".")

    return {
        "secret": secret,
        "encoded": encoded,
        "decoded": decoded,
        "hashed": hashed,
        "read_back": read_back,
        "exists": exists,
        "has_file": "test_output.txt" in listed
    }
```
"#;

    let doc = parser.parse(skill_yaml).expect("Must parse valid skill");
    let temp_dir = create_temp_dir("p1_caps");

    let mock_ctx = MockSecurityContext::new(100_000)
        .with_env("API_SECRET", "super_secret_key");

    let result = engine
        .execute(
            &doc.deterministic_code,
            "test_all_caps",
            &json!({}),
            &mock_ctx,
            &doc.manifest.limits,
        )
        .expect("Execution must succeed with all promised OCap capabilities");

    assert_eq!(result.output["secret"], "super_secret_key");
    assert_eq!(result.output["decoded"], "hello world");
    assert_eq!(result.output["read_back"], "asl runtime content");
    assert_eq!(result.output["exists"], true);

    // 2. Validating Problem #2: Clean error diagnostics without leaking .star internals
    let invalid_script = r#"
def run(ctx, input):
    return ctx.non_existent_module.some_method()
"#;
    let err = engine
        .execute(
            invalid_script,
            "run",
            &json!({}),
            &mock_ctx,
            &doc.manifest.limits,
        )
        .unwrap_err();

    let err_msg = err.to_string();
    assert!(!err_msg.contains(".star"), "Error must not leak internal .star filenames: {}", err_msg);
    assert!(err_msg.contains("ASL"), "Error must provide clear ASL context: {}", err_msg);

    let _ = fs::remove_dir_all(&temp_dir);
}

#[test]
fn test_problem_3_arbitrary_expression_pattern_matching() {
    let parser = CommonMarkYamlParser::new();
    let engine = StarlarkEngine::new();
    let mock_ctx = MockSecurityContext::new(100_000);

    let skill_code = r#"---
asl_version: "3.0"
name: "match-arbitrary"
description: "Match on arbitrary variable"
interface:
  entrypoint: "classify"
---
# Classification

```asl
def classify(ctx, input):
    ch = input.get("char", "")
    res = "unknown"
    match ch:
        when "a":
            res = "apple"
        when "b", "c":
            res = "banana_or_cherry"
        when in ["x", "y", "z"]:
            res = "end_of_alphabet"
        otherwise:
            res = "other"
    return {"result": res}
```
"#;

    let doc = parser.parse(skill_code).expect("Must parse and desugar match statement");
    assert!(!doc.deterministic_code.contains("match ch:"), "match must be desugared");

    for (c, expected) in [
        ("a", "apple"),
        ("b", "banana_or_cherry"),
        ("c", "banana_or_cherry"),
        ("y", "end_of_alphabet"),
        ("q", "other"),
    ] {
        let out = engine
            .execute(
                &doc.deterministic_code,
                "classify",
                &json!({"char": c}),
                &mock_ctx,
                &doc.manifest.limits,
            )
            .expect("Execution of desugared match must succeed");
        assert_eq!(out.output["result"], expected);
    }
}

#[test]
fn test_problem_4_and_5_string_iteration_and_safe_numeric_conversion() {
    let parser = CommonMarkYamlParser::new();
    let engine = StarlarkEngine::new();
    let mock_ctx = MockSecurityContext::new(100_000);

    let skill_code = r#"---
asl_version: "3.0"
name: "string-helpers"
description: "Iterate characters and safely parse ints"
interface:
  entrypoint: "parse_issue"
---
# String helpers

```asl
def parse_issue(ctx, input):
    raw_id = input.get("issue_id", "")
    char_list = chars(raw_id)

    # Safe validation without crashing
    digits = []
    for c in char_list:
        if is_digit(c):
            digits.append(c)

    # Safe conversion without try/except crash
    parsed_val = to_int(raw_id, default=-1)
    float_val = to_float(input.get("score", ""), default=0.0)

    return {
        "char_count": len(char_list),
        "digits_found": "".join(digits),
        "parsed_id": parsed_val,
        "score": float_val,
        "is_valid_int": is_int(raw_id)
    }
```
"#;

    let doc = parser.parse(skill_code).unwrap();

    // Valid numeric issue
    let out1 = engine
        .execute(
            &doc.deterministic_code,
            "parse_issue",
            &json!({"issue_id": "12345", "score": "98.5"}),
            &mock_ctx,
            &doc.manifest.limits,
        )
        .unwrap();

    assert_eq!(out1.output["char_count"], 5);
    assert_eq!(out1.output["digits_found"], "12345");
    assert_eq!(out1.output["parsed_id"], 12345);
    assert_eq!(out1.output["is_valid_int"], true);

    // Non-numeric issue with alphanumeric mixed
    let out2 = engine
        .execute(
            &doc.deterministic_code,
            "parse_issue",
            &json!({"issue_id": "PROJ-99", "score": "invalid"}),
            &mock_ctx,
            &doc.manifest.limits,
        )
        .unwrap();

    assert_eq!(out2.output["char_count"], 7);
    assert_eq!(out2.output["digits_found"], "99");
    assert_eq!(out2.output["parsed_id"], -1);
    assert_eq!(out2.output["is_valid_int"], false);
    assert_eq!(out2.output["score"], 0.0);
}

#[test]
fn test_problem_6_resilient_capabilities_deserialization() {
    let parser = CommonMarkYamlParser::new();

    let yaml_shorthand = r#"---
asl_version: "3.0"
name: "shorthand-caps"
description: "Testing shorthand capabilities parsing"
capabilities:
  fs:
    - "./my_data"
  domains:
    - "api.github.com"
  env:
    - "MY_KEY"
---
# Prompt
"#;

    let doc = parser.parse(yaml_shorthand).expect("Must parse shorthand list capabilities");
    assert_eq!(doc.manifest.capabilities.fs.confined_read_roots, vec!["./my_data".to_string()]);
    assert_eq!(doc.manifest.capabilities.net.allow_domains, vec!["api.github.com".to_string()]);
    assert_eq!(doc.manifest.capabilities.env.allow_keys, vec!["MY_KEY".to_string()]);

    let yaml_structured = r#"---
asl_version: "3.0"
name: "structured-caps"
description: "Testing structured capabilities parsing"
capabilities:
  fs:
    roots: ["/opt/data"]
    write: ["/tmp/scratch"]
  net:
    domains: ["service.internal"]
  env:
    keys: ["SECRET_KEY"]
---
# Prompt
"#;

    let doc2 = parser.parse(yaml_structured).expect("Must parse structured capabilities");
    assert_eq!(doc2.manifest.capabilities.fs.confined_read_roots, vec!["/opt/data".to_string()]);
    assert_eq!(doc2.manifest.capabilities.fs.allow_write, vec!["/tmp/scratch".to_string()]);
    assert_eq!(doc2.manifest.capabilities.net.allow_domains, vec!["service.internal".to_string()]);
    assert_eq!(doc2.manifest.capabilities.env.allow_keys, vec!["SECRET_KEY".to_string()]);
}
