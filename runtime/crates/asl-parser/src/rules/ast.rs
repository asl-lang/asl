//! Árvore Sintática Abstrata (AST) para a linguagem semântica declarativa ASL Rules.

use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct RulesBlock {
    pub guards: Vec<GuardClause>,
    pub matches: Vec<MatchSection>,
    pub otherwise: Option<Action>,
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct GuardClause {
    pub target: PathExpr,
    pub condition: GuardCondition,
    pub error_msg: String,
    pub line: usize,
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub enum GuardCondition {
    IsNotEmpty,
    IsEmpty,
    IsTrue,
    IsFalse,
    CompareOp(String, String),
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct MatchSection {
    pub target: PathExpr,
    pub when_clauses: Vec<WhenClause>,
    pub otherwise: Option<Action>,
    pub line: usize,
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct WhenClause {
    pub condition: PatternCondition,
    pub alias: Option<String>,
    pub action: Action,
    pub line: usize,
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub enum PatternCondition {
    StartsWithAny(Vec<String>),
    EndsWithAny(Vec<String>),
    ContainsAny(Vec<String>),
    MatchesRegex(String),
    Equals(ValueExpr),
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub enum Action {
    Accept {
        named_args: Vec<(String, ValueExpr)>,
        line: usize,
    },
    Reject {
        message: String,
        line: usize,
    },
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct PathExpr {
    pub root: String,
    pub segments: Vec<String>,
}

impl PathExpr {
    pub fn from_dotted(dotted: &str) -> Self {
        let parts: Vec<String> = dotted.split('.').map(|s| s.trim().to_string()).collect();
        if parts.is_empty() {
            Self {
                root: "input".to_string(),
                segments: Vec::new(),
            }
        } else {
            Self {
                root: parts[0].clone(),
                segments: parts[1..].to_vec(),
            }
        }
    }

    pub fn to_dotted(&self) -> String {
        if self.segments.is_empty() {
            self.root.clone()
        } else {
            format!("{}.{}", self.root, self.segments.join("."))
        }
    }
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub enum ValueExpr {
    LiteralString(String),
    LiteralInt(i64),
    LiteralFloat(f64),
    LiteralBool(bool),
    Path(PathExpr),
    Concat(Vec<ValueExpr>),
    Array(Vec<ValueExpr>),
    Identifier(String),
}
