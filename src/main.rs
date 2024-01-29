//use std::io;
mod lexer;
mod parser;
mod structs;


use crate::lexer::lexer;
use crate::parser::parse;



fn main() {
   /*  let mut input = String::new();
    match io::stdin().read_line(&mut input) {
        Ok(_) => {
            println!("Success!");
        }
        Err(e) => println!("Error: {}", e),
    } */

    println!("{:?}", parse(lexer("(1 * ( 1 + (3 * 2) + 7 )) * 2")));
    println!("Hello, world!");
}
