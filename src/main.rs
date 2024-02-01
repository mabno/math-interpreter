pub mod eval;
pub mod lexer;
pub mod parser;
pub mod structs;

use crate::eval::eval;
use crate::lexer::lexer;
use crate::parser::parse;
use std::io;

fn main() {
    println!("Welcome to the math interpreter! Written by Mariano 😊");
    loop {
        let mut input = String::new();
        match io::stdin().read_line(&mut input) {
            Ok(_) => {
                //println!("Success!");
            }
            Err(e) => println!("Error: {}", e),
        }

        let tokens = lexer(&input.trim());
        println!("{:?}", tokens);
        let ast = parse(tokens);
        if ast.is_some() {
            let result = eval(ast.unwrap());
            println!("> {}", result);
        }
        //println!("Hello, world!");
    }
}
