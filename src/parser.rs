use crate::tokenizer::Token;
use std::iter::Peekable;
use std::vec::IntoIter;

#[derive(Debug, Clone)]
pub enum Expr {
    Number(f64),
    UnaryOp(char, Box<Expr>),
    BinaryOp(Box<Expr>, char, Box<Expr>),
    Function(String, Box<Expr>),
}

pub fn parse(tokens: Vec<Token>) -> Result<Expr, String> {
    Parser::new(tokens).parse()
}

struct Parser {
    tokens: Peekable<IntoIter<Token>>,
}

impl Parser {
    pub fn new(tokens: Vec<Token>) -> Self {
        Self { tokens: tokens.into_iter().peekable() }
    }

    pub fn peek(&mut self) -> Option<&Token> {
        self.tokens.peek()
    }

    pub fn next(&mut self) -> Option<Token> {
        self.tokens.next()
    }

    pub fn parse(&mut self) -> Result<Expr, String> {
        self.parse_expression()
    }

    fn parse_expression(&mut self) -> Result<Expr, String> {
        let mut left = self.parse_term()?;

        while let Some(Token::Operator(op)) = self.peek() {
            let op_char = *op;
            if matches!(op_char, '+' | '-') {
                self.next();
                let right = self.parse_term()?;
                left = Expr::BinaryOp(Box::new(left), op_char, Box::new(right));
            } else {
                break;
            }
        }

        Ok(left)
    }

    fn parse_term(&mut self) -> Result<Expr, String> {
        let mut left = self.parse_factor()?;

        while let Some(Token::Operator(op)) = self.peek() {
            let op_char = *op;  
            if matches!(op_char, '*' | '/') {
                self.next();
                let right = self.parse_factor()?;
                left = Expr::BinaryOp(Box::new(left), op_char, Box::new(right));
            } else {
                break;
            }
        }

        Ok(left)
    }

    fn parse_factor(&mut self) -> Result<Expr, String> {
        let expr = match self.next() {
            Some(Token::Number(n)) => Expr::Number(n),
            Some(Token::Operator('+')) => {
                let inner_expr = self.parse_factor()?;
                Expr::UnaryOp('+', Box::new(inner_expr))
            }
            Some(Token::Operator('-')) => {
                let inner_expr = self.parse_factor()?;
                Expr::UnaryOp('-', Box::new(inner_expr))
            }
            Some(Token::Function(name)) => {
                let arg = self.parse_factor()?;
                Expr::Function(name, Box::new(arg))
            }
            Some(Token::LeftParen) => {
                let inner_expr = self.parse_expression()?;
                match self.next() {
                    Some(Token::RightParen) => inner_expr,
                    _ => return Err("parenthèse fermante manquante".to_string()),
                }
            }
            Some(token) => return Err(format!("unexpected token: {:?}", token)),
            None => return Err("unexpected end of input".to_string()),
        };

        Ok(expr)
    }
}

