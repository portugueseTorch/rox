#[cfg(test)]
mod tests {
    use crate::{
        parser::{ast::NodeType, parser::Parser},
        scanner::{
            scanner::Scanner,
            token::{Token, TokenType},
        },
    };

    fn scan<'a>(src: &'a str) -> Vec<Token<'a>> {
        let mut scanner = Scanner::new(src);
        let tokens = scanner.scan().unwrap();
        tokens
    }

    #[test]
    fn parse_number() {
        let tokens = scan("42;");
        assert_eq!(tokens.len(), 3, "Should have 3 tokens");
        let mut it = tokens.iter();
        assert_eq!(it.next().unwrap().token_type, TokenType::Number);
        assert_eq!(it.next().unwrap().token_type, TokenType::Semicolon);
        assert_eq!(it.next().unwrap().token_type, TokenType::EOF);

        let mut parser = Parser::new(tokens);
        let node = parser.parse();

        assert_eq!(parser.has_errors(), false, "Should not have parsing errors");
        assert!(matches!(node.node, NodeType::Constant(_)));
    }

    #[test]
    fn parse_identifier() {
        let tokens = scan("myVar;");
        let mut parser = Parser::new(tokens);
        let node = parser.parse();

        assert_eq!(parser.has_errors(), false, "Should not have parsing errors");
        assert!(matches!(node.node, NodeType::Var(_)));
    }

    #[test]
    fn parse_binop() {
        let tokens = scan("2 + 3;");
        let mut parser = Parser::new(tokens);
        let node = parser.parse();

        assert_eq!(parser.has_errors(), false, "Should not have parsing errors");
        assert!(matches!(node.node, NodeType::BinOp(_)));
    }

    #[test]
    fn parse_complex_binop() {
        let tokens = scan("2 + 3 * 4 + 5 * 6;");
        let mut parser = Parser::new(tokens);
        let node = parser.parse();

        assert_eq!(parser.has_errors(), false, "Should not have parsing errors");
        assert!(matches!(node.node, NodeType::BinOp(_)));
    }

    #[test]
    fn parse_incorrect_binop() {
        let tokens = scan("3 +");
        let mut parser = Parser::new(tokens);
        let node = parser.parse();

        assert_eq!(
            parser.has_errors(),
            true,
            "3 + 2 - is not a valid expression"
        );

        match &node.node {
            NodeType::BinOp(bin) => {
                assert!(matches!(bin.right.node, NodeType::Error))
            }
            _ => panic!("Should be binop"),
        }
    }

    #[test]
    fn parse_group_expr() {
        let tokens = scan("(3 + 2);");
        let mut parser = Parser::new(tokens);
        let node = parser.parse();

        assert!(!parser.has_errors());
        assert!(matches!(node.node, NodeType::Grouping(_)));
    }

    #[test]
    fn parse_arithmetic_with_group() {
        let tokens = scan("(3 + 2) * 10;");
        let mut parser = Parser::new(tokens);
        let node = parser.parse();

        assert!(!parser.has_errors());
        match &node.node {
            NodeType::BinOp(bin) => {
                assert!(matches!(bin.left.node, NodeType::Grouping(_)));
                assert!(matches!(bin.right.node, NodeType::Constant(_)));
            }
            _ => panic!("Should be binop"),
        }
    }

    #[test]
    fn parse_simple_unary() {
        let tokens = scan("-42;");
        let mut parser = Parser::new(tokens);
        let node = parser.parse();

        assert!(!parser.has_errors());
        match &node.node {
            NodeType::Unary(unary) => {
                assert!(matches!(unary.op, TokenType::Minus));
                assert!(matches!(unary.operand.node, NodeType::Constant(_)));
            }
            _ => panic!("Should be unary"),
        }
    }

    #[test]
    fn parse_multi_unary() {
        let tokens = scan("--42;");
        let mut parser = Parser::new(tokens);
        let node = parser.parse();

        assert!(!parser.has_errors());
        match &node.node {
            NodeType::Unary(unary) => {
                assert!(matches!(unary.op, TokenType::Minus));
                assert!(matches!(unary.operand.node, NodeType::Unary(_)));
            }
            _ => panic!("Should be unary"),
        }
    }

    #[test]
    fn parse_grouped_unary() {
        let tokens = scan("-(42 + 10);");
        let mut parser = Parser::new(tokens);
        let node = parser.parse();

        assert!(!parser.has_errors());
        match &node.node {
            NodeType::Unary(unary) => {
                assert!(matches!(unary.op, TokenType::Minus));
                assert!(matches!(unary.operand.node, NodeType::Grouping(_)));
            }
            _ => panic!("Should be unary"),
        }
    }

    #[test]
    fn parse_complex() {
        let tokens = scan("-(42 + 10) + 27 / (10 + (b * myVar));");
        let parser = Parser::new(tokens);

        assert!(!parser.has_errors());
    }

    #[test]
    fn parse_assignment() {
        let tokens = scan("myVar = 42;");
        let mut parser = Parser::new(tokens);
        let node = parser.parse();

        assert!(!parser.has_errors());
        assert!(matches!(node.node, NodeType::Assignment(_)));
    }

    #[test]
    fn parse_assignment_to_expression() {
        let tokens = scan("myVar = -(42 + 10);");
        let mut parser = Parser::new(tokens);
        let node = parser.parse();

        assert!(!parser.has_errors());
        assert!(matches!(node.node, NodeType::Assignment(_)));
    }

    #[test]
    fn parse_bool_expression() {
        let tokens = scan("myBool = true;");
        let mut parser = Parser::new(tokens);
        let node = parser.parse();

        assert!(!parser.has_errors());
        assert!(matches!(node.node, NodeType::Assignment(_)));
    }

    #[test]
    fn parse_comparison_expression() {
        let tokens = scan("true or false and 42;");
        let mut parser = Parser::new(tokens);
        let node = parser.parse();
        node.log();

        assert!(!parser.has_errors());
        assert!(matches!(node.node, NodeType::BinOp(_)));
    }
}
