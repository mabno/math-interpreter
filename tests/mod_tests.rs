#[cfg(test)]
mod tests {
    use math_interpreter::structs::{Node, Token};

    // Importa las funciones que deseas probar
    use math_interpreter::lexer::lexer;
    use math_interpreter::parser::parse;

    #[test]
    fn test_lexer() {
        let input = "1 + 2";
        let tokens = lexer(&input);
        assert_eq!(
            tokens,
            vec![
                Token::Number("1".to_string()),
                Token::PlusOp,
                Token::Number("2".to_string())
            ]
        );
    }

    #[test]
    fn test_parser() {
        let tokens = vec![
            Token::Number("1".to_string()),
            Token::PlusOp,
            Token::Number("2".to_string()),
        ];
        let ast = parse(tokens);
        assert_eq!(
            ast,
            Some(Node::Add(
                Box::new(Node::Number("1".to_string())),
                Box::new(Node::Number("2".to_string()))
            ))
        );
    }
}
