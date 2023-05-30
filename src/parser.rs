use crate::ast::{parse, Expression, LispError};
use std::collections::HashMap;

type Implementation = fn(Vec<Expression>, &mut Environment) -> Result<Expression, LispError>;

pub struct Environment {
    env: HashMap<String, Implementation>,
}

impl Default for Environment {
    fn default() -> Self {
        let map: HashMap<String, Implementation> = HashMap::new();
        let mut s = Self { env: map };

        s.env.insert(
            "+".to_string(),
            |args: Vec<Expression>, env: &mut Environment| -> Result<Expression, LispError> {
                let mut ret = 0.0;
                for v in args.into_iter() {
                    match run_exp(v, env) {
                        Ok(v) => match v {
                            Expression::Num(n) => {
                                ret += n;
                            }
                            _ => {}
                        },
                        Err(e) => return Err(e),
                    }
                }
                Ok(Expression::Num(ret))
            },
        );

        s.env.insert(
            "*".to_string(),
            |args: Vec<Expression>, env: &mut Environment| -> Result<Expression, LispError> {
                let mut ret = 1.0;
                for v in args.into_iter() {
                    match run_exp(v, env) {
                        Ok(v) => match v {
                            Expression::Num(n) => {
                                ret *= n;
                            }
                            _ => {}
                        },
                        Err(e) => return Err(e),
                    }
                }
                Ok(Expression::Num(ret))
            },
        );

        s.env.insert(
            "-".to_string(),
            |args: Vec<Expression>, env: &mut Environment| -> Result<Expression, LispError> {
                let len = args.len();
                let mut iter = args.into_iter();
                let mut ret = match iter.next() {
                    None => return Err(LispError::IllegalArguements),
                    Some(ret) => match run_exp(ret, env) {
                        Ok(v) => match v {
                            Expression::Num(n) => n,
                            _ => {
                                return Err(LispError::IllegalArguements);
                            }
                        },
                        Err(e) => return Err(e),
                    },
                };
                if len == 1 {
                    return Ok(Expression::Num(-ret));
                }
                for v in iter {
                    match run_exp(v, env) {
                        Ok(v) => match v {
                            Expression::Num(n) => {
                                ret -= n;
                            }
                            _ => {}
                        },
                        Err(e) => return Err(e),
                    }
                }
                Ok(Expression::Num(ret))
            },
        );

        s.env.insert(
            "/".to_string(),
            |args: Vec<Expression>, env: &mut Environment| -> Result<Expression, LispError> {
                let mut iter = args.into_iter();
                let mut ret = match iter.next() {
                    None => return Err(LispError::IllegalArguements),
                    Some(ret) => match run_exp(ret, env) {
                        Ok(v) => match v {
                            Expression::Num(n) => n,
                            _ => {
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
                            _ => {}
                        },
                        Err(e) => return Err(e),
                    }
                }
                Ok(Expression::Num(ret))
            },
        );

        s.env.insert(
            "<".to_string(),
            |args: Vec<Expression>, env: &mut Environment| -> Result<Expression, LispError> {
                if args.len() != 2 {
                    return Err(LispError::IllegalArguements);
                }
                let mut args = args.into_iter();
                let p1 = match run_exp(args.next().unwrap(), env)? {
                    Expression::Num(n) => n,
                    _ => return Err(LispError::IllegalArguements),
                };
                let p2 = match run_exp(args.next().unwrap(), env)? {
                    Expression::Num(n) => n,
                    _ => return Err(LispError::IllegalArguements),
                };

                Ok(Expression::Bool(p1 < p2))
            },
        );

        s.env.insert(
            ">".to_string(),
            |args: Vec<Expression>, env: &mut Environment| -> Result<Expression, LispError> {
                if args.len() != 2 {
                    return Err(LispError::IllegalArguements);
                }
                let mut args = args.into_iter();
                let p1 = match run_exp(args.next().unwrap(), env)? {
                    Expression::Num(n) => n,
                    _ => return Err(LispError::IllegalArguements),
                };
                let p2 = match run_exp(args.next().unwrap(), env)? {
                    Expression::Num(n) => n,
                    _ => return Err(LispError::IllegalArguements),
                };

                Ok(Expression::Bool(p1 > p2))
            },
        );

        return s;
    }
}

pub fn run(code: String) -> Vec<Result<Expression, LispError>> {
    let mut env = Environment::default();
    let expressions = parse(code).unwrap();

    let mut out = Vec::new();

    for exp in expressions.into_iter() {
        let ret = run_exp(exp, &mut env);
        out.push(ret);
        match out.last().unwrap() {
            Ok(r) => println!("{}", r),
            Err(e) => println!("Error: {:?}", e),
        };
    }

    out
}

fn run_exp(exp: Expression, env: &mut Environment) -> Result<Expression, LispError> {
    match exp {
        Expression::Func((name, params)) => {
            match (*env).env.get(&name) {
                None => {
                    Err(LispError::IllegalArguements) // TODO: get right error here
                }
                Some(f) => match f(params, env) {
                    Err(e) => Err(e),
                    Ok(e) => Ok(e),
                },
            }
        }
        _ => Ok(exp),
    }
}
