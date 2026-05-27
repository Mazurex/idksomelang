#[derive(Debug, PartialEq, Eq)]
#[allow(dead_code)]
#[derive(Copy)]
#[derive(Clone)]
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

    Return,

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

pub const KEYWORDS: &[(&str, TokenType)] = &[
    ("return", TokenType::Return),
];

pub const SYMBOLS: &[(&str, TokenType)] = &[
    ("**", TokenType::TimesTimes),
    ("==", TokenType::EqualsEquals),
    ("!=", TokenType::NotEquals),
    ("<=", TokenType::LessEqualThan),
    (">=", TokenType::MoreEqualThan),
    ("||", TokenType::Or),
    ("&&", TokenType::And),

    ("+", TokenType::Plus),
    ("-", TokenType::Minus),
    ("*", TokenType::Times),
    ("/", TokenType::Slash),
    ("%", TokenType::Percent),

    ("=", TokenType::Equals),
    (";", TokenType::Semi),
    ("(", TokenType::LParen),
    (")", TokenType::RParen),

    ("!", TokenType::Not),
    (">", TokenType::MoreThan),
    ("<", TokenType::LessThan),
];