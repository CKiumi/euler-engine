pub mod lexer;
use crate::{Expr, Sym};
use lexer::{Lexer, Token};
mod serializer;
pub use serializer::serialize;

use self::lexer::Infix;
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
        let mut infix_stack: Vec<Infix> = vec![];
        let mut expr_stack: Vec<Expr> = vec![match first_token {
            Token::Sym(slice) => Expr::Sym(Sym::new(slice)),
            Token::Num(num) => Expr::Num(num),
            Token::Eof => return Expr::Sym(Sym::new("")),
            _ => panic!("Unexpected first token"),
        }];
        loop {
            let next_token = self.lexer.next_token();
            match &next_token {
                token if *token == *end_token => {
                    for _ in 0..infix_stack.len() {
                        self.operate_infix(&mut expr_stack, &mut infix_stack);
                    }
                    break;
                }
                Token::Infix(Infix::Add) => {
                    while let Some(Infix::Circumflex) | Some(Infix::Mul) | Some(Infix::Add) =
                        infix_stack.last()
                    {
                        self.operate_infix(&mut expr_stack, &mut infix_stack);
                    }
                    infix_stack.push(Infix::Add);
                }
                Token::Infix(Infix::Circumflex) => {
                    infix_stack.push(Infix::Circumflex);
                    expr_stack.push(self.parse_arg());
                }
                Token::Infix(Infix::Underscore) => match expr_stack.pop().unwrap() {
                    Expr::Sym(mut sym) => {
                        expr_stack.push(Expr::Sym(sym.set_sub(self.lexer.arg_to_string())))
                    }
                    _ => panic!("Underscore must come after symbol"),
                },
                Token::Sym(sym) => {
                    if expr_stack.len() == infix_stack.len() {
                        expr_stack.push(Expr::Sym(Sym::new(sym)))
                    } else {
                        // Case of implicit mul
                        while let Some(Infix::Circumflex) | Some(Infix::Mul) = infix_stack.last() {
                            self.operate_infix(&mut expr_stack, &mut infix_stack);
                        }
                        expr_stack.push(Expr::Sym(Sym::new(sym)));
                        infix_stack.push(Infix::Mul);
                    }
                }
                Token::Num(num) => match infix_stack.last().unwrap() {
                    Infix::Add => expr_stack.push(Expr::Num(*num)),
                    _ => panic!("Number comes first or after +"),
                },
                _ => unimplemented!(),
            };
        }
        match expr_stack.pop() {
            Some(expr) if expr_stack.is_empty() => expr,
            _ => panic!("expr_stack must contain only one expr at last"),
        }
    }

    fn operate_infix(&self, expr_stack: &mut Vec<Expr>, infix_stack: &mut Vec<Infix>) {
        let right = expr_stack.pop().unwrap();
        let left = expr_stack.pop().unwrap();
        expr_stack.push(match infix_stack.pop().unwrap() {
            Infix::Mul => left * right,
            Infix::Add => left + right,
            Infix::Circumflex => left ^ right,
            _ => unimplemented!(),
        })
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
