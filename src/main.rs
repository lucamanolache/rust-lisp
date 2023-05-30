use crate::parser::run;
use std::io;

mod ast;
mod parser;

fn main() {
    let mut inp = String::new();

    while inp != "q" {
        inp.clear();
        io::stdin().read_line(&mut inp).unwrap();
        run(inp.clone());
    }
}

#[cfg(test)]
mod tests {
    use crate::{ast::Expression, parser::run};

    #[test]
    fn minus() {
        let zero = "(- 1 1)".to_owned();
        let neg = "(- 1)".to_owned();
        assert_eq!(
            run(zero).get(0).unwrap().as_ref().unwrap(),
            &Expression::Num(0.0)
        );
        assert_eq!(
            run(neg).get(0).unwrap().as_ref().unwrap(),
            &Expression::Num(-1.0)
        );
    }

    #[test]
    fn add_1_1() {
        let code = "(+ 1 1)".to_owned();
        assert_eq!(
            run(code).get(0).unwrap().as_ref().unwrap(),
            &Expression::Num(2.0)
        );
    }

    #[test]
    fn pemdas() {
        let code = "(+ 1 1 (* 3 3) (- 2 1) (/ 4 2) (+ 2 2 2))".to_owned(); // 1 + 1 + (3 * 3) + (2 - 1) + (4 / 2) + (2 + 2 +2) = 20
        assert_eq!(
            run(code).get(0).unwrap().as_ref().unwrap(),
            &Expression::Num(20.0)
        );
    }

    #[test]
    fn greater() {
        let f1 = "(> 1 2)".to_owned();
        let f2 = "(> 1 (+ 1 1))".to_owned();
        let f3 = "(> (- 2 1) (+ 1 1))".to_owned();

        let t1 = "(> 2 1)".to_owned();
        let t2 = "(> 5 (+ 1 1))".to_owned();
        let t3 = "(> (+ 2 1) (+ 1 1))".to_owned();

        assert_eq!(
            run(f1).get(0).unwrap().as_ref().unwrap(),
            &Expression::Bool(false)
        );
        assert_eq!(
            run(f2).get(0).unwrap().as_ref().unwrap(),
            &Expression::Bool(false)
        );
        assert_eq!(
            run(f3).get(0).unwrap().as_ref().unwrap(),
            &Expression::Bool(false)
        );
        assert_eq!(
            run(t1).get(0).unwrap().as_ref().unwrap(),
            &Expression::Bool(true)
        );
        assert_eq!(
            run(t2).get(0).unwrap().as_ref().unwrap(),
            &Expression::Bool(true)
        );
        assert_eq!(
            run(t3).get(0).unwrap().as_ref().unwrap(),
            &Expression::Bool(true)
        );
    }

    #[test]
    fn less() {
        let f1 = "(< 5 2)".to_owned();
        let f2 = "(< 20 (+ 1 1))".to_owned();
        let f3 = "(< (+ 2 1) (+ 1 1))".to_owned();

        let t1 = "(< 10 100)".to_owned();
        let t2 = "(< (- 5) (+ 1 1))".to_owned();
        let t3 = "(< (+ 2 1) (* 5 3))".to_owned();

        assert_eq!(
            run(f1).get(0).unwrap().as_ref().unwrap(),
            &Expression::Bool(false)
        );
        assert_eq!(
            run(f2).get(0).unwrap().as_ref().unwrap(),
            &Expression::Bool(false)
        );
        assert_eq!(
            run(f3).get(0).unwrap().as_ref().unwrap(),
            &Expression::Bool(false)
        );
        assert_eq!(
            run(t1).get(0).unwrap().as_ref().unwrap(),
            &Expression::Bool(true)
        );
        assert_eq!(
            run(t2).get(0).unwrap().as_ref().unwrap(),
            &Expression::Bool(true)
        );
        assert_eq!(
            run(t3).get(0).unwrap().as_ref().unwrap(),
            &Expression::Bool(true)
        );
    }
}
