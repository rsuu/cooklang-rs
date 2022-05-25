use crate::syntax::{
    tokens,
    tokens::{ParseInfo, Pos, Token, TokenType, TokenValue, Tokens},
};

pub fn lexer(v: &[char]) -> Tokens {
    let mut info = ParseInfo::new(v.len());
    let mut tokens: Tokens = Vec::new();
    let mut token: Token;
    let mut text: usize = 0;
    let mut pos = Pos::new();

    let mut ch: char;

    while info.ptr <= info.end {
        ch = v[info.ptr];

        // @雪梨{121.375589%g}
        match ch {
            '@' | '#' | '~' | '%' | '{' | '}' | '[' | ']' | '<' | '>' | '-' | ':' | ' ' | '\n'
            | '\r' => {
                eat_sign(&mut info, ch, &mut tokens);
            }

            '.' => {
                let last = tokens.last().expect("");
                if last.t == TokenType::Int && v[info.peek()].is_ascii_digit() {
                    let last_pos = last.p;
                    let mut pos = eat_number(&mut info, v);

                    pos.start = last_pos.start;
                    pos.offset = pos.offset + last_pos.offset + 1;
                    // Int: pos.offset
                    // Dot: 1
                    // Int: last_pos.offset

                    let len = &tokens.len();
                    tokens[len - 1] = Token {
                        t: TokenType::Float,
                        p: pos,
                    };
                } else {
                }
            }

            digit if digit.is_ascii_digit() => {
                // @xxx{111%g}

                token = Token {
                    t: TokenType::Int,
                    p: eat_number(&mut info, v),
                };
                tokens.push(token);
            }
            c => {
                eat_text(&mut info, v, &mut tokens);
                // println!("{}", info.ptr);
                // println!("{}", info.end);
                // println!("{:#?}", v);
                // println!("{:#?}", v.len());
            }
        }

        info.next();
        pos.start = info.ptr;
    }

    tokens
}

pub fn eat_sign(info: &mut ParseInfo, c: char, tokens: &mut Tokens) {
    let mut pos = Pos {
        start: info.ptr,
        offset: 0,
        end: info.ptr,
    };

    tokens.push(Token {
        t: c.match_sign(),
        p: pos,
    });
}

pub fn eat_text(info: &mut ParseInfo, v: &[char], tokens: &mut Tokens) {
    let mut pos = Pos {
        start: info.ptr,
        offset: 0,
        end: info.ptr,
    };

    while info.has_next() && v[info.peek()].match_sign() == TokenType::Text {
        pos.offset += 1;
        info.next();
    }

    pos.set_end();

    tokens.push(Token {
        t: TokenType::Text,
        p: pos,
    });
}

pub fn eat_number(info: &mut ParseInfo, v: &[char]) -> Pos {
    let mut pos = Pos {
        start: info.ptr,
        offset: 0,
        end: info.ptr,
    };

    while info.has_next() && v[info.peek()].is_ascii_digit() {
        pos.offset += 1;

        info.next();
    }

    pos.set_end();
    pos
}

pub trait CheckChar {
    fn match_sign(&self) -> TokenType;
}

impl CheckChar for char {
    fn match_sign(&self) -> TokenType {
        use tokens::TokenType::*;

        match *self {
            '@' => At,
            '#' => Pound,
            '~' => Tilde,
            '%' => Percent,
            '{' => OpenBrace,
            '}' => CloseBrace,
            '[' => OpenBracket,
            ']' => CloseBracket,
            // '.' => Dot,
            '<' => Lt,
            '>' => Gt,
            '-' => Minus,
            ':' => Colon,

            ' ' => WhiteSpace,
            '\n' => NewLine,
            '\r' => NewLine,

            _ => TokenType::Text,
        }
    }
}
