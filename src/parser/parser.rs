use crate::{
    errors::RoxError,
    scanner::token::{Token, TokenType},
};

use super::ast::{AssignmentExpr, BinaryExpr, Node, NodeType, UnaryExpr, Value};

macro_rules! parsing_error {
    ($parser:expr, $tok:expr, $msg:expr) => {
        $parser.handle_error($tok.clone(), $msg);
        return Node::new($tok.clone(), NodeType::Error);
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
        self.expect(TokenType::Semicolon);
        expr
    }

    fn parse_expr(&mut self, bp: usize) -> Node<'a> {
        let tok = self.next().clone();
        let lhs = match tok.token_type {
            TokenType::StringLiteral => {
                NodeType::Constant(Value::StringLiteral(tok.lexeme.unwrap()))
            }
            TokenType::Identifier => NodeType::Var(tok.lexeme.unwrap()),
            TokenType::Minus | TokenType::Plus | TokenType::Bang => {
                let (_, rbp) = prefix_binding_power(tok.token_type);
                let operand = self.parse_expr(rbp);
                NodeType::Unary(UnaryExpr {
                    op: tok.token_type,
                    operand: Box::new(operand),
                })
            }
            TokenType::True | TokenType::False => {
                let parsed_bool: bool = tok.lexeme.unwrap().parse().unwrap();
                NodeType::Constant(Value::Bool(parsed_bool))
            }
            TokenType::Number => {
                let num_as_str = tok.lexeme.unwrap();
                let parsed_num = num_as_str.parse().unwrap();
                NodeType::Constant(Value::Number(parsed_num))
            }
            TokenType::LeftParen => {
                let group_expr = self.parse_expr(0);
                if !group_expr.node.is_error() && !self.matches(TokenType::RightParen) {
                    parsing_error!(
                        self,
                        self.prev().unwrap(),
                        format!(
                            "unexpected token: expected '(' but got '{}'",
                            self.prev().unwrap().token_type
                        )
                    );
                }

                NodeType::Grouping(Box::new(group_expr))
            }
            _ => NodeType::Error,
        };

        // --- on error, return
        if lhs.is_error() {
            parsing_error!(
                self,
                self.prev().unwrap(),
                format!("unexpected token: '{}'", self.prev().unwrap().token_type)
            );
        }

        // --- build AST node
        let mut lhs = Node::new(tok.clone(), lhs);

        loop {
            let op = self.peek().clone();
            let op_type = op.token_type;
            let (lbp, rbp) = match op_type {
                TokenType::Plus
                | TokenType::Minus
                | TokenType::Star
                | TokenType::Slash
                | TokenType::Equal
                | TokenType::And
                | TokenType::Or => infix_binding_power(op.token_type),
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

            // --- emit ast node based on the type of the operator
            lhs = match op_type {
                TokenType::Equal => {
                    // --- left hand side needs to be an identifier
                    if !matches!(lhs.node, NodeType::Var(_)) {
                        parsing_error!(self, lhs.token, "invalid variable assignment".to_string());
                    }

                    // --- if the right hand side is an assignment, this is also invalid
                    if matches!(rhs.node, NodeType::Assignment(_)) {
                        parsing_error!(
                            self,
                            lhs.token,
                            "invalid chaining of assignments".to_string()
                        );
                    }

                    Node::new(
                        op,
                        NodeType::Assignment(AssignmentExpr {
                            name: lhs.token,
                            expr: Box::new(rhs),
                        }),
                    )
                }
                _ => Node::new(
                    op,
                    NodeType::BinOp(BinaryExpr {
                        left: Box::new(lhs),
                        right: Box::new(rhs),
                        op: op_type,
                    }),
                ),
            };
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

// --- Utils
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
    fn expect(&mut self, token_type: TokenType) {
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
        TokenType::Equal => (5, 6),
        TokenType::Or => (7, 8),
        TokenType::And => (9, 10),
        TokenType::Plus | TokenType::Minus => (20, 21),
        TokenType::Star | TokenType::Slash => (30, 31),
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
