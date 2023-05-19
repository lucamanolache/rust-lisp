use std::collections::HashMap;

pub enum Node {
    Int(i32),
    Expr {
        op: String,
        children: Vec<Node>,
    },
}

pub trait Method {
    fn operate(Vec<Node>) -> i32;

}

pub fn eval(node: &Node, HashMap<String, Box<dyn &Method>) -> i32 {
    match node {
        Node::Int(x) => {
            *x
        }
        Node::Expr { op, children } => {

        }
    }
}