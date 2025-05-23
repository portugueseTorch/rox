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
            '"' => return self.string(),
            '0'..='9' => return self.number(),
            'A'..='Z' | 'a'..='z' | '_' => return self.identifier(),
            _ => {}
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

    /// returns the len of the token being currently processed
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

    fn string(&mut self) -> anyhow::Result<Token<'a>> {
        while !self.is_at_end() && self.peek().unwrap() != '"' {
            // --- increase line number if we're at a new line
            if self.peek().unwrap() == '\n' {
                self.line += 1;
            }

            self.advance();
        }

        // --- fi we're at the end, we have an unterminated string
        if self.is_at_end() {
            scanning_error!(self, "unterminated string");
        }

        token!(self, TokenType::String, self.cur_span())
    }

    fn number(&mut self) -> anyhow::Result<Token<'a>> {
        while !self.is_at_end() && self.peek().unwrap().is_digit(10) {
            self.advance();
        }

        // check for decimal points
        if self.matches('.') {
            while !self.is_at_end() && self.peek().unwrap().is_digit(10) {
                self.advance();
            }
        }

        token!(self, TokenType::Number, self.cur_span())
    }

    fn identifier(&mut self) -> anyhow::Result<Token<'a>> {
        // --- scan the full word and try to match it afterwards
        while !self.is_at_end() && is_alphanumeric(self.peek().unwrap()) {
            self.advance();
        }

        return self.make_identifier();
    }

    fn make_identifier(&mut self) -> anyhow::Result<Token<'a>> {
        let identifier = &self.src[self.start..self.cur];

        match identifier {
            "and" => token!(self, TokenType::And, identifier.len()),
            "class" => token!(self, TokenType::Class, identifier.len()),
            "else" => token!(self, TokenType::Else, identifier.len()),
            "false" => token!(self, TokenType::False, identifier.len()),
            "for" => token!(self, TokenType::For, identifier.len()),
            "fun" => token!(self, TokenType::Fun, identifier.len()),
            "if" => token!(self, TokenType::If, identifier.len()),
            "nil" => token!(self, TokenType::Nil, identifier.len()),
            "or" => token!(self, TokenType::Or, identifier.len()),
            "print" => token!(self, TokenType::Print, identifier.len()),
            "return" => token!(self, TokenType::Return, identifier.len()),
            "super" => token!(self, TokenType::Super, identifier.len()),
            "this" => token!(self, TokenType::This, identifier.len()),
            "true" => token!(self, TokenType::True, identifier.len()),
            "var" => token!(self, TokenType::Var, identifier.len()),
            "while" => token!(self, TokenType::While, identifier.len()),
            _ => token!(self, TokenType::Identifier, identifier.len()),
        }
    }
}

fn is_alphanumeric(val: char) -> bool {
    return val.is_alphanumeric() || val == '_';
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn simple_scan() {
        let mut scanner = Scanner::new("this is a simple scan");
        assert!(scanner.scan().expect("This should be a valid scan") == ());
    }

    #[test]
    fn scan_empty_src() {
        let mut scanner = Scanner::new("");
        let token = scanner
            .scan_token()
            .expect("Should emit at least one token");
        assert_eq!(token.token_type, TokenType::EOF);
    }

    #[test]
    fn unknown_character() {
        let mut scanner = Scanner::new("#");
        match scanner.scan_token() {
            Err(_) => {}
            _ => panic!("# is not a valid char"),
        }
    }

    #[test]
    fn scan_reserved_keywords() {
        let mut scanner = Scanner::new("and class else");
        let token = scanner.scan_token().unwrap();
        assert_eq!(token.token_type, TokenType::And);
        let token = scanner.scan_token().unwrap();
        assert_eq!(token.token_type, TokenType::Class);
        let token = scanner.scan_token().unwrap();
        assert_eq!(token.token_type, TokenType::Else);
    }

    #[test]
    fn scan_identifier() {
        let mut scanner = Scanner::new("Hello");
        let token = scanner.scan_token().unwrap();
        assert_eq!(token.token_type, TokenType::Identifier);
        assert_eq!(
            token.lexeme.expect("identifier should have a lexeme"),
            "Hello"
        );
    }

    #[test]
    fn scan_number() {
        let mut scanner = Scanner::new("1337");
        let token = scanner.scan_token().unwrap();
        assert_eq!(token.token_type, TokenType::Number);
        assert_eq!(
            token.lexeme.expect("identifier should have a lexeme"),
            "1337"
        );
    }

    #[test]
    fn scan_decimal_number() {
        let mut scanner = Scanner::new("1337.42");
        let token = scanner.scan_token().unwrap();
        assert_eq!(token.token_type, TokenType::Number);
        assert_eq!(
            token.lexeme.expect("identifier should have a lexeme"),
            "1337.42"
        );
    }

    #[test]
    fn scan_whitespaces() {
        let mut scanner = Scanner::new("      \t\r\n");
        let token = scanner.scan_token().unwrap();
        assert_eq!(token.token_type, TokenType::EOF);
    }

    #[test]
    fn scan_whitespaces_and_comment() {
        let mut scanner = Scanner::new("      \t\r\n// this is a comment and should be ignored\n// this should also be a comment even though afterwards we simply get EOF");
        let token = scanner.scan_token().unwrap();
        assert_eq!(token.token_type, TokenType::EOF);
        assert_eq!(token.line, 3);
    }
}
