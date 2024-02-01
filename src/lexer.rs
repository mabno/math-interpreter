use core::panic;

use regex::Regex;

use crate::structs::Token;

const NUMBER_REGEX: &str = r"^\d+(\.\d*)?$";

fn is_number(source: &str) -> bool {
    let re = Regex::new(NUMBER_REGEX).unwrap();
    re.is_match(source)
}
fn is_pow (source: &str) -> bool {
    source == "^"
}
fn is_plus(source: &str) -> bool {
    source == "+"
}
fn is_minus(source: &str) -> bool {
    source == "-"
}
fn is_multiply(source: &str) -> bool {
    source == "*"
}
fn is_divide(source: &str) -> bool {
    source == "/"
}
fn is_left_parenthesis(source: &str) -> bool {
    source == "("
}
fn is_right_parenthesis(source: &str) -> bool {
    source == ")"
}

fn get_possible_tokens(source: &str) -> Vec<Token> {
    let mut possible_tokens: Vec<Token> = Vec::new();
    if is_number(source) {
        possible_tokens.push(Token::Number(source.to_string()));
    }
    if is_pow(source) {
        possible_tokens.push(Token::PowOp);
    }
    if is_plus(source) {
        possible_tokens.push(Token::PlusOp);
    }
    if is_minus(source) {
        possible_tokens.push(Token::MinusOp);
    }
    if is_multiply(source) {
        possible_tokens.push(Token::MultiplyOp);
    }
    if is_divide(source) {
        possible_tokens.push(Token::DivideOp);
    }
    if is_left_parenthesis(source) {
        possible_tokens.push(Token::LeftParenthesis);
    }
    if is_right_parenthesis(source) {
        possible_tokens.push(Token::RightParenthesis);
    }
    possible_tokens
}

pub fn lexer(source: &str) -> Vec<Token> {
    //println!("lexer: {}", source);

    let mut tokens: Vec<Token> = Vec::new();
    let mut previous_possible_tokens: Vec<Token> = Vec::new();

    let mut start = 0;
    let mut end = 1;

    while end <= source.len() {
        let current_char = &source[start..end];
        let possible_tokens: Vec<Token> = get_possible_tokens(current_char);
        //println!("{:?}_{}_({}-{})", possible_tokens, current_char, start, end);

        if possible_tokens.len() == 0 {
            if current_char == " " {
                start += 1;
                end += 1;
            } else {
                if previous_possible_tokens.len() >= 1 {
                    tokens.push(previous_possible_tokens[0].clone());
                    previous_possible_tokens = Vec::new();
                    start = end - 1;
                } else {
                    panic!("Invalid token: {}", current_char);
                }
            }
        } else {
            previous_possible_tokens = possible_tokens;
            end += 1;
        }

        if end > source.len() {
            tokens.push(previous_possible_tokens[0].clone());
        }
    }

    tokens
}
