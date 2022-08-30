use crate::{expr::FuncName, Num};

#[derive(Clone, PartialEq, Eq, Debug)]
pub enum Token {
    Infix(Infix),
    Minus,
    LCurlyBrace,
    RCurlyBrace,
    RParen,
    LParen,
    Num(Num),
    Sym(String),
    Func(FuncName),
    Frac,
    MatrixBegin,
    MatrixEnd,
    NewLine,
    Ampersand,
    Error(String),
    Eof,
}

#[derive(Clone, PartialEq, Eq, Debug, PartialOrd, Ord)]
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
            '-' => Token::Minus,
            '*' => Token::Infix(Infix::Mul),
            '(' => Token::LParen,
            ')' => Token::RParen,
            '{' => Token::LCurlyBrace,
            '}' => Token::RCurlyBrace,
            '^' => Token::Infix(Infix::Circumflex),
            '_' => Token::Infix(Infix::Underscore),
            '&' => Token::Ampersand,
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
        if self.cur == '\\' {
            self.read_char();
            return Token::NewLine;
        }
        while self.cur.is_ascii_alphabetic() {
            command.push(self.read_char());
        }
        match command.as_str() {
            "frac" => Token::Frac,
            "sin" => Token::Func(FuncName::Sin),
            "cos" => Token::Func(FuncName::Cos),
            "tan" => Token::Func(FuncName::Tan),
            "Re" => Token::Func(FuncName::Re),
            "Im" => Token::Func(FuncName::Im),
            "sqrt" => Token::Func(FuncName::Sqrt),
            "begin" => {
                self.skip_whitespace();
                match self.arg_to_string().as_str() {
                    "matrix" | "pmatrix" | "bmatrix" => Token::MatrixBegin,
                    _ => Token::Error("unknown command".to_string()),
                }
            }
            "end" => {
                self.skip_whitespace();
                match self.arg_to_string().as_str() {
                    "matrix" | "pmatrix" | "bmatrix" => Token::MatrixEnd,
                    _ => Token::Error("unknown command".to_string()),
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
                panic!("expected }}")
            }
        }
        self.read_char();
        result
    }
}

#[test]
fn test_lexer() {
    let mut lexer = Lexer::new(
        " - (\\Re(x) {\\} ab +3 2)\\zeta _{x} c\\x \\begin{pmatrix}a & b \\\\ c & d\\end{pmatrix}",
    );
    assert_eq!(lexer.next_token(), Token::Minus);
    assert_eq!(lexer.next_token(), Token::LParen);
    assert_eq!(lexer.next_token(), Token::Func(FuncName::Re));
    assert_eq!(lexer.next_token(), Token::LParen);
    assert_eq!(lexer.next_token(), Token::Sym("x".to_owned()));
    assert_eq!(lexer.next_token(), Token::RParen);
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
    assert_eq!(lexer.next_token(), Token::MatrixBegin);
    assert_eq!(lexer.next_token(), Token::Sym("a".to_owned()));
    assert_eq!(lexer.next_token(), Token::Ampersand);
    assert_eq!(lexer.next_token(), Token::Sym("b".to_owned()));
    assert_eq!(lexer.next_token(), Token::NewLine);
    assert_eq!(lexer.next_token(), Token::Sym("c".to_owned()));
    assert_eq!(lexer.next_token(), Token::Ampersand);
    assert_eq!(lexer.next_token(), Token::Sym("d".to_owned()));
    assert_eq!(lexer.next_token(), Token::MatrixEnd);
    assert_eq!(lexer.next_token(), Token::Eof);
}
