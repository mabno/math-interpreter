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
    Root,
    Parenthesis(Box<Node>),
    Add(Box<Node>, Box<Node>),
    Multiply(Box<Node>, Box<Node>),
    Divide(Box<Node>, Box<Node>),
    Pow(Box<Node>, Box<Node>),
    Number(String),
}
