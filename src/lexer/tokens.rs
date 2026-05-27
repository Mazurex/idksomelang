#[derive(Debug, PartialEq, Eq)]
#[allow(dead_code)]
#[derive(Copy)]
#[derive(Clone)]
pub enum TokenType {
    NumberLit,
    FloatLit,

    Plus,
    Minus,
    Times,
    TimesTimes,
    Slash,
    Percent,
    Inc,
    Dec,

    LParen,
    RParen,
    LBrace,
    RBrace,
    LBracket,
    RBracket,
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
    If,
    Else,
    Number,

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

    pub fn as_str(&self) -> String {
        match &self.v {
            Some(val) => format!("{:?}({})", self.t, val),
            None => format!("{:?}", self.t)
        }
    }
}

pub const KEYWORDS: &[(&str, TokenType)] = &[
    ("return", TokenType::Return),
    ("if", TokenType::If),
    ("else", TokenType::Else),
    ("Number", TokenType::Number)
];

pub const SYMBOLS: &[(&str, TokenType)] = &[
    ("**", TokenType::TimesTimes),
    ("==", TokenType::EqualsEquals),
    ("!=", TokenType::NotEquals),
    ("<=", TokenType::LessEqualThan),
    (">=", TokenType::MoreEqualThan),
    ("||", TokenType::Or),
    ("&&", TokenType::And),
    ("++", TokenType::Inc),
    ("--", TokenType::Dec),

    ("+", TokenType::Plus),
    ("-", TokenType::Minus),
    ("*", TokenType::Times),
    ("/", TokenType::Slash),
    ("%", TokenType::Percent),

    ("=", TokenType::Equals),
    (";", TokenType::Semi),
    ("(", TokenType::LParen),
    (")", TokenType::RParen),
    ("{", TokenType::LBrace),
    ("}", TokenType::RBrace),
    ("[", TokenType::LBracket),
    ("]", TokenType::RBracket),

    ("!", TokenType::Not),
    (">", TokenType::MoreThan),
    ("<", TokenType::LessThan),
];