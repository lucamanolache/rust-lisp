use std::fmt::Display;
use std::slice::Iter;

pub enum Expression {
    Num(f64),
    Bool(bool),
    String(String),
    List(Vec<Expression>),
    Func((String, Vec<Expression>)),
}

impl Display for Expression {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Expression::Num(n) => {
                write!(f, "({})", n)
            }
            Expression::Func((_, _)) => {
                write!(f, "()")
            }
            Expression::List(l) => {
                write!(f, "(");
                let mut iter = l.iter();
                match iter.next() {
                    None => {
                        return write!(f, ")");
                    }
                    Some(e) => {
                        write!(f, " {}", e);
                    }
                }
                for i in iter {
                    write!(f, ", {}", i);
                }
                write!(f, " )")
            }
            Expression::String(s) => {
                write!(f, "({})", s)
            }
            Expression::Bool(b) => {
                write!(f, "({})", b)
            }
        }
    }
}

#[derive(Debug)]
pub enum LispError {
    IllegalArguements,
}

enum State {
    Name,
    Args,
}

fn tokenize(code: String) -> Vec<String> {
    code.replace("(", " ( ")
        .replace(")", " ) ")
        .split_whitespace()
        .map(|x| x.to_string())
        .collect()
}

pub fn parse(code: String) -> Result<Vec<Expression>, LispError> {
    let tokens = tokenize(code);
    let mut tokens = tokens.iter();
    let mut expressions = Vec::new();
    loop {
        let next = tokens.next();
        if next.is_none() {
            return Ok(expressions);
        }
        if next.unwrap() == "(" {
            expressions.push(read_expression(&mut tokens).unwrap());
        }
    }
}

fn read_expression(tokens: &mut Iter<String>) -> Result<Expression, LispError> {
    let mut state = State::Name;
    let mut elements = Vec::<Expression>::new();
    let mut name = String::new();
    loop {
        let next = tokens.next();
        if next.is_none() {
            return Err(LispError::IllegalArguements); // TODO: this is matching parenthesis error, not illegal args
        }
        let next = next.unwrap();
        match &state {
            State::Name => {
                name = next.clone();
                state = State::Args;
            }
            State::Args => {
                if next == "(" {
                    elements.push(read_expression(tokens).unwrap());
                } else if next == ")" {
                    return Ok(Expression::Func((name, elements)));
                } else {
                    elements.push(match next.parse::<f64>() {
                        Err(_) => match next.parse::<bool>() {
                            Err(_) => Expression::String(next.to_string()),
                            Ok(n) => Expression::Bool(n),
                        },
                        Ok(n) => Expression::Num(n),
                    });
                }
            }
        }
    }
}
