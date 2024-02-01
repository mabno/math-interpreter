use crate::structs::{Node, Token};

/* impl Node {
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
 */

// Parsea un número
fn parse_number(tokens: &[Token]) -> Option<Node> {
    if tokens.len() == 1 {
        match &tokens[0] {
            Token::Number(n) => {
                let node = Node::Number(n.to_string());
                return Some(node);
            }
            _ => {}
        }
    } 
    //return Some(Node::Number("0".to_string())); 
    return None
}

// Parsea la operación potencia entre dos expresiones
fn parse_pow(tokens: &[Token]) -> Option<Node> {
    let mut opened_parenthesis = 0;

    for (i, token) in tokens.iter().enumerate() {
        match token {
            Token::LeftParenthesis => {
                opened_parenthesis += 1;
            }
            Token::RightParenthesis => {
                opened_parenthesis -= 1;
            }
            Token::PowOp => {
                if opened_parenthesis == 0 {
                    let mut node: Option<Node> = None;
                    if let Some(left_side) = parse_expression(&tokens[0..i], &node) {
                        if let Some(right_side) = parse_expression(&tokens[i + 1..], &node) {
                            node = Some(Node::Pow(Box::new(left_side), Box::new(right_side)));
                            return node;
                        }
                    }
                }
            }

            _ => {}
        }
    }
    return None;
}

// Parsea la operación suma entre dos expresiones
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
                    let mut node: Option<Node> = None;
                    if let Some(left_side) = parse_left_side_expression(&tokens[0..i], &node) {
                        if let Some(right_side) = parse_left_side_expression(&tokens[i + 1..], &node) {
                            node = Some(Node::Add(Box::new(left_side), Box::new(right_side)));
                            return node;
                        }
                    }
                }
            }

            _ => {}
        }
    }
    return None;
}

// Parsea la operación resta entre dos expresiones.
fn parse_substract(tokens: &[Token]) -> Option<Node> {
    let mut opened_parenthesis = 0;

    for (i, token) in tokens.iter().enumerate().rev() {
        match token {
            Token::LeftParenthesis => {
                opened_parenthesis += 1;
            }
            Token::RightParenthesis => {
                opened_parenthesis -= 1;
            }
            Token::MinusOp => {
                if opened_parenthesis == 0{
                    let mut node: Option<Node> = None;
                    println!("{:?}", &tokens[0..i]);
                    println!("{:?}", &tokens[i..]);
                    //panic!("dsfdsf");
                    if let Some(left_side) = parse_expression(&tokens[0..i], &node) {
                        if let Some(right_side) = parse_left_side_expression(&tokens[i..], &node) {

                        
                            node = Some(Node::Add(Box::new(left_side), Box::new(right_side)));
                            return node;
                        }
                    }
                }
            }

            _ => {}
        }
    }
    return None;
}

// Parsea la operación multiplicación entre dos expresiones
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
                    let mut node: Option<Node> = None;
                    if let Some(left_side) = parse_left_side_expression(&tokens[0..i], &node) {
                        if let Some(right_side) = parse_expression(&tokens[i + 1..], &node) {
                            node = Some(Node::Multiply(Box::new(left_side), Box::new(right_side)));
                            return node;
                        }
                    }
                }
            }

            _ => {}
        }
    }
    return None;
}

// Parsea la operación division entre dos expresiones
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
                    let mut node: Option<Node> = None;
                    if let Some(left_side) = parse_left_side_expression(&tokens[0..i], &node) {
                        if let Some(right_side) = parse_expression(&tokens[i + 1..], &node) {
                            node = Some(Node::Divide(Box::new(left_side), Box::new(right_side)));
                            return node;
                        }
                    }
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
            let mut node: Option<Node> = None;
            let inside_expression = parse_left_side_expression(&tokens[1..tokens.len() - 1], &node);
            if let Some(n) = inside_expression {
                return Some(Node::Parenthesis(Box::new(n)));
            }
        }
    }
    None
}

// Es una expresion normal, solamente contempla la posibilidad de que sea un numero negativo
fn parse_left_side_expression(tokens: &[Token], parent: &Option<Node>) -> Option<Node> {
    if tokens.len() == 0 {
        return Some(Node::Number("0".to_string()));
    }
    if tokens.len() > 1 && tokens[0] == Token::MinusOp{
        if let Some(n) = parse_expression(&tokens[1..], parent) {
        let mut node = Some(Node::Negative(
                Box::new(
                    n
            )));
            return node
        }
    }
    
    None
}

// Parsea una expresión, puede ser una operación o un número no negativo
fn parse_expression(tokens: &[Token], parent: &Option<Node>) -> Option<Node> {
    let mut node = parse_between_parenthesis_expression(tokens);
    if node.is_none() {
        node = parse_add(tokens);
    }
    if node.is_none() {
        node = parse_substract(tokens);
    }
    if node.is_none() {
        node = parse_multiply(tokens);
    }
    if node.is_none() {
        node = parse_pow(tokens);
    }
    if node.is_none() {
        node = parse_divide(tokens);
    }
    if node.is_none() {
        node = parse_number(tokens)
    }

    node
}

/*

( 1 - 1 ) * 5

Grammar!

S -> Expression

Expression -> Add | Substract |
              Multiply | Divide | BetweenParenthesisExpression | Number | LeftSideExpression

BetweenParenthesisExpression ->  LeftParenthesis LeftSideExpression RightParenthesis


Add -> LeftSideExpression PlusOp Expression
Substract -> LeftSideExpression MinusOp Expression
Multiply -> LeftSideExpression MultiplyOp Expression
Divide -> LeftSideExpression DivideOp Expression


*/

// Build AST (Abstract-Syntax-Tree)
pub fn parse(tokens: Vec<Token>) -> Option<Node> {
    let root = parse_expression(&tokens, &Some(Node::Root));

   

    let serialized = serde_json::to_string_pretty(&root).unwrap();
    println!("{}", serialized);

    if root.is_none() {
        panic!("SyntaxError: Invalid syntax");
    }

    root
}
