use tracing::{debug, trace};

#[derive(Debug, Clone, PartialEq)]
pub enum Token {
    Number(f64),
    Operator(char),
    Function(String),
    LeftParen,
    RightParen,
}

pub fn tokenize(input: &str) -> Result<Vec<Token>, String> {
    let mut tokens = Vec::new();
    let mut chars = input.chars().peekable();

    while let Some(&c) = chars.peek() {
        trace!("peek: {}", c);
        match c {
            '0'..='9' | '.' => {
                let mut number = String::new();
                while let Some(&c) = chars.peek() {
                    if c.is_digit(10) || c == '.' {
                        number.push(c);
                        chars.next();
                    } else if c == 'e' || c == 'E' {
                        number.push(c);
                        chars.next();
                        if let Some(&c) = chars.peek() {
                            if c == '-' || c == '+' {
                                number.push(c);
                                chars.next();
                            }
                        }
                    } else {
                        trace!("got char: {}", c);
                        break;
                    }
                }
                debug!("number: {}", number);
                tokens.push(Token::Number(number.parse().map_err(|err| 
                    format!("{}", err))?));
            },
            '+' | '-' | '*' | '/' => {
                trace!("got operator: {}", c);
                tokens.push(Token::Operator(c));
                chars.next();
            },
            '(' => {
                trace!("got left paren: {}", c);
                tokens.push(Token::LeftParen);
                chars.next();
            },
            ')' => {
                trace!("got right paren: {}", c);
                tokens.push(Token::RightParen);
                chars.next();
            },
            'a'..='z' | 'A'..='Z' => {
                trace!("got function: {}", c);
                let mut function = String::new();
                while let Some(&c) = chars.peek() {
                    if c.is_alphabetic() {
                        function.push(c);
                        chars.next();
                    } else {
                        break;
                    }
                }
                tokens.push(Token::Function(function));
            },
            ' ' | '\t' | '\n' | '\r' => { chars.next(); },
            _ => return Err(format!("Caractère inattendu : {}", c)),
        }
    }

    Ok(tokens)
}
