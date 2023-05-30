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
}
