#[cfg(test)]
mod tests {
    use math_interpreter::structs::{Node, Token};

    use math_interpreter::lexer::lexer;
    use math_interpreter::parser::parse;
    use math_interpreter::eval::eval;

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

    #[test]
    fn test_eval1() {
        let tokens = lexer("(1 + 2) * 9");
        let ast = parse(tokens);
        let result = eval(ast.unwrap());
        assert_eq!(
            result,
            27.0
        );
    }

    #[test]
    fn test_eval2() {
        let tokens = lexer("(1 / 2) * 2 + 5");
        let ast = parse(tokens);
        let result = eval(ast.unwrap());
        assert_eq!(
            result,
            6.0
        );
    }

    #[test]
    fn test_eval3() {
        let tokens = lexer("- 5 * 8  / 2");
        let ast = parse(tokens);
        let result = eval(ast.unwrap());
        assert_eq!(
            result,
            -20.0
        );
    }

    #[test]
    fn test_eval4() {
        let tokens = lexer("1 - (5 ^ 2) - 0.1");
        let ast = parse(tokens);
        let result = eval(ast.unwrap());
        assert_eq!(
            result,
            -24.1
        );
    }

    #[test]
    fn test_eval5() {
        let tokens = lexer("1 - 1 - 1 - 1");
        let ast = parse(tokens);
        let result = eval(ast.unwrap());
        assert_eq!(
            result,
            -2.0
        );
    }

    #[test]
    fn test_eval6() {
        let tokens = lexer("1 - (- 1)");
        let ast = parse(tokens);
        let result = eval(ast.unwrap());
        assert_eq!(
            result,
            2.0
        );
    }
    
    #[test]
    fn test_eval7() {
        let tokens = lexer("-0-0-0+1");
        let ast = parse(tokens);
        let result = eval(ast.unwrap());
        assert_eq!(
            result,
            1.0
        );
    }
}
