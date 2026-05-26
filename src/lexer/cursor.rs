pub struct Cursor {
    pub src: String,
    pub chars: Vec<char>,
    pub position: usize,
    pub line: usize,
    pub col: usize,
}

impl Cursor {
    pub fn init(src: String) -> Self {
        let chars = src.chars().collect();

        Self {
            src,
            chars,
            position: 0,
            line: 1,
            col: 0,
        }
    }

    pub fn is_eof(&self) -> bool {
        self.position >= self.chars.len()
    }

    pub fn peek(&self) -> Option<char> {
        if self.is_eof() {
            None
        } else {
            Some(self.chars[self.position])
        }
    }

    pub fn peek_next(&mut self) -> Option<char> {
        if self.is_eof() {
            None
        } else {
            Some(self.chars[self.position])
        }
    }

    pub fn advance(&mut self) -> Option<char> {
        if self.is_eof() {
            None
        } else {
            let c = self.peek();
            self.position += 1;
            c
        }
    }

    // TODO: Consume expected doesn't throw
    pub fn consume(&mut self, expected: char) -> Option<char> {
        if self.peek()? != expected {
            None
        } else {
            self.advance()
        }
    }

    pub fn get_line(&self, target_line: usize) -> String {
        self.src
            .lines()
            .nth(target_line - 1)
            .unwrap_or("")
            .to_string()
    }
}
