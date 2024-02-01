use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, Debug, Clone, PartialEq)]
pub enum Token {
    Number(String),
    PlusOp,
    MinusOp,
    MultiplyOp,
    DivideOp,
    LeftParenthesis,
    RightParenthesis,
    PowOp,
}

#[derive(Serialize, Deserialize, Debug, PartialEq)]

pub enum Node {
    Parenthesis(Box<Node>),
    Negative(Box<Node>),
    Add(Box<Node>, Box<Node>),
    Subtract(Box<Node>, Box<Node>),
    Multiply(Box<Node>, Box<Node>),
    Divide(Box<Node>, Box<Node>),
    Pow(Box<Node>, Box<Node>),
    Number(String),
}
/*
#[derive(Serialize, Deserialize, Debug, PartialEq)]
pub struct Node {
    pub t: NodeType,
    pub children: Vec<Node>,
}

impl Node {
    pub fn New(t: NodeType) -> Node {
        Node {
            t,
            children: Vec::new(),
        }
    }
    pub fn add_child(&mut self, n: Node) {
        self.children.push(n);
    }
}
 */
