use std::fmt::Display;

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
    Bang,
    BangEqual,
    Equal,
    EqualEqual,
    Greater,
    GreaterEqual,
    Less,
    LessEqual,
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
    EOF,
    Error,
}

impl Display for TokenType {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let msg = match self {
            TokenType::LeftParen => "(",
            TokenType::RightParen => ")",
            TokenType::LeftBrace => "{",
            TokenType::RightBrace => "}",
            TokenType::Comma => ",",
            TokenType::Dot => ".",
            TokenType::Minus => "-",
            TokenType::Plus => "+",
            TokenType::Semicolon => ";",
            TokenType::Slash => "/",
            TokenType::Star => "*",
            TokenType::Bang => "!",
            TokenType::BangEqual => "!=",
            TokenType::Equal => "=",
            TokenType::EqualEqual => "==",
            TokenType::Greater => ">",
            TokenType::GreaterEqual => ">=",
            TokenType::Less => "<",
            TokenType::LessEqual => "<=",
            TokenType::Identifier => "IDENT",
            TokenType::StringLiteral => "LITERAL",
            TokenType::Number => "NUMBER",
            TokenType::And => "AND",
            TokenType::Class => "CLASS",
            TokenType::Else => "ELSE",
            TokenType::False => "FALSE",
            TokenType::For => "FOR",
            TokenType::Fun => "FUN",
            TokenType::If => "IF",
            TokenType::Nil => "NIL",
            TokenType::Or => "OR",
            TokenType::Print => "PRINT",
            TokenType::Return => "RETURN",
            TokenType::Super => "SUPER",
            TokenType::This => "THIS",
            TokenType::True => "TRUE",
            TokenType::Var => "VAR",
            TokenType::While => "WHILE",
            TokenType::EOF => "EOF",
            TokenType::Error => "ERROR",
        };
        write!(f, "{}", msg.to_string())
    }
}
