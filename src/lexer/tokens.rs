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
    pub(crate) v: Option<Vec<u8>>,
}

impl Token {
    pub fn stringify_value(&self) -> String {
        match &self.v {
            Some(bytes) => String::from_utf8_lossy(bytes).into_owned(),
            None => String::new(),
        }
    }
}
