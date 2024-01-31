use std::io;
mod eval;
mod lexer;
mod parser;
mod structs;

use crate::eval::eval;
use crate::lexer::lexer;
use crate::parser::parse;

fn main() {
    let mut input = String::new();
    match io::stdin().read_line(&mut input) {
        Ok(_) => {
            //println!("Success!");
        }
        Err(e) => println!("Error: {}", e),
    }

    let tokens = lexer(&input.trim());
    //println!("{:?}", tokens);
    let ast = parse(tokens);
    if ast.is_some() {
        let result = eval(ast.unwrap());
        println!("{}", result);
    }
    //println!("Hello, world!");
}
