use crate::{
    errors::RoxError,
    scanner::token::{Token, TokenType},
};

use super::ast::{Node, Value};

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
        // TODO: implement statement parsing
        self.parse_expr()
    }

    fn parse_expr(&mut self) -> Node<'a> {
        let tok = self.next();

        let mut lhs = match tok.token_type {
            TokenType::Number => {
                let num_as_str = tok.lexeme.unwrap();
                let parsed_num = num_as_str.parse().unwrap();
                Node::Literal(Value::Number(parsed_num))
            }
            TokenType::StringLiteral => Node::Literal(Value::StringLiteral(tok.lexeme.unwrap())),
            TokenType::Identifier => Node::Var(tok.lexeme.unwrap()),
            _ => Node::Error,
        };
        if lhs.is_error() {
            let error_token = self.prev().unwrap();
            self.log_error(
                error_token.clone(),
                Some(format!("unexpected token: {:?}", error_token.token_type)),
            );
            return lhs;
        }

        unimplemented!();
    }

    pub fn has_errors(&self) -> bool {
        !self.errors.is_empty()
    }
}

/**
* Utils
*/
impl<'a> Parser<'a> {
    /// Advances cur and returns the previous token
    fn next(&mut self) -> &Token<'a> {
        let token = &self.tokens[self.cur];
        self.cur += 1;
        token
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
        if self.tokens[self.cur].token_type == token_type {
            self.next();
            return;
        }

        self.log_error(
            self.tokens[self.cur].clone(),
            Some(format!(
                "unexpected token type: expected {:?} but got {:?}",
                token_type, self.tokens[self.cur].token_type
            )),
        );
    }

    fn matches(&self, target: TokenType) -> bool {
        if self.peek().token_type == target {
            return true;
        }

        false
    }

    fn matches_any(&self, targets: Vec<TokenType>) -> bool {
        if targets.contains(&self.peek().token_type) {
            return true;
        }

        false
    }

    /// Returns the token currently being parsed
    fn peek(&self) -> &Token<'a> {
        &self.tokens[self.cur]
    }

    /// Returns the token at an offset of `step` from the token being parsed.
    /// ```
    /// look_ahead(0).unwrap() == peek()
    /// ```
    fn look_ahead(&self, step: usize) -> Option<&Token<'a>> {
        self.tokens.get(self.cur + step)
    }

    /// Builds a parsing error, adds it to the error vector,
    /// and moves cur until the next recoverable position
    fn log_error(&mut self, token: Token<'a>, msg: Option<String>) {
        self.errors.push(RoxError::ParsingError(token, msg));
        while !self.matches_any(vec![
            TokenType::Semicolon,
            TokenType::RightBrace,
            TokenType::RightParen,
        ]) {
            self.next();
        }
    }
}
