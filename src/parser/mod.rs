pub mod lexer;
use crate::{
    expr::{Frac, Func, FuncName, Gate, GateName, Ket, Mat, Par, Tensor},
    Expr, Num, Pow, Sym,
};
use lexer::{Lexer, Token};
mod serializer;
mod sympy;
pub use serializer::serialize;
pub use sympy::to_sympy;

use self::lexer::Infix;
pub struct Parser<'a> {
    pub lexer: Lexer<'a>,
    new_line: bool,
    end_matrix: bool,
}

impl<'a> Parser<'a> {
    pub fn new(input: &'a str) -> Self {
        let lexer = Lexer::new(input);
        Parser {
            lexer,
            new_line: false,
            end_matrix: false,
        }
    }
    pub fn parse(&mut self, end_token: &Token) -> Expr {
        let first_token = self.lexer.next_token();
        let mut infix_stack: Vec<Infix> = vec![];
        let mut expr_stack: Vec<Expr> = vec![match first_token {
            Token::Sym(slice) => self.handle_pre_defined(Sym::new(slice)),
            Token::Num(num) => Expr::Num(num),
            Token::LParen => Expr::Par(Par::new(self.parse(&Token::RParen))),
            Token::Bar => Expr::Ket(Ket::new(self.lexer.arg_to_string())),
            Token::Func(func) => self.parse_func(func),
            Token::Frac => Expr::Frac(Frac::new(self.parse_arg(), self.parse_arg())),
            Token::Minus => Expr::Num(Num::new(-1)),
            Token::MatrixBegin => self.parse_mat(),
            Token::Eof => return Expr::Sym(Sym::new("")),
            Token::NewLine | Token::Ampersand | Token::MatrixEnd => {
                panic!("Empty element for matrix is not allowed")
            }
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
                Token::NewLine => {
                    if let Token::Ampersand = end_token {
                        self.new_line = true;
                        for _ in 0..infix_stack.len() {
                            self.operate_infix(&mut expr_stack, &mut infix_stack);
                        }
                        break;
                    }
                }
                Token::MatrixEnd => {
                    if let Token::Ampersand = end_token {
                        self.end_matrix = true;
                        for _ in 0..infix_stack.len() {
                            self.operate_infix(&mut expr_stack, &mut infix_stack);
                        }
                        break;
                    }
                }
                Token::Bar => {
                    if expr_stack.len() == infix_stack.len() {
                        expr_stack.push(Expr::Ket(Ket::new(self.lexer.arg_to_string())));
                    } else {
                        // Case of implicit mul
                        while let Some(x) = infix_stack.last() && *x >= Infix::Mul {
                            self.operate_infix(&mut expr_stack, &mut infix_stack);
                        }
                        expr_stack.push(Expr::Ket(Ket::new(self.lexer.arg_to_string())));
                        infix_stack.push(Infix::Mul);
                    }
                }
                Token::LParen => {
                    if expr_stack.len() == infix_stack.len() {
                        expr_stack.push(Expr::Par(Par::new(self.parse(&Token::RParen))));
                    } else {
                        // Case of implicit mul
                        while let Some(x) = infix_stack.last() && *x >= Infix::Mul {
                            self.operate_infix(&mut expr_stack, &mut infix_stack);
                        }
                        expr_stack.push(Expr::Par(Par::new(self.parse(&Token::RParen))));
                        infix_stack.push(Infix::Mul);
                    }
                }
                Token::Minus => {
                    while let Some(x) = infix_stack.last() && *x > Infix::Add {
                        self.operate_infix(&mut expr_stack, &mut infix_stack);
                    }
                    infix_stack.push(Infix::Add);
                    expr_stack.push(Expr::Num(Num::new(-1)));
                }
                Token::Infix(Infix::Add) => {
                    while let Some(x) = infix_stack.last() && *x > Infix::Add {
                        self.operate_infix(&mut expr_stack, &mut infix_stack);
                    }
                    infix_stack.push(Infix::Add);
                }
                Token::Infix(Infix::Tensor) => {
                    while let Some(x) = infix_stack.last() && *x > Infix::Tensor {
                        self.operate_infix(&mut expr_stack, &mut infix_stack);
                    }
                    infix_stack.push(Infix::Tensor);
                }
                Token::Infix(Infix::Circumflex) => {
                    infix_stack.push(Infix::Circumflex);
                    expr_stack.push(self.parse_arg());
                }
                Token::Infix(Infix::Underscore) => match expr_stack.pop().unwrap() {
                    Expr::Sym(mut sym) => {
                        self.lexer.read_char();
                        sym.set_sub(self.lexer.arg_to_string());
                        expr_stack.push(Expr::Sym(sym));
                    }
                    Expr::Gate(gate) => match gate.name {
                        GateName::CNOT(_, _) | GateName::CZ(_, _) => {
                            self.lexer.read_char();
                            let parts: Vec<String> = self
                                .lexer
                                .arg_to_string()
                                .split(',')
                                .map(|s| s.to_string())
                                .collect();
                            dbg!(&parts);
                            expr_stack.push(Expr::Gate(gate.change_qbits(
                                parts[0].parse::<u32>().unwrap(),
                                parts[1].parse::<u32>().unwrap(),
                            )))
                        }
                        _ => {
                            self.lexer.read_char();
                            let qbit = self.lexer.arg_to_string().parse::<u32>().unwrap();
                            expr_stack.push(Expr::Gate(gate.change_qbit(qbit)));
                        }
                    },
                    _ => panic!("Underscore must come after symbol"),
                },
                Token::Sym(sym) => {
                    self.handle_implicit_mul(
                        &mut expr_stack,
                        &mut infix_stack,
                        self.handle_pre_defined(Sym::new(sym)),
                    );
                }
                Token::Num(num) => match infix_stack.last() {
                    Some(Infix::Add) => match expr_stack.last().unwrap() {
                        Expr::Num(x) if x.num == -1 => {
                            expr_stack.pop();
                            expr_stack.push(Expr::Num(Num::new(-num.num)));
                        }
                        _ => expr_stack.push(Expr::Num(*num)),
                    },
                    //start with negative number
                    None => expr_stack[0] = expr_stack[0].clone() * Expr::Num(*num),
                    _ => panic!("Number comes first or after + -"),
                },
                Token::Func(func) => {
                    let expr = self.parse_func(*func);
                    self.handle_implicit_mul(&mut expr_stack, &mut infix_stack, expr);
                }
                Token::Frac => {
                    let expr = Expr::Frac(Frac::new(self.parse_arg(), self.parse_arg()));
                    self.handle_implicit_mul(&mut expr_stack, &mut infix_stack, expr);
                }
                Token::MatrixBegin => {
                    let expr = self.parse_mat();
                    self.handle_implicit_mul(&mut expr_stack, &mut infix_stack, expr);
                }
                Token::Eof => panic!("Unexpected end of input"),
                _ => unimplemented!(),
            };
        }
        match expr_stack.pop() {
            Some(expr) if expr_stack.is_empty() => expr,
            _ => panic!("expr_stack must contain only one expr at last"),
        }
    }

    fn handle_pre_defined(&self, sym: Sym) -> Expr {
        match sym.symbol.as_str() {
            "H" => Expr::Gate(Gate::new(GateName::H(0))),
            "X" => Expr::Gate(Gate::new(GateName::X(0))),
            "Y" => Expr::Gate(Gate::new(GateName::Y(0))),
            "Z" => Expr::Gate(Gate::new(GateName::Z(0))),
            "S" => Expr::Gate(Gate::new(GateName::S(0))),
            "T" => Expr::Gate(Gate::new(GateName::T(0))),
            "I" => Expr::Gate(Gate::new(GateName::I(0))),
            "CNOT" => Expr::Gate(Gate::new(GateName::CNOT(0, 1))),
            "CZ" => Expr::Gate(Gate::new(GateName::CZ(0, 1))),
            _ => Expr::Sym(sym),
        }
    }

    fn handle_implicit_mul(
        &self,
        expr_stack: &mut Vec<Expr>,
        infix_stack: &mut Vec<Infix>,
        expr: Expr,
    ) {
        if expr_stack.len() == infix_stack.len() {
            expr_stack.push(expr)
        } else {
            // Case of implicit mul
            while let Some(Infix::Circumflex) | Some(Infix::Mul) = infix_stack.last() {
                self.operate_infix(expr_stack, infix_stack);
            }
            expr_stack.push(expr);
            infix_stack.push(Infix::Mul);
        }
    }

    fn operate_infix(&self, expr_stack: &mut Vec<Expr>, infix_stack: &mut Vec<Infix>) {
        let right = expr_stack.pop().unwrap();
        let left = expr_stack.pop().unwrap();
        expr_stack.push(match infix_stack.pop().unwrap() {
            Infix::Mul => left * right,
            Infix::Add => left + right,
            Infix::Circumflex => left ^ right,
            Infix::Tensor => Expr::Tensor(Tensor::new(vec![left, right])),
            _ => unimplemented!(),
        })
    }

    fn parse_arg(&mut self) -> Expr {
        match self.lexer.next_token() {
            Token::LCurlyBrace => self.parse(&Token::RCurlyBrace),
            _ => unimplemented!(),
        }
    }

    fn parse_func(&mut self, name: FuncName) -> Expr {
        if let FuncName::Sqrt = name {
            return Expr::Func(Func::new(name, self.parse_arg()));
        }
        let token = self.lexer.next_token();
        match token {
            Token::LParen => Expr::Func(Func::new(name, self.parse(&Token::RParen))),
            Token::Infix(Infix::Circumflex) => {
                let pow = self.parse_arg();
                self.lexer.next_token();
                let func = Expr::Func(Func::new(name, self.parse(&Token::RParen)));
                Expr::Pow(Pow::new(func, pow))
            }
            _ => unimplemented!(),
        }
    }

    fn parse_mat(&mut self) -> Expr {
        let mut mat = Vec::new();
        loop {
            let mut row = Vec::new();
            loop {
                row.push(self.parse(&Token::Ampersand));
                if self.new_line || self.end_matrix {
                    self.new_line = false;
                    break;
                }
            }
            mat.push(row);
            if self.end_matrix {
                self.end_matrix = false;
                break;
            }
        }
        Expr::Mat(Mat::new(mat))
    }
}

pub fn latex_to_expr(latex: &str) -> Expr {
    let latex = latex.replace("\\left", "").replace("\\right", "");
    let mut parser = Parser::new(latex.as_str());
    parser.parse(&Token::Eof)
}

#[test]
fn test_parser() {
    use super::expr::test_util::asrt;
    let tests = [
        ["aaaa", "a*a*a*a"],
        ["a+a+a", "a+a+a"],
        ["-a+b", "-a+b"],
        ["x-(a+b)", "x-(a+b)"],
        ["a-b", "a-b"],
        ["a-1", "a-1"],
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
        ["(a+b)", "(a+b)"],
        ["\\left(a+b\\right)", "(a+b)"],
        ["\\left(a+b\\right)+\\left(a+b\\right)", "(a+b)+(a+b)"],
        ["\\left(a+b\\right)\\left(a+b\\right)", "(a+b)*(a+b)"],
        ["\\left(a+b\\right)^{2}", "(a+b)^{2}"],
        ["\\Re(a+b)", "Re(a+b)"],
        ["\\Re^{2}(a+b)", "Re^{2}(a+b)"],
        ["x\\Re^{x+y}(a+b)y", "x*Re^{x+y}(a+b)*y"],
        ["\\sqrt{2}", "sqrt(2)"],
        ["\\sqrt{2}^{2}", "sqrt^{2}(2)"],
        ["\\frac{2}{2}", "frac(2)(2)"],
        ["a\\frac{2}{2}+b", "a*frac(2)(2)+b"],
        [
            "\\begin{pmatrix}a+ b & b \\\\ c & d\\end{pmatrix}b",
            "mat([[a+b, b], [c, d]])*b",
        ],
        ["\\left|00\\right>", "|00>"],
        ["H_{0}\\left|00\\right>", "H(0)*|00>"],
        ["H_{2}", "H(2)"],
        ["H", "H(0)"],
        ["\\operatorname{CNOT}_{1,2}", "CNOT(1, 2)"],
        ["\\operatorname{CZ}_{0,1}", "CZ(0, 1)"],
    ];
    tests.iter().for_each(|test| {
        asrt(latex_to_expr(test[0]), test[1]);
    });
}
