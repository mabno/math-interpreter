use crate::structs::Token;

#[derive(Debug)]
enum NonTerminalType {
    Expression,
    BetweenParenthesisExpression,
    Add,
    Subtract,
    Multiply,
    Divide,
}

#[derive(Debug)]
enum NodeType {
  Terminal(Token),
  NonTerminal(NonTerminalType)
}

#[derive(Debug)]
pub struct Node {
    children: Vec<Node>,
    t: NodeType
}

impl Node {
    fn new(t: NodeType) -> Node {
        Node {
            children: Vec::new(),
            t: t
        }
    }

    fn add_child(&mut self, child: Node) {
        self.children.push(child);
    }
}


fn parse_add(tokens: &[Token]) -> Option<Node> {
    let mut opened_parenthesis = 0;

    for (i, token) in tokens.iter().enumerate() {
        match token {
            Token::LeftParenthesis => {
                opened_parenthesis += 1;
            }
            Token::RightParenthesis => {
                opened_parenthesis -= 1;
            }
            Token::PlusOp => {
                if opened_parenthesis == 0 {
                    let mut node = Node::new(NodeType::NonTerminal(NonTerminalType::Add));
                    let left_side = parse_expression(&tokens[0..i]);
                    let right_side = parse_expression(&tokens[i+1..]);
                    if let Some(n) = left_side { node.add_child(n) }
                    node.add_child(Node::new(NodeType::Terminal(Token::PlusOp)));
                    if let Some(n) = right_side { node.add_child(n) }
                    return Some(node);
              
                }
            }

            _ => {}
        }

      }
    return None;
}

fn parse_substract(tokens: &[Token]) -> Option<Node> {
    let mut opened_parenthesis = 0;

    for (i, token) in tokens.iter().enumerate() {
        match token {
            Token::LeftParenthesis => {
                opened_parenthesis += 1;
            }
            Token::RightParenthesis => {
                opened_parenthesis -= 1;
            }
            Token::MinusOp => {
                if opened_parenthesis == 0 {
                    let mut node = Node::new(NodeType::NonTerminal(NonTerminalType::Add));
                    let left_side = parse_expression(&tokens[0..i]);
                    let right_side = parse_expression(&tokens[i+1..]);
                    if let Some(n) = left_side { node.add_child(n) }
                    node.add_child(Node::new(NodeType::Terminal(Token::MinusOp)));
                    if let Some(n) = right_side { node.add_child(n) }
                    return Some(node);
              
                }
            }

            _ => {}
        }

      }
    return None;
}


fn parse_multiply(tokens: &[Token]) -> Option<Node> {
    let mut opened_parenthesis = 0;

    for (i, token) in tokens.iter().enumerate() {
        match token {
            Token::LeftParenthesis => {
                opened_parenthesis += 1;
            }
            Token::RightParenthesis => {
                opened_parenthesis -= 1;
            }
            Token::MultiplyOp => {
                if opened_parenthesis == 0 {
                    let mut node = Node::new(NodeType::NonTerminal(NonTerminalType::Add));
                    let left_side = parse_expression(&tokens[0..i]);
                    let right_side = parse_expression(&tokens[i+1..]);
                    if let Some(n) = left_side { node.add_child(n) }
                    node.add_child(Node::new(NodeType::Terminal(Token::MultiplyOp)));
                    if let Some(n) = right_side { node.add_child(n) }
                    return Some(node);
              
                }
            }

            _ => {}
        }

      }
    return None;
}


fn parse_divide(tokens: &[Token]) -> Option<Node> {
    let mut opened_parenthesis = 0;

    for (i, token) in tokens.iter().enumerate() {
        match token {
            Token::LeftParenthesis => {
                opened_parenthesis += 1;
            }
            Token::RightParenthesis => {
                opened_parenthesis -= 1;
            }
            Token::DivideOp => {
                if opened_parenthesis == 0 {
                    let mut node = Node::new(NodeType::NonTerminal(NonTerminalType::Add));
                    let left_side = parse_expression(&tokens[0..i]);
                    let right_side = parse_expression(&tokens[i+1..]);
                    if let Some(n) = left_side { node.add_child(n) }
                    node.add_child(Node::new(NodeType::Terminal(Token::DivideOp)));
                    if let Some(n) = right_side { node.add_child(n) }
                    return Some(node);
              
                }
            }

            _ => {}
        }

      }
    return None;
}



fn parse_between_parenthesis_expression(tokens: &[Token]) -> Option<Node> {
  let first = tokens.first();
  let last = tokens.last();

  if first.is_some() && last.is_some() {
      let first = first.unwrap();
      let last = last.unwrap();

      if *first == Token::LeftParenthesis && *last == Token::RightParenthesis {
        let mut node = Node::new(NodeType::NonTerminal(NonTerminalType::BetweenParenthesisExpression));
        node.add_child(Node::new(NodeType::Terminal(Token::LeftParenthesis)));
        let inside_expression = parse_expression(&tokens[1..tokens.len()-1]);
        if let Some(n) = inside_expression { node.add_child(n) }
        node.add_child(Node::new(NodeType::Terminal(Token::RightParenthesis)));
        return Some(node);
        }
  }
    None
}


fn parse_expression(tokens: &[Token]) -> Option<Node> {
    let mut node = parse_between_parenthesis_expression(&tokens);
    if node.is_none() {
      node = parse_add(&tokens);
    }
    if node.is_none() {
        node = parse_substract(&tokens);
    }
    if node.is_none(){
        node = parse_multiply(&tokens);
    }
    if node.is_none() {
        node = parse_divide(&tokens);
    }

    node
   
}


/*

( 1 - 1 ) * 5

Grammar!

S -> Expression

Expression -> Add | Substract |
              Multiply | Divide | BetweenParenthesisExpression
        
BetweenParenthesisExpression ->  LeftParenthesisToken Expression RightParenthesisToken

Expression -> NumberToken

Add -> Expression PlusToken Expression
Substract -> Expression MinusToken Expression
Multiply -> Expression MultiplyToken Expression
Divide -> Expression DivideToken Expression


*/


// Build AST (Abstract-Syntax-Tree)
pub fn parse(tokens: Vec<Token>) -> Option<Node> {
    let root = parse_expression(&tokens);
    root
}