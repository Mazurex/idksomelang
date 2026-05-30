#![allow(dead_code)]

use crate::cursor::LexerCursor;

#[derive(Copy, Clone, Debug)]
pub struct Position {
    pub line: usize,
    pub col: usize,
}

impl Position {
    pub fn new(line: usize, col: usize) -> Self {
        Self { line, col }
    }

    pub fn from_cursor(cursor: &LexerCursor) -> Self {
        Self {
            line: cursor.position.line,
            col: cursor.position.col
        }
    }

    pub fn advance_col(&mut self) {
        self.col += 1;
    }

    pub fn advance_line(&mut self) {
        self.line += 1;
        self.col = 0;
    }
}

#[derive(Clone, Debug)]
pub struct Span {
    pub start: Position,
    pub end: Position,
}

impl Span {
    pub fn new(start: Position, end: Position) -> Self {
        Self { start, end }
    }

    pub fn single(pos: Position) -> Self {
        Self {
            start: pos,
            end: pos,
        }
    }

    pub fn extend_to(&mut self, end: Position) {
        self.end = end;
    }
}
