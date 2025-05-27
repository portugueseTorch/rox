use crate::{
    errors::RoxError,
    scanner::token::{Token, TokenType},
};

pub struct Parser<'a> {
    cur: usize,
    tokens: Vec<Token<'a>>,
    has_errors: bool,
    errors: Vec<RoxError<'a>>,
}

impl<'a> Parser<'a> {
    pub fn new(tokens: Vec<Token<'a>>) -> Self {
        Parser {
            cur: 0,
            tokens,
            has_errors: false,
            errors: vec![],
        }
    }

    /// Advances cur and returns the previous token
    fn next(&mut self) -> &Token<'a> {
        let token = &self.tokens[self.cur];
        self.cur += 1;
        token
    }

    /// Asserts that the current token is of the provided type.
    /// If it is not sets the error flag to true and generates the appropriate error
    fn assert(&mut self, token_type: TokenType) {
        if self.tokens[self.cur].token_type == token_type {
            self.next();
            return;
        }

        self.has_errors = true;
        self.errors.push(RoxError::ParsingError(
            self.tokens[self.cur].clone(),
            token_type,
            None,
        ));
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
}
