//! Lexer canônico para a linguagem semântica declarativa ASL Rules com normalização de indentação.

#[derive(Debug, Clone, PartialEq)]
pub enum TokenKind {
    Guard,
    Match,
    When,
    Otherwise,
    Else,
    Reject,
    Accept,
    StartsWith,
    EndsWith,
    Contains,
    Matches,
    Any,
    As,
    Is,
    Not,
    Empty,
    Colon,
    LParen,
    RParen,
    LBracket,
    RBracket,
    Comma,
    Eq,
    Plus,
    Dot,
    EqEq,
    NotEq,
    Ident(String),
    Str(String),
    Int(i64),
    Float(f64),
    Bool(bool),
    Indent,
    Dedent,
    Newline,
    Eof,
}

#[derive(Debug, Clone, PartialEq)]
pub struct Token {
    pub kind: TokenKind,
    pub line: usize,
    pub col: usize,
}

pub struct Lexer<'a> {
    chars: Vec<char>,
    pos: usize,
    line: usize,
    col: usize,
    indent_stack: Vec<usize>,
    pending_tokens: Vec<Token>,
    _phantom: std::marker::PhantomData<&'a ()>,
}

impl<'a> Lexer<'a> {
    pub fn new(input: &'a str) -> Self {
        Self {
            chars: input.chars().collect(),
            pos: 0,
            line: 1,
            col: 1,
            indent_stack: vec![0],
            pending_tokens: Vec::new(),
            _phantom: std::marker::PhantomData,
        }
    }

    pub fn tokenize(mut self) -> Result<Vec<Token>, String> {
        let mut tokens = Vec::new();
        let mut at_line_start = true;

        while self.pos < self.chars.len() {
            if let Some(t) = self.pending_tokens.pop() {
                tokens.push(t);
                continue;
            }

            if at_line_start {
                at_line_start = false;
                let mut spaces = 0;
                while self.pos < self.chars.len() {
                    match self.chars[self.pos] {
                        ' ' => {
                            spaces += 1;
                            self.advance();
                        }
                        '\t' => {
                            spaces += 2;
                            self.advance();
                        }
                        '#' => {
                            self.skip_comment();
                            break;
                        }
                        '\n' | '\r' => {
                            break;
                        }
                        _ => break,
                    }
                }

                if self.pos < self.chars.len() && self.chars[self.pos] != '\n' && self.chars[self.pos] != '\r' && self.chars[self.pos] != '#' {
                    let current_indent = *self.indent_stack.last().unwrap();
                    if spaces > current_indent {
                        self.indent_stack.push(spaces);
                        tokens.push(Token { kind: TokenKind::Indent, line: self.line, col: self.col });
                    } else if spaces < current_indent {
                        while spaces < *self.indent_stack.last().unwrap() {
                            self.indent_stack.pop();
                            tokens.push(Token { kind: TokenKind::Dedent, line: self.line, col: self.col });
                        }
                        if spaces != *self.indent_stack.last().unwrap() {
                            return Err(format!("Line {}: Inconsistent indentation.", self.line));
                        }
                    }
                }
            }

            if self.pos >= self.chars.len() {
                break;
            }

            let c = self.chars[self.pos];

            if c == '#' {
                self.skip_comment();
                continue;
            }

            if c == ' ' || c == '\t' {
                self.advance();
                continue;
            }

            if c == '\n' || c == '\r' {
                self.advance();
                tokens.push(Token { kind: TokenKind::Newline, line: self.line - 1, col: self.col });
                at_line_start = true;
                continue;
            }

            let start_line = self.line;
            let start_col = self.col;

            let token = match c {
                ':' => { self.advance(); Token { kind: TokenKind::Colon, line: start_line, col: start_col } }
                '(' => { self.advance(); Token { kind: TokenKind::LParen, line: start_line, col: start_col } }
                ')' => { self.advance(); Token { kind: TokenKind::RParen, line: start_line, col: start_col } }
                '[' => { self.advance(); Token { kind: TokenKind::LBracket, line: start_line, col: start_col } }
                ']' => { self.advance(); Token { kind: TokenKind::RBracket, line: start_line, col: start_col } }
                ',' => { self.advance(); Token { kind: TokenKind::Comma, line: start_line, col: start_col } }
                '+' => { self.advance(); Token { kind: TokenKind::Plus, line: start_line, col: start_col } }
                '.' => { self.advance(); Token { kind: TokenKind::Dot, line: start_line, col: start_col } }
                '=' => {
                    self.advance();
                    if self.pos < self.chars.len() && self.chars[self.pos] == '=' {
                        self.advance();
                        Token { kind: TokenKind::EqEq, line: start_line, col: start_col }
                    } else {
                        Token { kind: TokenKind::Eq, line: start_line, col: start_col }
                    }
                }
                '!' => {
                    self.advance();
                    if self.pos < self.chars.len() && self.chars[self.pos] == '=' {
                        self.advance();
                        Token { kind: TokenKind::NotEq, line: start_line, col: start_col }
                    } else {
                        return Err(format!("Linha {}: '!' inesperado.", start_line));
                    }
                }
                '"' | '\'' => {
                    let s = self.read_string(c)?;
                    Token { kind: TokenKind::Str(s), line: start_line, col: start_col }
                }
                _ if c.is_ascii_digit() || (c == '-' && self.peek_digit()) => {
                    let tok = self.read_number()?;
                    Token { kind: tok, line: start_line, col: start_col }
                }
                _ if c.is_ascii_alphabetic() || c == '_' => {
                    let ident = self.read_identifier();
                    let kind = match ident.as_str() {
                        "guard" => TokenKind::Guard,
                        "match" => TokenKind::Match,
                        "when" => TokenKind::When,
                        "otherwise" => TokenKind::Otherwise,
                        "else" => TokenKind::Else,
                        "reject" => TokenKind::Reject,
                        "accept" => TokenKind::Accept,
                        "starts_with" => TokenKind::StartsWith,
                        "ends_with" => TokenKind::EndsWith,
                        "contains" => TokenKind::Contains,
                        "matches" => TokenKind::Matches,
                        "any" => TokenKind::Any,
                        "as" => TokenKind::As,
                        "is" => TokenKind::Is,
                        "not" => TokenKind::Not,
                        "empty" => TokenKind::Empty,
                        "true" => TokenKind::Bool(true),
                        "false" => TokenKind::Bool(false),
                        _ => TokenKind::Ident(ident),
                    };
                    Token { kind, line: start_line, col: start_col }
                }
                other => {
                    return Err(format!("Linha {}:{}: Caractere inesperado '{}'.", start_line, start_col, other));
                }
            };

            tokens.push(token);
        }

        while self.indent_stack.len() > 1 {
            self.indent_stack.pop();
            tokens.push(Token { kind: TokenKind::Dedent, line: self.line, col: self.col });
        }

        tokens.push(Token { kind: TokenKind::Eof, line: self.line, col: self.col });
        Ok(tokens)
    }

    fn advance(&mut self) {
        if self.pos < self.chars.len() {
            if self.chars[self.pos] == '\n' {
                self.line += 1;
                self.col = 1;
            } else {
                self.col += 1;
            }
            self.pos += 1;
        }
    }

    fn skip_comment(&mut self) {
        while self.pos < self.chars.len() && self.chars[self.pos] != '\n' && self.chars[self.pos] != '\r' {
            self.pos += 1;
        }
    }

    fn peek_digit(&self) -> bool {
        if self.pos + 1 < self.chars.len() {
            self.chars[self.pos + 1].is_ascii_digit()
        } else {
            false
        }
    }

    fn read_string(&mut self, quote: char) -> Result<String, String> {
        self.advance();
        let mut s = String::new();
        while self.pos < self.chars.len() {
            let c = self.chars[self.pos];
            if c == quote {
                self.advance();
                return Ok(s);
            }
            if c == '\\' {
                self.advance();
                if self.pos >= self.chars.len() {
                    return Err("Unterminated string literal with escape".to_string());
                }
                let esc = self.chars[self.pos];
                match esc {
                    'n' => s.push('\n'),
                    'r' => s.push('\r'),
                    't' => s.push('\t'),
                    '\\' => s.push('\\'),
                    '\'' => s.push('\''),
                    '"' => s.push('"'),
                    _ => { s.push('\\'); s.push(esc); }
                }
            } else {
                s.push(c);
            }
            self.advance();
        }
        Err("Unterminated string literal".to_string())
    }

    fn read_number(&mut self) -> Result<TokenKind, String> {
        let mut s = String::new();
        let mut is_float = false;

        if self.chars[self.pos] == '-' {
            s.push('-');
            self.advance();
        }

        while self.pos < self.chars.len() && (self.chars[self.pos].is_ascii_digit() || self.chars[self.pos] == '.') {
            if self.chars[self.pos] == '.' {
                if is_float {
                    break;
                }
                is_float = true;
            }
            s.push(self.chars[self.pos]);
            self.advance();
        }

        if is_float {
            s.parse::<f64>().map(TokenKind::Float).map_err(|e| e.to_string())
        } else {
            s.parse::<i64>().map(TokenKind::Int).map_err(|e| e.to_string())
        }
    }

    fn read_identifier(&mut self) -> String {
        let mut s = String::new();
        while self.pos < self.chars.len() && (self.chars[self.pos].is_ascii_alphanumeric() || self.chars[self.pos] == '_') {
            s.push(self.chars[self.pos]);
            self.advance();
        }
        s
    }
}
