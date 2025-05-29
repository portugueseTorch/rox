use crate::scanner::token::{Token, TokenType};

#[derive(Debug, Clone)]
pub enum RoxError<'a> {
    SyntaxError(String),
    ParsingError(Token<'a>, Option<String>),
}
