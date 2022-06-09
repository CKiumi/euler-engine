pub mod lexer;
use crate::{Expr, Pow, Sym};
use lexer::{Lexer, Token};
mod serializer;
pub use serializer::serialize;
pub struct Parser<'a> {
    pub lexer: Lexer<'a>,
}

impl<'a> Parser<'a> {
    pub fn new(input: &'a str) -> Self {
        let lexer = Lexer::new(input);
        Parser { lexer }
    }
    pub fn parse(&mut self, end_token: &Token) -> Expr<'a> {
        let first_token = self.lexer.next_token();
        let mut expr = match first_token {
            Token::Sym(slice) => Expr::Sym(Sym::new(slice)),
            Token::Num(num) => Expr::Num(num),
            Token::Eof => return Expr::Sym(Sym::new("")),
            _ => panic!("Unexpected first token"),
        };
        let mut last_token = first_token;
        let mut is_last = false;
        while !is_last {
            let next_token = self.lexer.next_token();
            match next_token {
                token if token == *end_token => is_last = true,
                Token::AddInfix => {
                    last_token = Token::AddInfix;
                }
                Token::Circumflex => {
                    expr = match expr {
                        Expr::Sym(sym) => Expr::Pow(Pow::new(Expr::Sym(sym), self.parse_arg())),
                        _ => unimplemented!(),
                    };
                }
                Token::UnderScore => {
                    expr = match last_token {
                        Token::Sym(sym) => {
                            Expr::Sym(Sym::new(sym).set_sub(self.lexer.arg_to_string()))
                        }
                        _ => unimplemented!(),
                    };
                }
                Token::Sym(slice) => {
                    let sym = Expr::Sym(Sym::new(slice));
                    if let Token::AddInfix = last_token {
                        expr = expr + sym;
                    } else {
                        expr = match expr {
                            Expr::Add(mut add) => {
                                let last = add.exprs.pop().unwrap();
                                Expr::Add(add) + last * sym
                            }
                            expr => expr * sym,
                        }
                    }
                    last_token = next_token;
                }
                Token::Num(num) => {
                    if let Token::AddInfix = last_token {
                        expr = expr + Expr::Num(num);
                    } else {
                        panic!("Number comes first or after +");
                    }
                    last_token = next_token;
                }

                _ => unimplemented!(),
            };
        }
        expr
    }

    fn parse_arg(&mut self) -> Expr<'a> {
        match self.lexer.next_token() {
            Token::LCurlyBrace => self.parse(&Token::RCurlyBrace),
            _ => unimplemented!(),
        }
    }
}

pub fn latex_to_expr(latex: &str) -> Expr {
    let mut parser = Parser::new(latex);
    parser.parse(&Token::Eof)
}
#[test]
fn test_parser() {
    assert_eq!(latex_to_expr("aaaa").to_string(), "a*a*a*a");
    assert_eq!(latex_to_expr("a+a+a").to_string(), "a+a+a");
    assert_eq!(latex_to_expr("a+bc").to_string(), "a+b*c");
    assert_eq!(
        latex_to_expr("ab+b\\alpha sdas+x").to_string(),
        "a*b+b*\\alpha*s*d*a*s+x"
    );
    assert_eq!(latex_to_expr("").to_string(), "");
    assert_eq!(latex_to_expr("b^{a}").to_string(), "b^{a}");
    assert_eq!(latex_to_expr("b_{a}").to_string(), "b_{a}");
    assert_eq!(latex_to_expr("b_{a}^{c}").to_string(), "b_{a}^{c}");
    assert_eq!(latex_to_expr("2a").to_string(), "2*a");
    assert_eq!(latex_to_expr("23a").to_string(), "23*a");
    assert_eq!(latex_to_expr("23a+23a").to_string(), "23*a+23*a");
    assert_eq!(latex_to_expr("x^2+x^2").to_string(), "x^2+x^2");
}
