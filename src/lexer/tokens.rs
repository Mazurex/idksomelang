#[derive(Debug, PartialEq, Eq)]
#[allow(dead_code)]
pub enum TokenType {
    Number,
    Float,

    Plus,
    Minus,
    Times,
    TimesTimes,
    Slash,
    Percent,

    LParen,
    RParen,
    Semi,

    Equals,
    EqualsEquals,
    NotEquals,
    LessThan,
    MoreThan,
    LessEqualThan,
    MoreEqualThan,

    Not,
    Or,
    And,

    Identifier,
    Keyword,

    EOF,
}

#[derive(Debug)]
pub struct Token {
    pub(crate) t: TokenType,
    pub(crate) v: Option<String>,
}

impl Token {
    pub fn new(t: TokenType) -> Self {
        Self { t, v: None }
    }

    pub fn with_value(t: TokenType, v: String) -> Self {
        Self { t, v: Some(v) }
    }
}
