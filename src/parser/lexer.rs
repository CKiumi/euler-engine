use crate::Num;

#[derive(Clone, PartialEq, Eq, Debug)]
pub enum Token<'a> {
    Infix(Infix),
    LCurlyBrace,
    RCurlyBrace,
    RParen,
    LParen,
    Num(Num),
    Sym(&'a str),
    Error(&'a str),
    Eof,
}

#[derive(Clone, PartialEq, Eq, Debug)]
pub enum Infix {
    Add,
    Mul,
    Underscore,
    Circumflex,
}

#[derive(Debug, Clone)]
pub struct Lexer<'a> {
    cursor: usize,
    input: &'a str,
}

impl<'a> Lexer<'a> {
    pub fn new(input: &'a str) -> Self {
        Lexer { cursor: 0, input }
    }

    pub fn cur(&self) -> char {
        self.input[self.cursor..].chars().next().unwrap_or('\u{0}')
    }

    pub fn read_char(&mut self) -> char {
        let char = self.cur();
        self.cursor += 1;
        char
    }

    pub fn skip_whitespace(&mut self) {
        while let ' ' | '\t' | '\n' | '\r' = self.cur() {
            self.read_char();
        }
    }

    pub fn next_token(&mut self) -> Token<'a> {
        self.skip_whitespace();
        let token = match self.cur() {
            '+' => Token::Infix(Infix::Add),
            '*' => Token::Infix(Infix::Mul),
            '{' => Token::LCurlyBrace,
            '}' => Token::RCurlyBrace,
            '^' => Token::Infix(Infix::Circumflex),
            '_' => Token::Infix(Infix::Underscore),
            '\u{0}' => return Token::Eof,
            '\\' => return self.read_command(),
            c if c.is_ascii_alphabetic() => Token::Sym(&self.input[self.cursor..self.cursor + 1]),
            c if c.is_ascii_digit() => return Token::Num(self.read_number()),
            _ => panic!("non ascii char is not allowed"),
        };
        self.read_char();
        token
    }

    fn read_command(&mut self) -> Token<'a> {
        self.read_char();
        let mut offset = 0;
        while self.cur().is_ascii_alphabetic() {
            self.read_char();
            offset += 1;
        }
        match &self.input[self.cursor - offset..self.cursor] {
            "left" => {
                self.skip_whitespace();
                match self.read_char() {
                    '(' => Token::LParen,
                    _ => Token::Error("Unexpected left command"),
                }
            }
            "right" => {
                self.skip_whitespace();
                match self.read_char() {
                    ')' => Token::RParen,
                    _ => Token::Error("Unexpected right command"),
                }
            }
            _ => Token::Sym(&self.input[self.cursor - 1 - offset..self.cursor]),
        }
    }

    fn read_number(&mut self) -> Num {
        let mut number = String::new();
        while self.cur().is_ascii_digit() {
            number.push(self.read_char());
            self.skip_whitespace();
        }
        match number.parse::<i32>() {
            Ok(num) => Num::new(num),
            Err(_) => panic!("Unexpected number parse error"),
        }
    }

    pub fn arg_to_string(&mut self) -> &'a str {
        let mut offset = 0;
        while self.cur() != '}' {
            self.read_char();
            offset += 1;
            if self.cur() == '\u{0}' {
                panic!("expected {}", "}")
            }
        }
        self.read_char();
        &self.input[self.cursor - offset..self.cursor - 1]
    }
}

#[test]
fn test_lexer() {
    let mut lexer = Lexer::new(" \\left ( {\\} ab +3 2\\right) c\\x");
    assert_eq!(lexer.next_token(), Token::LParen);
    assert_eq!(lexer.next_token(), Token::LCurlyBrace);
    assert_eq!(lexer.next_token(), Token::Sym("\\"));
    assert_eq!(lexer.next_token(), Token::RCurlyBrace);
    assert_eq!(lexer.next_token(), Token::Sym("a"));
    assert_eq!(lexer.next_token(), Token::Sym("b"));
    assert_eq!(lexer.next_token(), Token::Infix(Infix::Add));
    assert_eq!(lexer.next_token(), Token::Num(Num::new(32)));
    assert_eq!(lexer.next_token(), Token::RParen);
    assert_eq!(lexer.next_token(), Token::Sym("c"));
    assert_eq!(lexer.next_token(), Token::Sym("\\x"));
    assert_eq!(lexer.next_token(), Token::Eof);
}
