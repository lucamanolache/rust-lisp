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

// #[cfg(test)]
// mod tests {
//     #[test]
//     fn add_1_1() {
//         assert_eq!()
//     }
// }
