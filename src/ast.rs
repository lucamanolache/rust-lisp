use std::collections::HashMap;
use std::slice::Iter;

pub enum Expression {
    Atom(f64),
    Expression((String, Vec<Expression>)),
}

#[derive(Debug)]
pub enum LispError {
    IllegalArguements,
}

#[derive(PartialEq)]
enum State {
    Name,
    Args,
}

#[derive(Default)]
pub struct Environment {
    env: HashMap<String, Expression>,
}

fn tokenize(code: String) -> Vec<String> {
    code.replace("(", "( ")
        .replace(")", ") ")
        .split_whitespace()
        .map(|x| x.to_string())
        .collect()
}

pub fn parse(tokens: &[String]) -> Result<Vec<Expression>, LispError> {
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
        match state {
            Name => {
                name = *next;
                state = State::Args;
            }
            Args => {
                if next == "(" {
                    elements.push(read_expression(tokens).unwrap());
                } else if next == ")" {
                    return Ok(Expression::Expression((name, elements)));
                } else {
                    elements.push(Expression::Atom(next.parse().unwrap()));
                }
            }
        }
    }
}
