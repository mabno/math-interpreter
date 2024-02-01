use crate::structs::Node;

pub fn eval(ast: Node) -> f32 {
    match ast {
        Node::Add(a, b) => {
            let a = eval(*a);
            let b = eval(*b);
            a + b
        }
        Node::Multiply(a, b) => {
            let a = eval(*a);
            let b = eval(*b);
            a * b
        }
        Node::Divide(a, b) => {
            let a = eval(*a);
            let b = eval(*b);
            if b == 0.0 {
                panic!("Division by zero");
            }
            a / b
        }
        Node::Pow(a, b) => {
            let a = eval(*a);
            let b = eval(*b);
            a.powf(b)
        }
        Node::Number(n) => n.parse::<f32>().unwrap(),
        Node::Parenthesis(n) => eval(*n),
        Node::Root => 0.0,
    }
}
