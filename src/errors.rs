use std::fmt::Display;

use crate::scanner::token::Token;

#[derive(Debug, Clone)]
pub struct RoxError<'a> {
    pub token: Token<'a>,
    pub msg: String,
}

impl<'a> RoxError<'a> {
    pub fn new(token: Token<'a>, msg: String) -> Self {
        Self { token, msg }
    }
}

impl<'a> Display for RoxError<'a> {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "[ERROR]: at {}: {}", self.token.line, self.msg)
    }
}
