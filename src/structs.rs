#[derive(Debug)]
#[derive(Clone)]
#[derive(PartialEq)]
pub enum Token {
  Number(String),
  PlusOp,
  MinusOp,
  MultiplyOp,
  DivideOp,
  LeftParenthesis,
  RightParenthesis
}