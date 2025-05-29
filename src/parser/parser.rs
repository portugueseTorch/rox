use crate::{
    errors::RoxError,
    scanner::token::{Token, TokenType},
};

use super::ast::{BinaryExpr, Node, UnaryExpr, Value};

macro_rules! parsing_error {
    ($parser:expr, $tok:expr, $msg:expr) => {
        $parser.handle_error($tok.clone(), $msg);
        return Node::Error;
    };
}

pub struct Parser<'a> {
    cur: usize,
    tokens: Vec<Token<'a>>,
    errors: Vec<RoxError<'a>>,
}

impl<'a> Parser<'a> {
    pub fn new(tokens: Vec<Token<'a>>) -> Self {
        Parser {
            cur: 0,
            tokens,
            errors: vec![],
        }
    }

    pub fn parse(&mut self) -> Node<'a> {
        let expr = self.parse_expr(0);
        self.assert(TokenType::Semicolon);
        expr
    }

    fn parse_expr(&mut self, bp: usize) -> Node<'a> {
        if self.is_at_end() {
            return Node::Error;
        }

        let tok = self.next().clone();
        let mut lhs = match tok.token_type {
            TokenType::StringLiteral => Node::Literal(Value::StringLiteral(tok.lexeme.unwrap())),
            TokenType::Identifier => Node::Var(tok.lexeme.unwrap()),
            TokenType::Minus | TokenType::Plus | TokenType::Bang => {
                let (_, rbp) = prefix_binding_power(tok.token_type);
                let operand = self.parse_expr(rbp);
                Node::Unary(UnaryExpr {
                    op: tok,
                    operand: Box::new(operand),
                })
            }
            TokenType::Number => {
                let num_as_str = tok.lexeme.unwrap();
                let parsed_num = num_as_str.parse().unwrap();
                Node::Literal(Value::Number(parsed_num))
            }
            TokenType::LeftParen => {
                let group_expr = self.parse_expr(0);
                if !group_expr.is_error() && !self.matches(TokenType::RightParen) {
                    parsing_error!(
                        self,
                        self.prev().unwrap(),
                        format!(
                            "unexpected token: expected '(' but got '{}'",
                            self.prev().unwrap().token_type
                        )
                    );
                }

                Node::Grouping(Box::new(group_expr))
            }
            _ => Node::Error,
        };
        if lhs.is_error() {
            parsing_error!(
                self,
                self.prev().unwrap(),
                format!("unexpected token: '{}'", self.prev().unwrap().token_type)
            );
        }

        loop {
            let op = self.peek().clone();
            let (lbp, rbp) = match op.token_type {
                TokenType::Plus | TokenType::Minus | TokenType::Star | TokenType::Slash => {
                    infix_binding_power(op.token_type)
                }
                TokenType::EOF | TokenType::Semicolon | TokenType::RightParen => break,
                _ => {
                    parsing_error!(
                        self,
                        op,
                        format!(
                            "unexpected token: expected arithmetic operator but got '{}'",
                            op.token_type
                        )
                    );
                }
            };

            // --- if the left binding power of the operator is lower, break
            if lbp < bp {
                break;
            }

            // --- parse right hand side
            self.next();
            let rhs = self.parse_expr(rbp);

            // --- emit Node based on the type of the operator
            lhs = Node::BinOp(BinaryExpr {
                left: Box::new(lhs),
                right: Box::new(rhs),
                op,
            })
        }

        lhs
    }

    pub fn has_errors(&self) -> bool {
        !self.errors.is_empty()
    }

    pub fn log_errors(&self) {
        assert!(!self.errors.is_empty());
        println!(
            "Errors detected while parsing: found {} errors",
            self.errors.len()
        );

        for error in self.errors.iter() {
            eprintln!("{}", error);
        }
    }
}

/**
* Utils
*/
impl<'a> Parser<'a> {
    /// Advances cur and returns the previous token
    fn next(&mut self) -> &Token<'a> {
        self.cur += 1;
        self.prev().unwrap_or(&Token {
            token_type: TokenType::EOF,
            line: 0,
            lexeme: None,
        })
    }

    fn is_at_end(&self) -> bool {
        self.cur >= self.tokens.len()
    }

    /// Returns a reference to the previous token, if any
    fn prev(&self) -> Option<&Token<'a>> {
        if self.cur - 1 < 0 {
            return None;
        }

        self.tokens.get(self.cur - 1)
    }

    /// Asserts that the current token is of the provided type.
    /// If it is not sets the error flag to true and generates the appropriate error
    fn assert(&mut self, token_type: TokenType) {
        if self.is_at_end() {
            return;
        }

        if self.tokens[self.cur].token_type == token_type {
            self.next();
            return;
        }

        self.handle_error(
            self.tokens[self.cur].clone(),
            format!(
                "unexpected token type: expected '{}' but got '{}'",
                token_type, self.tokens[self.cur].token_type
            ),
        );
    }

    /// If current token matches target, iterates and returns true
    fn matches(&mut self, target: TokenType) -> bool {
        if self.peek().token_type == target {
            self.next();
            return true;
        }

        false
    }

    fn equals_any(&self, targets: Vec<TokenType>) -> bool {
        if targets.contains(&self.peek().token_type) {
            return true;
        }

        false
    }

    /// Returns the token currently being parsed
    fn peek(&self) -> &Token<'a> {
        self.tokens.get(self.cur).unwrap_or(&Token {
            token_type: TokenType::EOF,
            line: 0,
            lexeme: None,
        })
    }

    /// Returns the token at an offset of `step` from the token being parsed.
    /// ```
    /// look_ahead(0).unwrap() == peek()
    /// ```
    fn _look_ahead(&self, step: usize) -> Option<&Token<'a>> {
        self.tokens.get(self.cur + step)
    }

    /// Builds a parsing error, adds it to the error vector,
    /// and moves cur until the next recoverable position
    fn handle_error(&mut self, token: Token<'a>, msg: String) {
        self.errors.push(RoxError::new(token, msg));
        while !self.is_at_end()
            && !self.equals_any(vec![
                TokenType::Semicolon,
                TokenType::RightBrace,
                TokenType::RightParen,
            ])
        {
            self.next();
        }
    }
}

fn infix_binding_power(token_type: TokenType) -> (usize, usize) {
    match token_type {
        TokenType::Plus | TokenType::Minus => (10, 11),
        TokenType::Star | TokenType::Slash => (20, 21),
        _ => panic!("invalid infix token_type: '{}'", token_type),
    }
}

fn prefix_binding_power(token_type: TokenType) -> ((), usize) {
    match token_type {
        TokenType::Minus | TokenType::Plus => ((), 90),
        TokenType::Bang => ((), 100),
        _ => panic!("invalid prefix token_type: '{}'", token_type),
    }
}

#[cfg(test)]
mod tests {
    use crate::scanner::scanner::Scanner;

    use super::*;

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
        assert!(matches!(node, Node::Literal(_)));
    }

    #[test]
    fn parse_identifier() {
        let tokens = scan("myVar;");
        let mut parser = Parser::new(tokens);
        let node = parser.parse();

        assert_eq!(parser.has_errors(), false, "Should not have parsing errors");
        assert!(matches!(node, Node::Var(_)));
    }

    #[test]
    fn parse_binop() {
        let tokens = scan("2 + 3;");
        let mut parser = Parser::new(tokens);
        let node = parser.parse();

        assert_eq!(parser.has_errors(), false, "Should not have parsing errors");
        assert!(matches!(node, Node::BinOp(_)));
    }

    #[test]
    fn parse_complex_binop() {
        let tokens = scan("2 + 3 * 4 + 5 * 6;");
        let mut parser = Parser::new(tokens);
        let node = parser.parse();

        assert_eq!(parser.has_errors(), false, "Should not have parsing errors");
        assert!(matches!(node, Node::BinOp(_)));
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

        match &node {
            Node::BinOp(bin) => {
                assert!(matches!(*bin.right, Node::Error))
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
        assert!(matches!(node, Node::Grouping(_)));
    }

    #[test]
    fn parse_arithmetic_with_group() {
        let tokens = scan("(3 + 2) * 10;");
        let mut parser = Parser::new(tokens);
        let node = parser.parse();

        assert!(!parser.has_errors());
        match &node {
            Node::BinOp(bin) => {
                assert!(matches!(*bin.left, Node::Grouping(_)));
                assert!(matches!(*bin.right, Node::Literal(_)));
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
        match &node {
            Node::Unary(unary) => {
                assert!(matches!(unary.op.token_type, TokenType::Minus));
                assert!(matches!(*unary.operand, Node::Literal(_)));
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
        match &node {
            Node::Unary(unary) => {
                assert!(matches!(unary.op.token_type, TokenType::Minus));
                assert!(matches!(*unary.operand, Node::Unary(_)));
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
        match &node {
            Node::Unary(unary) => {
                assert!(matches!(unary.op.token_type, TokenType::Minus));
                assert!(matches!(*unary.operand, Node::Grouping(_)));
            }
            _ => panic!("Should be unary"),
        }
    }

    #[test]
    fn parse_complex() {
        let tokens = scan("-(42 + 10) + 27 / (10 + (b * myVar));");
        let mut parser = Parser::new(tokens);
        let node = parser.parse();

        assert!(!parser.has_errors());
    }
}
