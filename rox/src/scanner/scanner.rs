use super::token::{Token, TokenType};
use crate::{scanning_error, token};

macro_rules! if_then {
    ($cond:expr, $true:expr, $false:expr) => {
        if $cond {
            $true
        } else {
            $false
        }
    };
}

#[derive(Debug)]
pub struct Scanner<'a> {
    src: &'a str,
    /// anchor point of the token being scanned
    start: usize,
    /// iterator over src, points to the next char to be scanned
    cur: usize,
    line: usize,
}

impl<'a> Scanner<'a> {
    pub fn new(src: &'a str) -> Self {
        Self {
            src,
            start: 0,
            cur: 0,
            line: 1,
        }
    }

    pub fn scan(&mut self) -> anyhow::Result<()> {
        Ok(())
    }

    pub fn scan_token(&mut self) -> anyhow::Result<Token<'a>> {
        self.skip_whitespaces();

        // --- point start to the current token
        self.start = self.cur;

        // --- if we are at the end of the file, emit a EOF token
        if self.is_at_end() {
            return token!(self, TokenType::EOF);
        }

        match self.advance().unwrap() {
            '(' => return token!(self, TokenType::LeftParen, 1),
            ')' => return token!(self, TokenType::RightParen, 1),
            '{' => return token!(self, TokenType::LeftBrace, 1),
            '}' => return token!(self, TokenType::RightBrace, 1),
            ';' => return token!(self, TokenType::Semicolon, 1),
            ',' => return token!(self, TokenType::Comma, 1),
            '.' => return token!(self, TokenType::Dot, 1),
            '-' => return token!(self, TokenType::Minus, 1),
            '+' => return token!(self, TokenType::Plus, 1),
            '/' => return token!(self, TokenType::Slash, 1),
            '*' => return token!(self, TokenType::Star, 1),
            '!' => {
                return token!(
                    self,
                    if_then!(self.matches('='), TokenType::BangEqual, TokenType::Bang),
                    self.cur_span()
                )
            }
            '<' => {
                return token!(
                    self,
                    if_then!(self.matches('='), TokenType::LessEqual, TokenType::Less),
                    self.cur_span()
                )
            }
            '>' => {
                return token!(
                    self,
                    if_then!(
                        self.matches('='),
                        TokenType::GreaterEqual,
                        TokenType::Greater
                    ),
                    self.cur_span()
                )
            }
            '=' => {
                return token!(
                    self,
                    if_then!(self.matches('='), TokenType::EqualEqual, TokenType::Equal),
                    self.cur_span()
                )
            }
            _ => unimplemented!(),
        }

        scanning_error!(self)
    }

    fn is_at_end(&self) -> bool {
        self.cur >= self.src.len()
    }

    /// consumes the character and itereates it
    fn advance(&mut self) -> Option<char> {
        if self.is_at_end() {
            return None;
        }

        let rest = &self.src[self.cur..];
        let (_, char) = rest.char_indices().next()?;
        self.cur += char.len_utf8();

        Some(char)
    }

    /// returns true and iterates if the next char matches c, returning false otherwise
    fn matches(&mut self, c: char) -> bool {
        if self.is_at_end() {
            return false;
        }

        if self.peek().unwrap() != c {
            return false;
        }

        // --- iterate cur
        self.cur += 1;
        true
    }

    /// peeks at the next character in the source string
    fn peek(&self) -> Option<char> {
        if self.is_at_end() {
            return None;
        }
        let rest = &self.src[self.cur..];
        rest.chars().next()
    }

    fn peek_next(&self) -> Option<char> {
        if self.is_at_end() {
            return None;
        }
        let rest = &self.src[self.cur..];
        rest.chars().nth(1)
    }

    /// returns the span of the token being currently processed
    fn cur_span(&self) -> usize {
        self.cur - self.start
    }

    fn skip_whitespaces(&mut self) {
        loop {
            let c = self.peek();
            if c.is_none() {
                break;
            }

            match c.unwrap() {
                ' ' | '\r' | '\t' => {
                    self.advance();
                }
                '\n' => {
                    self.line += 1;
                    self.advance();
                }
                '/' => {
                    if let Some('/') = self.peek_next() {
                        while !self.is_at_end() && self.peek().unwrap() != '\n' {
                            self.advance();
                        }
                    } else {
                        break;
                    }
                }
                _ => break,
            };
        }
    }
}
