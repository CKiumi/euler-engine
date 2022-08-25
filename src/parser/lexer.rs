use crate::Num;

#[derive(Clone, PartialEq, Eq, Debug)]
pub enum Token {
    Infix(Infix),
    LCurlyBrace,
    RCurlyBrace,
    RParen,
    LParen,
    Num(Num),
    Sym(String),
    Error(String),
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
    input: std::str::Chars<'a>,
    pub cur: char,
    pub peek: char,
}

impl<'a> Lexer<'a> {
    pub fn new(input: &'a str) -> Self {
        let mut lexer = Lexer {
            input: input.chars(),
            cur: ' ',
            peek: ' ',
        };
        lexer.read_char();
        lexer.read_char();
        lexer
    }

    pub fn read_char(&mut self) -> char {
        let c = self.cur;
        self.cur = self.peek;
        self.peek = self.input.next().unwrap_or('\u{0}');
        c
    }

    pub fn skip_whitespace(&mut self) {
        while self.cur == ' ' || self.cur == '\t' || self.cur == '\n' || self.cur == '\r' {
            self.read_char();
        }
    }

    pub fn next_token(&mut self) -> Token {
        self.skip_whitespace();
        let token = match self.cur {
            '+' => Token::Infix(Infix::Add),
            '*' => Token::Infix(Infix::Mul),
            '{' => Token::LCurlyBrace,
            '}' => Token::RCurlyBrace,
            '^' => Token::Infix(Infix::Circumflex),
            '_' => Token::Infix(Infix::Underscore),
            '\u{0}' => return Token::Eof,
            '\\' => return self.read_command(),
            c if c.is_ascii_alphabetic() => Token::Sym(self.cur.to_string()),
            c if c.is_ascii_digit() => return Token::Num(self.read_number()),
            _ => panic!("non ascii char is not allowed"),
        };
        self.read_char();
        token
    }

    fn read_command(&mut self) -> Token {
        self.read_char();
        let mut command = String::new();

        while self.cur.is_ascii_alphabetic() {
            command.push(self.read_char());
        }

        match &command as &str {
            "left" => {
                self.skip_whitespace();
                match self.read_char() {
                    '(' => Token::LParen,
                    _ => Token::Error("Unexpected left command".to_string()),
                }
            }
            "right" => {
                self.skip_whitespace();
                match self.read_char() {
                    ')' => Token::RParen,
                    _ => Token::Error("Unexpected right command".to_string()),
                }
            }
            _ => Token::Sym(format!("\\{}", command)),
        }
    }

    fn read_number(&mut self) -> Num {
        let mut number = String::new();
        while self.cur.is_ascii_digit() {
            number.push(self.read_char());
            self.skip_whitespace();
        }
        match number.parse::<i32>() {
            Ok(num) => Num::new(num),
            Err(_) => panic!("Unexpected number parse error"),
        }
    }

    pub fn arg_to_string(&mut self) -> String {
        let mut result = String::new();
        self.read_char();
        while self.cur != '}' {
            result.push(self.read_char());
            if self.cur == '\u{0}' {
                panic!("expected {}", "}")
            }
        }
        self.read_char();
        result
    }
}

#[test]
fn test_lexer() {
    let mut lexer = Lexer::new(" \\left ( {\\} ab +3 2\\right)\\zeta _{x} c\\x");
    assert_eq!(lexer.next_token(), Token::LParen);
    assert_eq!(lexer.next_token(), Token::LCurlyBrace);
    assert_eq!(lexer.next_token(), Token::Sym("\\".to_owned()));
    assert_eq!(lexer.next_token(), Token::RCurlyBrace);
    assert_eq!(lexer.next_token(), Token::Sym("a".to_owned()));
    assert_eq!(lexer.next_token(), Token::Sym("b".to_owned()));
    assert_eq!(lexer.next_token(), Token::Infix(Infix::Add));
    assert_eq!(lexer.next_token(), Token::Num(Num::new(32)));
    assert_eq!(lexer.next_token(), Token::RParen);
    assert_eq!(lexer.next_token(), Token::Sym("\\zeta".to_owned()));
    assert_eq!(lexer.next_token(), Token::Infix(Infix::Underscore));
    assert_eq!(lexer.next_token(), Token::LCurlyBrace);
    assert_eq!(lexer.next_token(), Token::Sym("x".to_owned()));
    assert_eq!(lexer.next_token(), Token::RCurlyBrace);
    assert_eq!(lexer.next_token(), Token::Sym("c".to_owned()));
    assert_eq!(lexer.next_token(), Token::Sym("\\x".to_owned()));
    assert_eq!(lexer.next_token(), Token::Eof);
}
