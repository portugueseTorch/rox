#[macro_export]
macro_rules! token {
    ($scanner:expr, $tok_type:expr) => {
        Ok(Token::new($tok_type, $scanner.line, None))
    };
    ($scanner:expr, $tok_type:expr, $len:expr) => {
        Ok(Token::new(
            $tok_type,
            $scanner.line,
            Some(&$scanner.src[$scanner.start..$scanner.start + $len]),
        ))
    };
}

#[macro_export]
macro_rules! scanning_error {
    ($scanner:expr) => {
        anyhow::bail!(
            "scanning error in line {} at {}",
            $scanner.line,
            &$scanner.src[$scanner.start..$scanner.cur]
        )
    };
    ($scanner:expr, $err:expr) => {
        anyhow::bail!(
            "scanning error in line {} at {}: {}",
            $scanner.line,
            &$scanner.src[$scanner.start..$scanner.cur],
            $err
        )
    };
}

#[derive(Debug, Clone)]
pub struct Token<'a> {
    /// lexeme info
    pub lexeme: Option<&'a str>,
    /// line of the token
    pub line: usize,
    pub token_type: TokenType,
}

impl<'a> Token<'a> {
    pub fn new(token_type: TokenType, line: usize, lexeme: Option<&'a str>) -> Self {
        Self {
            lexeme,
            line,
            token_type,
        }
    }
}

#[derive(Debug, Clone, Copy, Eq, PartialEq, Hash)]
pub enum TokenType {
    LeftParen,
    RightParen,
    LeftBrace,
    RightBrace,
    Comma,
    Dot,
    Minus,
    Plus,
    Semicolon,
    Slash,
    Star,
    //
    Bang,
    BangEqual,
    Equal,
    EqualEqual,
    Greater,
    GreaterEqual,
    Less,
    LessEqual,
    //
    Identifier,
    StringLiteral,
    Number,
    //
    And,
    Class,
    Else,
    False,
    For,
    Fun,
    If,
    Nil,
    Or,
    Print,
    Return,
    Super,
    This,
    True,
    Var,
    While,
    //
    EOF,
    Error,
}
