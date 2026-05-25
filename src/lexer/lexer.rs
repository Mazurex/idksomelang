#![allow(dead_code)]

use crate::error::{LexerError, LexerErrorKind};
use crate::lexer::tokens::{Token, TokenType};

const KEYWORDS: [&str; 1] = ["return"];

pub struct Lexer {
    pub file_name: String,
    pub(crate) src: String,
    pub(crate) chars: Vec<char>,
    pub(crate) position: usize,
    pub(crate) line: usize,
    pub(crate) col: usize,
}

impl Lexer {
    pub fn init(src: String, file_name: String) -> Lexer {
        let chars = src.chars().collect();

        Self {
            file_name,
            src,
            chars,
            position: 0,
            line: 1,
            col: 0,
        }
    }

    fn peek(&self) -> char {
        if self.position >= self.src.len() {
            return '\0';
        }

        self.chars[self.position]
    }

    fn peek_next(&self) -> char {
        if self.position + 1 >= self.src.len() {
            return '\0';
        }

        self.chars[self.position + 1]
    }

    fn is_eof(&self) -> bool {
        self.position >= self.src.len()
    }

    fn advance(&mut self) -> char {
        if self.is_eof() {
            return '\0';
        }

        let c = self.peek();
        self.position += 1;
        c
    }

    pub fn get_line(&self, target_line: usize) -> String {
        self.src
            .lines()
            .nth(target_line - 1)
            .unwrap_or("")
            .to_string()
    }

    pub fn tokenize(&mut self) -> Result<Vec<Token>, LexerError> {
        let mut tokens: Vec<Token> = Vec::new();

        while self.position < self.src.len() {
            let c = self.peek();

            if c == '\n' {
                self.line += 1;
                self.col = 0;
            } else {
                self.col += 1;
            }

            if c == '/' && self.peek_next() == '/' {
                while self.peek_next() != '\n' && self.peek_next() != '\0' {
                    self.advance();
                }

                self.advance();

                continue;
            }

            // Skip whitespace
            if c.is_whitespace() {
                self.advance();
                continue;
            }

            // Numeric Literals
            if c.is_numeric() {
                let mut value = String::new();
                let mut is_float = false;

                while self.peek().is_numeric() {
                    let n = self.peek_next();
                    if n.is_alphabetic() {
                        return Err(LexerError::init(
                            self,
                            LexerErrorKind::InvalidNumber,
                            "Invalid number literal".to_string(),
                        ));
                    }

                    if n == '.' {
                        if is_float {
                            return Err(LexerError::init(
                                self,
                                LexerErrorKind::InvalidNumber,
                                String::from("Invalid number literal"),
                            ));
                        }

                        is_float = true;

                        value.push(self.advance());

                        if !self.peek_next().is_numeric() {
                            return Err(LexerError::init(
                                self,
                                LexerErrorKind::InvalidNumber,
                                String::from("Invalid number literal"),
                            ));
                        }
                    }

                    value.push(self.advance());
                }

                tokens.push(Token {
                    t: if is_float {
                        TokenType::Float
                    } else {
                        TokenType::Number
                    },
                    value: Some(Vec::from(value)),
                });

                continue;
            }

            // Keywords and Identifiers
            if c.is_alphabetic() {
                let mut value = String::new();

                while self.peek().is_alphanumeric() {
                    value.push(self.advance());
                }

                tokens.push(Token {
                    t: if KEYWORDS.contains(&value.as_str()) {
                        TokenType::Keyword
                    } else {
                        TokenType::Identifier
                    },
                    value: Some(Vec::from(value)),
                });

                continue;
            }

            // Double symbols (ie. ==)
            match TokenType::try_from_two(c, self.peek_next()) {
                Ok(token) => {
                    tokens.push(Token {
                        t: token,
                        value: None,
                    });

                    self.advance();
                    self.advance();

                    continue;
                }
                Err(_) => {}
            }

            // Singular symbols (ie. =)
            match TokenType::try_from(c) {
                Ok(token) => {
                    tokens.push(Token {
                        t: token,
                        value: None,
                    });

                    self.advance();

                    continue;
                }
                Err(_) => {}
            }

            return Err(LexerError::init(
                self,
                LexerErrorKind::UnexpectedCharacter,
                String::from("Unknown symbol"),
            ));
        }

        tokens.push(Token {
            t: TokenType::EOF,
            value: None,
        });

        Ok(tokens)
    }
}
