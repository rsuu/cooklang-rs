pub type Tokens = Vec<Token>;

#[derive(Debug, PartialEq, Copy, Clone)]
pub struct Token {
    pub t: TokenType,
    pub p: Pos,
}

#[derive(Debug, PartialEq)]
pub enum TokenValue {
    Text(String),
    Sign,
    Null,
}

#[derive(Debug, PartialEq, Copy, Clone)]
pub enum TokenType {
    At,           // '@'
    Pound,        // '#'
    Tilde,        // '~'
    Percent,      // '%'
    OpenBrace,    // '{'
    CloseBrace,   // '}'
    OpenBracket,  // '['
    CloseBracket, // ']'
    Dot,          // '.'
    Lt,           // '<'
    Gt,           // '>'
    Minus,        // '-'
    Colon,        // ':'

    NewLine,    // '\n' | '\r'
    WhiteSpace, // ' '

    Int, //
    Float,

    Text, // other

    Null,
}

#[derive(Debug, PartialEq, Copy, Clone)]
pub struct Pos {
    pub start: usize,
    pub offset: usize,
    pub end: usize,
}

impl Pos {
    pub fn new() -> Self {
        Self {
            start: 0,
            offset: 0,
            end: 0,
        }
    }

    pub fn next(&mut self) {
        (*self).start += 1;
    }

    pub fn set_start(&mut self, start: usize) {
        (*self).start = start;
    }

    pub fn set_offset(&mut self, offset: usize) {
        (*self).offset = offset;
    }

    pub fn set_end(&mut self) {
        (*self).end = self.start + self.offset;
    }
}

#[derive(Debug, PartialEq)]
pub struct ParseInfo {
    pub ptr: usize,
    pub start: usize,
    pub end: usize,
}

impl ParseInfo {
    pub fn new(end: usize) -> Self {
        Self {
            ptr: 0,
            start: 0,
            end: end - 1,
        }
    }

    pub fn has_next(&self) -> bool {
        if self.ptr <= self.end {
            true
        } else {
            false
        }
    }

    pub fn next(&mut self) {
        self.ptr += 1;
    }

    pub fn peek(&self) -> usize {
        self.ptr + 1
    }
}
