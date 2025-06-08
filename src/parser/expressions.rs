use crate::scanner::token::{Token, TokenType};

use super::ast::{Expr, ExprNode};

#[derive(Clone)]
pub struct BinaryExpr<'a> {
    pub op: TokenType,
    pub left: Box<ExprNode<'a>>,
    pub right: Box<ExprNode<'a>>,
}

#[derive(Clone)]
pub struct UnaryExpr<'a> {
    pub op: TokenType,
    pub operand: Box<ExprNode<'a>>,
}

#[derive(Clone)]
pub struct AssignmentExpr<'a> {
    pub name: Token<'a>,
    pub expr: Box<ExprNode<'a>>,
}

#[derive(Clone)]
pub struct CallExpr<'a> {
    pub calee: Box<ExprNode<'a>>,
    pub args: Vec<ExprNode<'a>>,
}

#[derive(Clone)]
pub struct PropertyAccessExpr<'a> {
    pub object: Box<ExprNode<'a>>,
    pub property: Token<'a>,
}

// --- may be subject to constant folding
#[derive(Clone)]
pub enum Value<'a> {
    StringLiteral(&'a str),
    Number(i32),
    Bool(bool),
    Nil,
}

#[cfg(test)]
mod tests {
    use crate::{
        parser::{ast::Expr, parser::Parser},
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
        let node = parser.parse_expression(true);

        assert_eq!(parser.has_errors(), false, "Should not have parsing errors");
        assert!(matches!(node.node, Expr::Constant(_)));
    }

    #[test]
    fn parse_identifier() {
        let tokens = scan("myVar;");
        let mut parser = Parser::new(tokens);
        let node = parser.parse_expression(true);

        assert_eq!(parser.has_errors(), false, "Should not have parsing errors");
        assert!(matches!(node.node, Expr::Var(_)));
    }

    #[test]
    fn parse_binop() {
        let tokens = scan("2 + 3;");
        let mut parser = Parser::new(tokens);
        let node = parser.parse_expression(true);

        assert_eq!(parser.has_errors(), false, "Should not have parsing errors");
        assert!(matches!(node.node, Expr::BinOp(_)));
    }

    #[test]
    fn parse_complex_binop() {
        let tokens = scan("2 + 3 * 4 + 5 * 6;");
        let mut parser = Parser::new(tokens);
        let node = parser.parse_expression(true);

        assert_eq!(parser.has_errors(), false, "Should not have parsing errors");
        assert!(matches!(node.node, Expr::BinOp(_)));
    }

    #[test]
    fn parse_incorrect_binop() {
        let tokens = scan("3 +");
        let mut parser = Parser::new(tokens);
        let node = parser.parse_expression(true);

        assert_eq!(
            parser.has_errors(),
            true,
            "3 + 2 - is not a valid expression"
        );

        match &node.node {
            Expr::BinOp(bin) => {
                assert!(matches!(bin.right.node, Expr::Error))
            }
            _ => panic!("Should be binop"),
        }
    }

    #[test]
    fn parse_with_group() {
        let tokens = scan("(3 + 2) * 10;");
        let mut parser = Parser::new(tokens);
        let node = parser.parse_expression(true);

        assert!(!parser.has_errors());
        match &node.node {
            Expr::BinOp(bin) => {
                assert!(matches!(bin.left.node, Expr::Grouping(_)));
                assert!(matches!(bin.right.node, Expr::Constant(_)));
            }
            _ => panic!("Should be binop"),
        }
    }

    #[test]
    fn parse_simple_unary() {
        let tokens = scan("-42;");
        let mut parser = Parser::new(tokens);
        let node = parser.parse_expression(true);

        assert!(!parser.has_errors());
        match &node.node {
            Expr::Unary(unary) => {
                assert!(matches!(unary.op, TokenType::Minus));
                assert!(matches!(unary.operand.node, Expr::Constant(_)));
            }
            _ => panic!("Should be unary"),
        }
    }

    #[test]
    fn parse_multi_unary() {
        let tokens = scan("--42;");
        let mut parser = Parser::new(tokens);
        let node = parser.parse_expression(true);

        assert!(!parser.has_errors());
        match &node.node {
            Expr::Unary(unary) => {
                assert!(matches!(unary.op, TokenType::Minus));
                assert!(matches!(unary.operand.node, Expr::Unary(_)));
            }
            _ => panic!("Should be unary"),
        }
    }

    #[test]
    fn parse_grouped_unary() {
        let tokens = scan("-(42 + 10);");
        let mut parser = Parser::new(tokens);
        let node = parser.parse_expression(true);

        assert!(!parser.has_errors());
        match &node.node {
            Expr::Unary(unary) => {
                assert!(matches!(unary.op, TokenType::Minus));
                assert!(matches!(unary.operand.node, Expr::Grouping(_)));
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
    fn parse_assignment_to_expression() {
        let tokens = scan("myVar = -(42 + 10);");
        let mut parser = Parser::new(tokens);
        let node = parser.parse_expression(true);

        assert!(!parser.has_errors());
        assert!(matches!(node.node, Expr::Assignment(_)));
    }

    #[test]
    fn parse_logical_expression() {
        let tokens = scan("true or false and 42;");
        let mut parser = Parser::new(tokens);
        let node = parser.parse_expression(true);

        assert!(!parser.has_errors());
        assert!(matches!(node.node, Expr::BinOp(_)));
    }

    #[test]
    fn parse_equality_expression() {
        let tokens = scan("32 == 27;");
        let mut parser = Parser::new(tokens);
        let node = parser.parse_expression(true);

        assert!(!parser.has_errors());
        assert!(matches!(node.node, Expr::BinOp(_)));
    }

    #[test]
    fn parse_equality_expression_2() {
        let tokens = scan("32 != 27;");
        let mut parser = Parser::new(tokens);
        let node = parser.parse_expression(true);

        assert!(!parser.has_errors());
        assert!(matches!(node.node, Expr::BinOp(_)));
    }

    #[test]
    fn parse_comparison_expression() {
        let tokens = scan("32 >= 27 and 10 < 11 or 9 <= 6 and 8 > 2;");
        let mut parser = Parser::new(tokens);
        let node = parser.parse_expression(true);

        assert!(!parser.has_errors());
        assert!(matches!(node.node, Expr::BinOp(_)));
    }

    #[test]
    fn parse_property_access() {
        let tokens = scan("object.property;");
        let mut parser = Parser::new(tokens);
        let node = parser.parse_expression(true);

        assert!(!parser.has_errors());
        assert!(matches!(node.node, Expr::PropertyAccess(_)));
    }

    #[test]
    fn parse_call_expression() {
        let tokens = scan("obj.myFunc(42, hello);");
        let mut parser = Parser::new(tokens);
        let node = parser.parse_expression(true);
        node.log();

        assert!(!parser.has_errors());
        assert!(matches!(node.node, Expr::Call(_)));
    }

    #[test]
    fn parse_call_expression_multiple_args() {
        let tokens = scan("myFunc(42, hello + 3);");
        let mut parser = Parser::new(tokens);
        let node = parser.parse_expression(true);

        assert!(!parser.has_errors());
        assert!(matches!(node.node, Expr::Call(_)));
    }

    #[test]
    fn parse_call_prop_access() {
        let tokens = scan("obj.methodOne(42).methodTwo(hello, goodbye)();");
        let mut parser = Parser::new(tokens);
        let node = parser.parse_expression(true);

        assert!(!parser.has_errors());
        // assert!(matches!(node.node, NodeType::Call(_)));
    }
}
