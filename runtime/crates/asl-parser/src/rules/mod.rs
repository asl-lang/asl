//! Módulo de regras semânticas declarativas do ASL (asl:rules).

pub mod ast;
pub mod lexer;
pub mod parser;

pub use ast::*;
pub use lexer::Lexer;
pub use parser::Parser;

/// Realiza o parsing de uma string de regras no bloco RulesBlock da AST
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
  input.intent is not empty else reject("A intenção do commit não pode estar vazia.")

match input.intent:
  when starts_with any(["feat", "fix", "docs"]) as prefix:
    accept(is_valid=true, commit_type=prefix, message=input.intent)
  when contains any(["bug", "corrigir"]):
    accept(is_valid=true, commit_type="fix", message="fix: " + input.intent)
  otherwise:
    accept(is_valid=true, commit_type="feat", message="feat: " + input.intent)
"#;

        let ast = parse_rules(input).expect("Deveria parsear regras com sucesso");
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
  input.flag is empty else reject("Flag deve estar vazia.")
"#;
        let ast = parse_rules(input).expect("Deveria parsear");
        assert_eq!(ast.guards[0].condition, GuardCondition::IsEmpty);
    }
}
