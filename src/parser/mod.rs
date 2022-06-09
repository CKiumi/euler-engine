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
            match &next_token {
                token if *token == *end_token => is_last = true,
                Token::AddInfix => {}
                Token::Circumflex => {
                    expr = match expr {
                        Expr::Sym(sym) => Expr::Sym(sym) ^ self.parse_arg(),
                        Expr::Add(mut add) => {
                            let last = add.exprs.pop().unwrap();
                            match last {
                                Expr::Mul(mut mul) => {
                                    let last = mul.exprs.pop().unwrap();
                                    Expr::Add(add + mul * Pow::new(last, self.parse_arg()))
                                }
                                _ => Expr::Add(add + Pow::new(last, self.parse_arg())),
                            }
                        }
                        Expr::Mul(mut mul) => {
                            let last = mul.exprs.pop().unwrap();
                            Expr::Mul(mul * Pow::new(last, self.parse_arg()))
                        }
                        _ => unimplemented!(),
                    };
                }
                Token::UnderScore => {
                    expr = match expr {
                        Expr::Sym(mut sym) => Expr::Sym(sym.set_sub(self.lexer.arg_to_string())),
                        Expr::Mul(mut mul) => {
                            let last = mul.exprs.pop().unwrap();
                            match last {
                                Expr::Sym(mut sym) => {
                                    Expr::Mul(mul)
                                        * Expr::Sym(sym.set_sub(self.lexer.arg_to_string()))
                                }
                                _ => unimplemented!(),
                            }
                        }
                        Expr::Add(mut add) => {
                            let last = add.exprs.pop().unwrap();
                            match last {
                                Expr::Sym(mut sym) => {
                                    Expr::Add(add)
                                        + Expr::Sym(sym.set_sub(self.lexer.arg_to_string()))
                                }
                                Expr::Mul(mut mul) => {
                                    let last = mul.exprs.pop().unwrap();
                                    match last {
                                        Expr::Sym(mut sym) => {
                                            Expr::Add(add)
                                                + Expr::Mul(mul)
                                                    * Expr::Sym(
                                                        sym.set_sub(self.lexer.arg_to_string()),
                                                    )
                                        }
                                        _ => panic!("Underscore must come after symbol"),
                                    }
                                }
                                _ => panic!("Underscore must come after symbol"),
                            }
                        }
                        _ => panic!("Underscore must come after symbol"),
                    };
                }
                Token::Sym(slice) => {
                    let sym = Expr::Sym(Sym::new(slice));
                    expr = match last_token {
                        Token::AddInfix => expr + sym,
                        _ => match expr {
                            Expr::Add(mut add) => {
                                let last = add.exprs.pop().unwrap();
                                Expr::Add(add) + last * sym
                            }
                            expr => expr * sym,
                        },
                    }
                }
                Token::Num(num) => match last_token {
                    Token::AddInfix => expr = expr + Expr::Num(*num),
                    _ => panic!("Number comes first or after +"),
                },

                _ => unimplemented!(),
            };
            last_token = next_token;
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
    let tests = [
        ["aaaa", "a*a*a*a"],
        ["a+a+a", "a+a+a"],
        ["a+bc", "a+b*c"],
        ["ab+b\\alpha sdas+x", "a*b+b*\\alpha*s*d*a*s+x"],
        ["aaaa", "a*a*a*a"],
        ["", ""],
        ["b^{a}", "b^{a}"],
        ["b_{a}", "b_{a}"],
        ["b_{a}^{c}", "b_{a}^{c}"],
        ["2a", "2*a"],
        ["23a", "23*a"],
        ["23a+23a", "23*a+23*a"],
        ["x^{2}+x^{2}", "x^{2}+x^{2}"],
        ["2x^{2}+2x^{2}", "2*x^{2}+2*x^{2}"],
        ["2x^{x+y}+2x^{xy}", "2*x^{x+y}+2*x^{x*y}"],
        ["2x_{2}^{2}", "2*x_{2}^{2}"],
        ["a_{b}^{c}+d_{e}^{f}", "a_{b}^{c}+d_{e}^{f}"],
        ["a_{b}^{c}+xd_{e}^{f}", "a_{b}^{c}+x*d_{e}^{f}"],
    ];
    tests.iter().for_each(|test| {
        assert_eq!(latex_to_expr(test[0]).to_string(), test[1]);
    });
}
