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

        if let Err(e) = tokens {
            println!("Error: {:?}", e);
            continue;
        }

        //println!("{:?}", tokens);
        let ast = parse(tokens.unwrap());
        match ast {
            Err(e) => println!("Error: {:?}", e),
            Ok(ast) => {
                let result = eval(ast);
                match result {
                    Ok(r) => println!("> {}", r),
                    Err(e) => println!("Error: {:?}", e),
                }
            }
        }
        //println!("Hello, world!");
    }
}
