//! Parser sintático recursivo descendente EBNF para a linguagem semântica declarativa ASL Rules.

use super::ast::*;
use super::lexer::{Token, TokenKind};

pub struct Parser {
    tokens: Vec<Token>,
    pos: usize,
}

impl Parser {
    pub fn new(tokens: Vec<Token>) -> Self {
        Self { tokens, pos: 0 }
    }

    pub fn parse_rules_block(&mut self) -> Result<RulesBlock, String> {
        let mut guards = Vec::new();
        let mut matches = Vec::new();
        let mut otherwise = None;

        self.skip_newlines();

        while !self.is_at_end() {
            match self.peek_kind() {
                TokenKind::Guard => {
                    guards.extend(self.parse_guard_section()?);
                }
                TokenKind::Match => {
                    matches.push(self.parse_match_section()?);
                }
                TokenKind::Otherwise => {
                    if otherwise.is_some() {
                        return Err(format!("Linha {}: Múltiplas cláusulas 'otherwise' encontradas.", self.peek().line));
                    }
                    otherwise = Some(self.parse_otherwise_section()?);
                }
                TokenKind::Dedent | TokenKind::Newline => {
                    self.advance();
                }
                _ => {
                    let tok = self.peek();
                    return Err(format!("Linha {}:{}: Token inesperado '{:?}'. Esperado 'guard:', 'match' ou 'otherwise:'.", tok.line, tok.col, tok.kind));
                }
            }
            self.skip_newlines();
        }

        Ok(RulesBlock { guards, matches, otherwise })
    }

    fn parse_guard_section(&mut self) -> Result<Vec<GuardClause>, String> {
        self.consume(TokenKind::Guard)?;
        self.consume(TokenKind::Colon)?;
        self.consume_newline_or_indent()?;

        let mut clauses = Vec::new();
        let mut in_indent = false;

        if self.match_token(TokenKind::Indent) {
            in_indent = true;
        }

        while !self.is_at_end() {
            self.skip_newlines();
            if in_indent && self.check(TokenKind::Dedent) {
                self.advance();
                break;
            }
            if self.check(TokenKind::Match) || self.check(TokenKind::Otherwise) || self.check(TokenKind::Eof) {
                break;
            }

            let line = self.peek().line;
            let path = self.parse_path_expr()?;
            let cond = self.parse_guard_condition()?;
            self.consume(TokenKind::Else)?;
            self.consume(TokenKind::Reject)?;
            self.consume(TokenKind::LParen)?;
            let error_msg = self.parse_string_literal()?;
            self.consume(TokenKind::RParen)?;
            self.skip_newlines();

            clauses.push(GuardClause { target: path, condition: cond, error_msg, line });
        }

        Ok(clauses)
    }

    fn parse_guard_condition(&mut self) -> Result<GuardCondition, String> {
        if self.match_token(TokenKind::Is) {
            if self.match_token(TokenKind::Not) {
                if self.match_token(TokenKind::Empty) {
                    return Ok(GuardCondition::IsNotEmpty);
                }
                return Err(format!("Linha {}: Esperado 'empty' após 'is not'.", self.peek().line));
            } else if self.match_token(TokenKind::Empty) {
                return Ok(GuardCondition::IsEmpty);
            }
        }
        if self.match_token(TokenKind::EqEq) {
            let lit = self.parse_value_literal_str()?;
            return Ok(GuardCondition::CompareOp("==".to_string(), lit));
        }
        Err(format!("Linha {}: Condição de guarda inválida. Use 'is not empty' ou 'is empty'.", self.peek().line))
    }

    fn parse_match_section(&mut self) -> Result<MatchSection, String> {
        let line = self.peek().line;
        self.consume(TokenKind::Match)?;
        let target = self.parse_path_expr()?;
        self.consume(TokenKind::Colon)?;
        self.consume_newline_or_indent()?;

        let mut when_clauses = Vec::new();
        let mut otherwise = None;
        let mut in_indent = false;

        if self.match_token(TokenKind::Indent) {
            in_indent = true;
        }

        while !self.is_at_end() {
            self.skip_newlines();
            if in_indent && self.check(TokenKind::Dedent) {
                self.advance();
                break;
            }
            if self.check(TokenKind::When) {
                when_clauses.push(self.parse_when_clause()?);
            } else if self.check(TokenKind::Otherwise) {
                if otherwise.is_some() {
                    return Err(format!("Linha {}: Múltiplas cláusulas 'otherwise' no bloco match.", self.peek().line));
                }
                otherwise = Some(self.parse_otherwise_section()?);
            } else if self.check(TokenKind::Match) || self.check(TokenKind::Guard) {
                break;
            } else {
                let tok = self.peek();
                return Err(format!("Linha {}: Esperado 'when' ou 'otherwise' no bloco match, encontrado '{:?}'.", tok.line, tok.kind));
            }
        }

        if when_clauses.is_empty() {
            return Err(format!("Linha {}: Bloco match deve conter ao menos uma cláusula 'when'.", line));
        }

        Ok(MatchSection { target, when_clauses, otherwise, line })
    }

    fn parse_when_clause(&mut self) -> Result<WhenClause, String> {
        let line = self.peek().line;
        self.consume(TokenKind::When)?;
        let condition = self.parse_pattern_condition()?;

        let alias = if self.match_token(TokenKind::As) {
            match self.advance().kind {
                TokenKind::Ident(s) => Some(s),
                _ => return Err(format!("Linha {}: Esperado identificador após 'as'.", line)),
            }
        } else {
            None
        };

        self.consume(TokenKind::Colon)?;
        self.consume_newline_or_indent()?;

        let mut in_indent = false;
        if self.match_token(TokenKind::Indent) {
            in_indent = true;
        }

        self.skip_newlines();
        let action = self.parse_action()?;
        self.skip_newlines();

        if in_indent && self.check(TokenKind::Dedent) {
            self.advance();
        }

        Ok(WhenClause { condition, alias, action, line })
    }

    fn parse_pattern_condition(&mut self) -> Result<PatternCondition, String> {
        let line = self.peek().line;
        if self.match_token(TokenKind::StartsWith) {
            let list = self.parse_string_or_any_list()?;
            return Ok(PatternCondition::StartsWithAny(list));
        } else if self.match_token(TokenKind::EndsWith) {
            let list = self.parse_string_or_any_list()?;
            return Ok(PatternCondition::EndsWithAny(list));
        } else if self.match_token(TokenKind::Contains) {
            let list = self.parse_string_or_any_list()?;
            return Ok(PatternCondition::ContainsAny(list));
        } else if self.match_token(TokenKind::Matches) {
            self.consume(TokenKind::LParen)?;
            let regex = self.parse_string_literal()?;
            self.consume(TokenKind::RParen)?;
            return Ok(PatternCondition::MatchesRegex(regex));
        } else if self.match_token(TokenKind::EqEq) {
            let val = self.parse_expression()?;
            return Ok(PatternCondition::Equals(val));
        }
        Err(format!("Linha {}: Condição de padrão inválida. Esperado starts_with, ends_with, contains ou matches.", line))
    }

    fn parse_string_or_any_list(&mut self) -> Result<Vec<String>, String> {
        if self.match_token(TokenKind::Any) {
            self.consume(TokenKind::LParen)?;
            self.consume(TokenKind::LBracket)?;
            let mut list = Vec::new();
            while !self.check(TokenKind::RBracket) && !self.is_at_end() {
                list.push(self.parse_string_literal()?);
                if !self.match_token(TokenKind::Comma) {
                    break;
                }
            }
            self.consume(TokenKind::RBracket)?;
            self.consume(TokenKind::RParen)?;
            Ok(list)
        } else {
            let single = self.parse_string_literal()?;
            Ok(vec![single])
        }
    }

    fn parse_otherwise_section(&mut self) -> Result<Action, String> {
        self.consume(TokenKind::Otherwise)?;
        self.consume(TokenKind::Colon)?;
        self.consume_newline_or_indent()?;

        let mut in_indent = false;
        if self.match_token(TokenKind::Indent) {
            in_indent = true;
        }

        self.skip_newlines();
        let action = self.parse_action()?;
        self.skip_newlines();

        if in_indent && self.check(TokenKind::Dedent) {
            self.advance();
        }

        Ok(action)
    }

    fn parse_action(&mut self) -> Result<Action, String> {
        let line = self.peek().line;
        if self.match_token(TokenKind::Accept) {
            self.consume(TokenKind::LParen)?;
            let mut named_args = Vec::new();
            self.skip_newlines();
            while !self.check(TokenKind::RParen) && !self.is_at_end() {
                let key = match self.advance().kind {
                    TokenKind::Ident(s) => s,
                    other => return Err(format!("Linha {}: Esperado nome de argumento em accept(), encontrado '{:?}'.", line, other)),
                };
                self.consume(TokenKind::Eq)?;
                let val = self.parse_expression()?;
                named_args.push((key, val));
                self.skip_newlines();
                if !self.match_token(TokenKind::Comma) {
                    break;
                }
                self.skip_newlines();
            }
            self.consume(TokenKind::RParen)?;
            Ok(Action::Accept { named_args, line })
        } else if self.match_token(TokenKind::Reject) {
            self.consume(TokenKind::LParen)?;
            let msg = self.parse_string_literal()?;
            self.consume(TokenKind::RParen)?;
            Ok(Action::Reject { message: msg, line })
        } else {
            Err(format!("Linha {}: Ação inválida. Esperado 'accept(...)' ou 'reject(...)'.", line))
        }
    }

    fn parse_expression(&mut self) -> Result<ValueExpr, String> {
        let mut left = self.parse_primary_expression()?;
        while self.match_token(TokenKind::Plus) {
            let right = self.parse_primary_expression()?;
            left = match left {
                ValueExpr::Concat(mut parts) => {
                    parts.push(right);
                    ValueExpr::Concat(parts)
                }
                other => ValueExpr::Concat(vec![other, right]),
            };
        }
        Ok(left)
    }

    fn parse_primary_expression(&mut self) -> Result<ValueExpr, String> {
        let tok = self.peek();
        match tok.kind.clone() {
            TokenKind::Str(s) => { self.advance(); Ok(ValueExpr::LiteralString(s)) }
            TokenKind::Int(n) => { self.advance(); Ok(ValueExpr::LiteralInt(n)) }
            TokenKind::Float(f) => { self.advance(); Ok(ValueExpr::LiteralFloat(f)) }
            TokenKind::Bool(b) => { self.advance(); Ok(ValueExpr::LiteralBool(b)) }
            TokenKind::LBracket => {
                self.advance();
                let mut items = Vec::new();
                while !self.check(TokenKind::RBracket) && !self.is_at_end() {
                    items.push(self.parse_expression()?);
                    if !self.match_token(TokenKind::Comma) {
                        break;
                    }
                }
                self.consume(TokenKind::RBracket)?;
                Ok(ValueExpr::Array(items))
            }
            TokenKind::Ident(ref name) => {
                if self.peek_next_is_dot() {
                    let path = self.parse_path_expr()?;
                    Ok(ValueExpr::Path(path))
                } else {
                    let s = name.clone();
                    self.advance();
                    Ok(ValueExpr::Identifier(s))
                }
            }
            other => Err(format!("Linha {}: Expressão inválida '{:?}'.", tok.line, other)),
        }
    }

    fn parse_path_expr(&mut self) -> Result<PathExpr, String> {
        let mut segments = Vec::new();
        let root = match self.advance().kind {
            TokenKind::Ident(s) => s,
            other => return Err(format!("Linha {}: Esperado identificador raiz no caminho, encontrado '{:?}'.", self.peek().line, other)),
        };

        while self.match_token(TokenKind::Dot) {
            match self.advance().kind {
                TokenKind::Ident(s) => segments.push(s),
                other => return Err(format!("Linha {}: Esperado identificador após '.', encontrado '{:?}'.", self.peek().line, other)),
            }
        }

        Ok(PathExpr { root, segments })
    }

    fn parse_string_literal(&mut self) -> Result<String, String> {
        let tok = self.advance();
        match tok.kind {
            TokenKind::Str(s) => Ok(s),
            _ => Err(format!("Linha {}: Esperado literal de string.", tok.line)),
        }
    }

    fn parse_value_literal_str(&mut self) -> Result<String, String> {
        let tok = self.advance();
        match tok.kind {
            TokenKind::Str(s) => Ok(s),
            TokenKind::Int(n) => Ok(n.to_string()),
            TokenKind::Float(f) => Ok(f.to_string()),
            TokenKind::Bool(b) => Ok(b.to_string()),
            _ => Err(format!("Linha {}: Esperado literal primitivo.", tok.line)),
        }
    }

    fn peek(&self) -> &Token {
        self.tokens.get(self.pos).unwrap_or(&Token { kind: TokenKind::Eof, line: 0, col: 0 })
    }

    fn peek_kind(&self) -> &TokenKind {
        &self.peek().kind
    }

    fn check(&self, kind: TokenKind) -> bool {
        self.peek_kind() == &kind
    }

    fn match_token(&mut self, kind: TokenKind) -> bool {
        if self.check(kind) {
            self.advance();
            true
        } else {
            false
        }
    }

    fn advance(&mut self) -> Token {
        if !self.is_at_end() {
            let t = self.tokens[self.pos].clone();
            self.pos += 1;
            t
        } else {
            self.peek().clone()
        }
    }

    fn consume(&mut self, kind: TokenKind) -> Result<Token, String> {
        if self.check(kind.clone()) {
            Ok(self.advance())
        } else {
            let tok = self.peek();
            Err(format!("Linha {}:{}: Esperado '{:?}', encontrado '{:?}'.", tok.line, tok.col, kind, tok.kind))
        }
    }

    fn consume_newline_or_indent(&mut self) -> Result<(), String> {
        while self.match_token(TokenKind::Newline) {}
        Ok(())
    }

    fn skip_newlines(&mut self) {
        while self.match_token(TokenKind::Newline) {}
    }

    fn peek_next_is_dot(&self) -> bool {
        self.tokens.get(self.pos + 1).map(|t| t.kind == TokenKind::Dot).unwrap_or(false)
    }

    fn is_at_end(&self) -> bool {
        self.peek_kind() == &TokenKind::Eof
    }
}
