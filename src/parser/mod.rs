pub mod lexer;
use crate::{Add, Expr, Mul, Num, Pow, Sym};
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
        let mut expr = Expr::Num(Num::new(1));
        let mut last_token = Token::Eof;
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
                    if let Token::AddInfix = last_token {
                        expr = match expr {
                            Expr::Sym(prev) => Expr::Add(prev + Sym::new(slice)),
                            Expr::Add(prev) => Expr::Add(prev + Sym::new(slice)),
                            Expr::Mul(prev) => Expr::Add(prev + Sym::new(slice)),
                            _ => unimplemented!(),
                        }
                    } else {
                        expr = match expr {
                            Expr::Sym(prev) => Expr::Mul(prev * Sym::new(slice)),
                            Expr::Mul(mul) => Expr::Mul(mul * Sym::new(slice)),
                            Expr::Add(mut add) => {
                                let last = add.exprs.pop().unwrap();
                                Expr::Add(
                                    Add::new(add.exprs) + Mul::new(vec![last]) * Sym::new(slice),
                                )
                            }
                            _ => Expr::Sym(Sym::new(slice)),
                        }
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
    println!("{}", latex_to_expr("aaaa"));
    println!("{}", latex_to_expr("a+a+a"));
    println!("{}", latex_to_expr("a+bc"));
    println!("{}", latex_to_expr("ab+b\\alpha sdas+x"));
    println!("{}", latex_to_expr("b^{a}"));
    println!("{}", latex_to_expr("b_{a}"));
    println!("{}", latex_to_expr("b_{a}^{c}"));
}
