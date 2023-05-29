use crate::ast::{parse, Expression, LispError};
use std::collections::HashMap;

type Implementation = fn(&Vec<Expression>, &mut Environment) -> Result<Expression, LispError>;

pub struct Environment {
    env: HashMap<String, Implementation>,
}

impl Default for Environment {
    fn default() -> Self {
        let map: HashMap<String, Implementation> = HashMap::new();
        let mut s = Self { env: map };

        s.env.insert(
            "+".to_string(),
            |args: &Vec<Expression>, env: &mut Environment| -> Result<Expression, LispError> {
                let mut ret = 0.0;
                for v in args.iter() {
                    match run_exp(v, env) {
                        Ok(v) => match v {
                            Expression::Num(n) => {
                                ret += n;
                            }
                            Expression::Func(_) => {}
                        },
                        Err(e) => return Err(e),
                    }
                }
                Ok(Expression::Num(ret))
            },
        );

        s.env.insert(
            "*".to_string(),
            |args: &Vec<Expression>, env: &mut Environment| -> Result<Expression, LispError> {
                let mut ret = 1.0;
                for v in args.iter() {
                    match run_exp(v, env) {
                        Ok(v) => match v {
                            Expression::Num(n) => {
                                ret *= n;
                            }
                            Expression::Func(_) => {}
                        },
                        Err(e) => return Err(e),
                    }
                }
                Ok(Expression::Num(ret))
            },
        );

        s.env.insert(
            "-".to_string(),
            |args: &Vec<Expression>, env: &mut Environment| -> Result<Expression, LispError> {
                let mut iter = args.iter();
                let mut ret = match iter.next() {
                    None => return Err(LispError::IllegalArguements),
                    Some(ret) => match run_exp(ret, env) {
                        Ok(v) => match v {
                            Expression::Num(n) => n,
                            Expression::Func(_) => {
                                return Err(LispError::IllegalArguements);
                            }
                        },
                        Err(e) => return Err(e),
                    },
                };
                for v in iter {
                    match run_exp(v, env) {
                        Ok(v) => match v {
                            Expression::Num(n) => {
                                ret -= n;
                            }
                            Expression::Func(_) => {}
                        },
                        Err(e) => return Err(e),
                    }
                }
                Ok(Expression::Num(ret))
            },
        );

        s.env.insert(
            "/".to_string(),
            |args: &Vec<Expression>, env: &mut Environment| -> Result<Expression, LispError> {
                let mut iter = args.iter();
                let mut ret = match iter.next() {
                    None => return Err(LispError::IllegalArguements),
                    Some(ret) => match run_exp(ret, env) {
                        Ok(v) => match v {
                            Expression::Num(n) => n,
                            Expression::Func(_) => {
                                return Err(LispError::IllegalArguements);
                            }
                        },
                        Err(e) => return Err(e),
                    },
                };
                for v in iter {
                    match run_exp(v, env) {
                        Ok(v) => match v {
                            Expression::Num(n) => {
                                ret /= n;
                            }
                            Expression::Func(_) => {}
                        },
                        Err(e) => return Err(e),
                    }
                }
                Ok(Expression::Num(ret))
            },
        );

        s.env.insert(
            "define".to_string(),
            |args: &Vec<Expression>, env: &mut Environment| -> Result<Expression, LispError> {
                let mut iter = args.iter();
                let mut ret = match iter.next() {
                    None => return Err(LispError::IllegalArguements),
                    Some(ret) => match run_exp(ret, env) {
                        Ok(v) => match v {
                            Expression::Num(n) => n,
                            Expression::Func(_) => {
                                return Err(LispError::IllegalArguements);
                            }
                        },
                        Err(e) => return Err(e),
                    },
                };
                for v in iter {
                    match run_exp(v, env) {
                        Ok(v) => match v {
                            Expression::Num(n) => {
                                ret /= n;
                            }
                            Expression::Func(_) => {}
                        },
                        Err(e) => return Err(e),
                    }
                }
                Ok(Expression::Num(ret))
            },
        );

        return s;
    }
}

pub fn run(code: String) {
    let mut env = Environment::default();
    let expressions = parse(code).unwrap();

    for exp in expressions.iter() {
        println!("{}", run_exp(exp, &mut env).unwrap());
    }
}

fn run_exp(exp: &Expression, env: &mut Environment) -> Result<Expression, LispError> {
    match exp {
        Expression::Num(n) => {
            return Ok(Expression::Num(*n));
        }
        Expression::Func((name, params)) => {
            match (*env).env.get(name) {
                None => {
                    Err(LispError::IllegalArguements) // TODO: get right error here
                }
                Some(f) => f(&params, env),
            }
        }
    }
}
