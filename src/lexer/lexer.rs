#![allow(dead_code)]

use crate::error::{LexerError, LexerErrorKind};
use crate::lexer::cursor::Cursor;
use crate::lexer::tokens::{Token, TokenType};

const KEYWORDS: [&str; 1] = ["return"];

pub struct Lexer {
    pub file_name: String,
    pub cursor: Cursor,
}

impl Lexer {
    pub fn new(src: String, file_name: String) -> Lexer {
        Self {
            file_name,
            cursor: Cursor::init(src),
        }
    }

    pub fn try_comment(&mut self) -> bool {
        if self.cursor.peek() == Some('/') && self.cursor.peek_next() == Some('/') {
            while self.cursor.peek() != Some('\n') && self.cursor.peek() != Some('\0') {
                self.cursor.advance();
            }

            true
        } else {
            false
        }
    }

    pub fn try_number(&mut self) -> Result<Option<Token>, LexerError> {
        let Some(c) = self.cursor.peek() else {
            return Ok(None);
        };

        if !c.is_ascii_digit() {
            return Ok(None);
        };

        let mut value = String::new();
        let mut is_float = false;

        while let Some(c) = self.cursor.peek() {
            if c.is_ascii_digit() {
                value.push(c);
                self.cursor.advance();
                continue;
            }

            if c == '.' {
                if is_float {
                    return Err(LexerError::init(
                        self,
                        LexerErrorKind::InvalidNumber,
                        String::from("Invalid float literals"),
                    ));
                }

                is_float = true;

                value.push(c);
                self.cursor.advance();

                match self.cursor.peek() {
                    Some(next) if next.is_ascii_digit() => {}
                    _ => {
                        return Err(LexerError::init(
                            self,
                            LexerErrorKind::InvalidNumber,
                            String::from("Expected digit after '.'"),
                        ));
                    }
                }

                continue;
            }

            if c.is_alphabetic() {
                return Err(LexerError::init(
                    self,
                    LexerErrorKind::InvalidNumber,
                    String::from("Invalid number literal"),
                ));
            }

            break;
        }

        Ok(Some(Token::with_value(
            if is_float {
                TokenType::Float
            } else {
                TokenType::Number
            },
            value,
        )))
    }

    pub fn push_single(&mut self, tokens: &mut Vec<Token>, t: TokenType) {
        tokens.push(Token { t, v: None });
        self.cursor.advance();
    }

    pub fn push_double(&mut self, tokens: &mut Vec<Token>, t: TokenType) {
        tokens.push(Token { t, v: None });
        self.cursor.advance();
        self.cursor.advance();
    }

    pub fn tokenize(&mut self) -> Result<Vec<Token>, LexerError> {
        let mut tokens: Vec<Token> = Vec::new();

        while let Some(c) = self.cursor.peek() {
            // TODO: Separate line management into its own manager
            if c == '\n' {
                self.cursor.line += 1;
                self.cursor.col = 0;
            } else {
                self.cursor.col += 1;
            }

            if self.try_comment() {
                continue;
            }

            // Skip whitespace
            if c.is_whitespace() {
                self.cursor.advance();
                continue;
            }

            // Numeric Literals
            if let Some(token) = self.try_number()? {
                tokens.push(token);
                continue;
            }

            // Keywords and Identifiers
            if c.is_alphabetic() {
                let mut value = String::new();

                while self.cursor.peek().is_some_and(char::is_alphanumeric) {
                    value.push(self.cursor.advance().expect(""));
                }

                tokens.push(Token {
                    t: if KEYWORDS.contains(&value.as_str()) {
                        TokenType::Keyword
                    } else {
                        TokenType::Identifier
                    },
                    v: Some(value),
                });

                continue;
            }

            let (symbol_token, advance_by) = match (c, self.cursor.peek_next().unwrap_or(' ')) {
                ('*', '*') => (Some(TokenType::TimesTimes), 2),

                ('=', '=') => (Some(TokenType::EqualsEquals), 2),
                ('!', '=') => (Some(TokenType::NotEquals), 2),
                ('<', '=') => (Some(TokenType::LessEqualThan), 2),
                ('>', '=') => (Some(TokenType::MoreEqualThan), 2),
                ('|', '|') => (Some(TokenType::Or), 2),
                ('&', '&') => (Some(TokenType::And), 2),

                ('+', _) => (Some(TokenType::Plus), 1),
                ('-', _) => (Some(TokenType::Minus), 1),
                ('*', _) => (Some(TokenType::Times), 1),
                ('/', _) => (Some(TokenType::Slash), 1),
                ('%', _) => (Some(TokenType::Percent), 1),

                ('=', _) => (Some(TokenType::Equals), 1),
                (';', _) => (Some(TokenType::Semi), 1),
                ('(', _) => (Some(TokenType::LParen), 1),
                (')', _) => (Some(TokenType::RParen), 1),

                ('!', _) => (Some(TokenType::Not), 1),
                ('>', _) => (Some(TokenType::MoreThan), 1),
                ('<', _) => (Some(TokenType::LessThan), 1),
                _ => (None, 0),
            };

            if symbol_token.is_some() {
                tokens.push(Token {
                    t: symbol_token.expect(""), // Since we literally check if its Some right above, this expect is just rust being rust
                    v: None,
                });

                for _ in 0..advance_by {
                    self.cursor.advance();
                }

                continue;
            }

            return Err(LexerError::init(
                self,
                LexerErrorKind::UnexpectedCharacter,
                String::from("Unknown symbol"),
            ));
        }

        tokens.push(Token {
            t: TokenType::EOF,
            v: None,
        });

        Ok(tokens)
    }
}
