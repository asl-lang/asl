//! Declarative semantic rules module for ASL (asl:rules).

pub mod ast;
pub mod lexer;
pub mod parser;
pub mod transpiler;

pub use ast::*;
pub use lexer::Lexer;
pub use parser::Parser;
pub use transpiler::RulesTranspiler;

/// Parses a rules string into a RulesBlock in the AST
pub fn parse_rules(source: &str) -> Result<RulesBlock, String> {
    let lexer = Lexer::new(source);
    let tokens = lexer.tokenize()?;
    let mut parser = Parser::new(tokens);
    parser.parse_rules_block()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_parse_rules_basic() {
        let input = r#"
guard:
  input.intent is not empty else reject("Commit intent cannot be empty.")

match input.intent:
  when starts_with any(["feat", "fix", "docs"]) as prefix:
    accept(is_valid=true, commit_type=prefix, message=input.intent)
  when contains any(["bug", "patch"]):
    accept(is_valid=true, commit_type="fix", message="fix: " + input.intent)
  otherwise:
    accept(is_valid=true, commit_type="feat", message="feat: " + input.intent)
"#;

        let ast = parse_rules(input).expect("Should parse rules successfully");
        assert_eq!(ast.guards.len(), 1);
        assert_eq!(ast.guards[0].target.to_dotted(), "input.intent");
        assert_eq!(ast.guards[0].condition, GuardCondition::IsNotEmpty);
        assert_eq!(ast.matches.len(), 1);
        assert_eq!(ast.matches[0].when_clauses.len(), 2);
        assert!(ast.matches[0].otherwise.is_some());
    }

    #[test]
    fn test_parse_guard_is_empty() {
        let input = r#"
guard:
  input.flag is empty else reject("Flag must be empty.")
"#;
        let ast = parse_rules(input).expect("Should parse");
        assert_eq!(ast.guards[0].condition, GuardCondition::IsEmpty);
    }

    #[test]
    fn test_transpile_rules_to_starlark() {
        use asl_core_traits::RulesTranspilerPort;
        use asl_spec::SkillManifest;

        let input = r#"
guard:
  input.intent is not empty else reject("Commit intent cannot be empty.")

match input.intent:
  when starts_with any(["feat", "fix", "docs"]) as prefix:
    accept(is_valid=true, commit_type=prefix, message=input.intent)
  when contains any(["bug", "patch"]):
    accept(is_valid=true, commit_type="fix", message="fix: " + input.intent)
  otherwise:
    accept(is_valid=true, commit_type="feat", message="feat: " + input.intent)
"#;

        let manifest: SkillManifest = serde_yaml::from_str(r#"
asl_version: "3.0"
name: "commit-validator"
interface:
  entrypoint: "validate_and_format"
"#).unwrap();

        let transpiler = RulesTranspiler::new();
        let res = transpiler.transpile(input, &manifest).expect("Transpilation should succeed with AOT");
        assert!(res.starlark_code.contains("def validate_and_format(ctx, input):"));
        assert!(res.starlark_code.contains("_asl_get(input, [\"intent\"], \"\")"));
        assert!(res.starlark_code.contains("_asl_starts_with_any"));
        assert!(!res.source_map.is_empty());
    }
}
