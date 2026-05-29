use crate::lexer::lexer::Lexer;

pub enum LexerErrorKind {
    UnexpectedCharacter,
    UnterminatedString,
    UnterminatedChar,
    InvalidChar,
    InvalidNumber,

    UnterminatedMultilineComment
}

pub struct LexerError {
    pub kind: LexerErrorKind,
    pub file_name: String,
    pub line: usize,
    pub col: usize,
    pub message: String,
    pub source_line: String,
}

impl LexerError {
    pub fn init(lexer: &Lexer, kind: LexerErrorKind, message: String) -> Self {
        Self {
            kind,
            file_name: lexer.file_name.clone(),
            line: lexer.cursor.line,
            col: lexer.cursor.col,
            message,
            source_line: lexer.cursor.get_line(lexer.cursor.line),
        }
    }

    pub fn display(&self) {
        println!("\x1b[31merror:\x1b[0m {}", self.message);
        println!(" --> {}:{}:{}", self.file_name, self.line, self.col);
        println!();

        let line_str = self.line.to_string();

        println!("{} | {}", line_str, self.source_line);

        let padding = line_str.len() + 3 + self.col;
        println!("{:>width$}\x1b[31m^\x1b[0m", "", width = padding);

        println!("note: {}", self.message);
    }
}