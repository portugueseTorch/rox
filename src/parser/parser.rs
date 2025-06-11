use crate::{
    errors::RoxError,
    scanner::token::{Token, TokenType},
};

use super::{
    ast::ExprNode,
    expressions::{
        AssignmentExpr, BinaryExpr, CallExpr, Expr, PropertyAccessExpr, UnaryExpr, Value,
    },
    statements::{
        ClassDeclStatement, ForStmt, FuncDeclStatement, IfStmt, ReturnStmt, Stmt, VarDeclStatement,
        WhileStmt,
    },
};

macro_rules! parsing_error {
    ($parser:expr, $tok:expr, $msg:expr) => {
        $parser.handle_error($tok.clone(), $msg);
        return ExprNode::new($tok.clone(), Expr::Error);
    };
}
macro_rules! valid_infix_op {
    () => {
        TokenType::Plus
            | TokenType::Minus
            | TokenType::Star
            | TokenType::Slash
            | TokenType::Equal
            | TokenType::Less
            | TokenType::LessEqual
            | TokenType::Greater
            | TokenType::GreaterEqual
            | TokenType::EqualEqual
            | TokenType::BangEqual
            | TokenType::And
            | TokenType::Or
            | TokenType::Dot
            | TokenType::LeftParen
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

    pub fn parse(&mut self) -> Vec<Stmt<'a>> {
        let mut statements = vec![];
        while !self.is_at_end() {
            let stmt = self.parse_statement(true);
            statements.push(stmt);
        }

        statements
    }

    fn parse_statement(&mut self, expect_semicolon: bool) -> Stmt<'a> {
        // --- match on token type
        match self.peek().token_type {
            TokenType::If => self.parse_if(),
            TokenType::While => self.parse_while(),
            TokenType::For => self.parse_for(),
            TokenType::Var => self.parse_var_decl(expect_semicolon),
            TokenType::Return => self.parse_return(),
            TokenType::Fun => self.parse_func_decl(),
            TokenType::Class => self.parse_class_decl(),
            _ => Stmt::Expression(self.parse_expression(expect_semicolon)),
        }
    }

    fn parse_class_decl(&mut self) -> Stmt<'a> {
        self.next();

        // --- parse class name
        let name = self.next().clone();
        if !matches!(name.token_type, TokenType::Identifier) {
            self.handle_error(
                name.clone(),
                format!(
                    "unexpected token: expected 'IDENT' but got '{}'",
                    name.token_type
                ),
            );

            return Stmt::Error;
        }

        self.expect(TokenType::LeftBrace);

        // --- parse methods
        let mut methods = vec![];
        while !self.is_at_end() && !matches!(self.peek().token_type, TokenType::RightBrace) {
            let stmt = self.parse_statement(true);
            match stmt {
                Stmt::FuncDecl(decl) => methods.push(decl),
                _ => {
                    self.handle_error(
                        name.clone(),
                        format!(
                            "unexpected token: expected 'IDENT' but got '{}'",
                            name.token_type
                        ),
                    );

                    return Stmt::Error;
                }
            };
        }

        self.expect(TokenType::RightBrace);

        Stmt::ClassDecl(ClassDeclStatement { name, methods })
    }

    fn parse_func_decl(&mut self) -> Stmt<'a> {
        self.next();

        // --- parse function name
        let name = self.next().clone();
        if !matches!(name.token_type, TokenType::Identifier) {
            self.handle_error(
                name.clone(),
                format!(
                    "unexpected token: expected 'IDENT' but got '{}'",
                    name.token_type
                ),
            );

            return Stmt::Error;
        }

        self.expect(TokenType::LeftParen);

        // --- parse parameters, if any
        let mut params = vec![];
        while !self.is_at_end() && !matches!(self.peek().token_type, TokenType::RightParen) {
            let param = self.parse_expr(0);

            // --- params should all be vars
            if !matches!(param.node, Expr::Var(_)) {
                self.handle_error(
                    name.clone(),
                    format!(
                        "unexpected token: expected 'IDENT' but got '{}'",
                        name.token_type
                    ),
                );

                return Stmt::Error;
            }

            params.push(param.token.clone());
            self.matches(TokenType::Comma);
        }

        self.expect(TokenType::RightParen);
        self.expect(TokenType::LeftBrace);

        // --- parse body
        let mut body = vec![];
        while !self.is_at_end() && !matches!(self.peek().token_type, TokenType::RightBrace) {
            let stmt = self.parse_statement(true);
            body.push(stmt);
        }

        self.expect(TokenType::RightBrace);

        Stmt::FuncDecl(FuncDeclStatement {
            name,
            parameters: params,
            body,
        })
    }

    fn parse_return(&mut self) -> Stmt<'a> {
        self.next();
        let mut value = None;

        // --- parse return expression, if any
        if !self.matches(TokenType::Semicolon) {
            value = Some(self.parse_expression(true));
        }

        Stmt::Return(ReturnStmt { value })
    }

    fn parse_var_decl(&mut self, expect_semicolon: bool) -> Stmt<'a> {
        self.next();

        let var_name = self.next().clone();

        // --- if the token is not an identifier, error
        if !matches!(var_name.token_type, TokenType::Identifier) {
            self.handle_error(
                var_name.clone(),
                format!(
                    "unexpected token: expected 'IDENT' but got '{}'",
                    var_name.token_type
                ),
            );

            return Stmt::Error;
        }

        // --- parse initializer, if any
        let mut initializer = None;
        if self.matches(TokenType::Equal) {
            initializer = Some(self.parse_expression(expect_semicolon));
        }

        Stmt::VarDecl(VarDeclStatement {
            var_name,
            initializer,
        })
    }

    fn parse_for(&mut self) -> Stmt<'a> {
        self.next();

        // --- expect next token to be left paren
        self.expect(TokenType::LeftParen);

        // --- try to parse initializer
        let mut initializer = None;
        if !matches!(self.peek().token_type, TokenType::Semicolon) {
            initializer = Some(Box::new(self.parse_statement(false)));
        }
        // --- expect next token to be semicolon
        self.expect(TokenType::Semicolon);

        // --- try to parse condition
        let mut condition = None;
        if !matches!(self.peek().token_type, TokenType::Semicolon) {
            condition = Some(self.parse_expr(0));
        }
        // --- expect next token to be semicolon
        self.expect(TokenType::Semicolon);

        // --- try to parse increment
        let mut increment = None;
        if !matches!(self.peek().token_type, TokenType::RightParen) {
            increment = Some(self.parse_expr(0));
        }
        // --- expect next token to be semicolon followed by ')' and '{'
        self.expect(TokenType::RightParen);
        self.expect(TokenType::LeftBrace);

        let mut body = vec![];
        while !self.is_at_end() && !matches!(self.peek().token_type, TokenType::RightBrace) {
            let stmt = self.parse_statement(true);
            body.push(stmt);
        }

        self.expect(TokenType::RightBrace);

        Stmt::For(ForStmt {
            initializer,
            condition,
            increment,
            body,
        })
    }

    fn parse_while(&mut self) -> Stmt<'a> {
        self.next();

        // --- expect next token to be left paren
        self.expect(TokenType::LeftParen);

        // --- parse if expression
        let condition = self.parse_expr(0);

        // --- expect next token to be a right paren followed by a left brace
        self.expect(TokenType::RightParen);
        self.expect(TokenType::LeftBrace);

        let mut body = vec![];
        while !self.is_at_end() && !matches!(self.peek().token_type, TokenType::RightBrace) {
            let stmt = self.parse_statement(true);
            body.push(stmt);
        }

        // --- expect a curly brace on the right
        self.expect(TokenType::RightBrace);

        Stmt::While(WhileStmt { condition, body })
    }

    fn parse_if(&mut self) -> Stmt<'a> {
        self.next();

        // --- expect next token to be left paren
        self.expect(TokenType::LeftParen);

        // --- parse if expression
        let condition = self.parse_expr(0);

        // --- expect next token to be a right paren followed by a left brace
        self.expect(TokenType::RightParen);
        self.expect(TokenType::LeftBrace);

        // --- parse if body
        let mut if_body = vec![];
        while !self.is_at_end() && !matches!(self.peek().token_type, TokenType::RightBrace) {
            let stmt = self.parse_statement(true);
            if_body.push(stmt);
        }

        // --- expect a curly brace on the right
        self.expect(TokenType::RightBrace);

        // --- check presence of else block
        let mut else_body = vec![];
        if self.matches(TokenType::Else) {
            // --- expecte next token to be a left brace
            self.expect(TokenType::LeftBrace);

            while !self.is_at_end() && !matches!(self.peek().token_type, TokenType::RightBrace) {
                let stmt = self.parse_statement(true);
                else_body.push(stmt);
            }

            // --- expect a curly brace on the right
            self.expect(TokenType::RightBrace);
        }

        Stmt::If(IfStmt {
            condition,
            if_body,
            else_body,
        })
    }

    pub fn parse_expression(&mut self, expect_semicolon: bool) -> ExprNode<'a> {
        let expr = self.parse_expr(0);
        if expect_semicolon {
            self.expect(TokenType::Semicolon);
        }
        expr
    }

    fn parse_expr(&mut self, bp: usize) -> ExprNode<'a> {
        let tok = self.next().clone();
        let lhs = match tok.token_type {
            TokenType::StringLiteral => {
                Expr::Constant(Value::StringLiteral(tok.lexeme.unwrap().to_string()))
            }
            TokenType::Identifier => Expr::Var(tok.lexeme.unwrap()),
            TokenType::Minus | TokenType::Plus | TokenType::Bang => {
                let (_, rbp) = prefix_binding_power(tok.token_type);
                let operand = self.parse_expr(rbp);
                Expr::Unary(UnaryExpr {
                    op: tok.token_type,
                    operand: Box::new(operand),
                })
            }
            TokenType::True | TokenType::False => {
                let parsed_bool: bool = tok.lexeme.unwrap().parse().unwrap();
                Expr::Constant(Value::Bool(parsed_bool))
            }
            TokenType::Number => {
                let num_as_str = tok.lexeme.unwrap();
                let parsed_num = num_as_str.parse().unwrap();
                Expr::Constant(Value::Number(parsed_num))
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

                Expr::Grouping(Box::new(group_expr))
            }
            _ => Expr::Error,
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
        let mut lhs = ExprNode::new(tok.clone(), lhs);

        loop {
            let op = self.peek().clone();
            let op_type = match op.token_type {
                valid_infix_op!() => op.token_type,
                TokenType::EOF
                | TokenType::Semicolon
                | TokenType::RightParen
                | TokenType::Comma => break,
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

            // --- parse postfix expression, if appropriate
            if let Some((lbp, rbp)) = postfix_binding_power(op_type) {
                if lbp < bp {
                    break;
                }

                lhs = self.parse_postfix_expression(rbp, lhs, op.clone());
            }

            // --- parse infix expression, if appropriate
            if let Some((lbp, rbp)) = infix_binding_power(op_type) {
                if lbp < bp {
                    break;
                }

                lhs = self.parse_infix_expression(rbp, lhs, op.clone());
            }
        }

        lhs
    }

    fn parse_postfix_expression(
        &mut self,
        bp: usize,
        lhs: ExprNode<'a>,
        op: Token<'a>,
    ) -> ExprNode<'a> {
        self.next();

        match &op.token_type {
            TokenType::Dot => {
                let rhs = self.parse_expr(bp);

                ExprNode::new(
                    rhs.token.clone(),
                    Expr::PropertyAccess(PropertyAccessExpr {
                        object: Box::new(lhs),
                        property: rhs.token,
                    }),
                )
            }
            TokenType::LeftParen => {
                // --- while we are not at the end and current token is not a right brace, keep parsing
                let mut args = vec![];
                while !self.is_at_end() && !matches!(self.peek().token_type, TokenType::RightParen)
                {
                    let expr = self.parse_expr(0);
                    args.push(expr);

                    // --- if the current token is a comma, advance
                    self.matches(TokenType::Comma);
                }

                // --- on exit, we should have a right paren for a correct function call
                self.expect(TokenType::RightParen);

                ExprNode::new(
                    op.clone(),
                    Expr::Call(CallExpr {
                        calee: Box::new(lhs),
                        args,
                    }),
                )
            }
            _ => panic!("Invalid postfix operator"),
        }
    }

    fn parse_infix_expression(
        &mut self,
        bp: usize,
        lhs: ExprNode<'a>,
        op: Token<'a>,
    ) -> ExprNode<'a> {
        // --- parse right hand side
        self.next();
        let rhs = self.parse_expr(bp);
        let token_type = op.token_type;

        // --- emit ast node based on the type of the operator
        match &op.token_type {
            TokenType::Equal => {
                // --- left hand side needs to be an identifier
                if !matches!(lhs.node, Expr::Var(_)) {
                    parsing_error!(self, lhs.token, "invalid variable assignment".to_string());
                }

                // --- if the right hand side is an assignment, this is also invalid
                if matches!(rhs.node, Expr::Assignment(_)) {
                    parsing_error!(
                        self,
                        lhs.token,
                        "invalid chaining of assignments".to_string()
                    );
                }

                ExprNode::new(
                    op.clone(),
                    Expr::Assignment(AssignmentExpr {
                        name: lhs.token,
                        expr: Box::new(rhs),
                    }),
                )
            }
            _ => ExprNode::new(
                op,
                Expr::BinOp(BinaryExpr {
                    left: Box::new(lhs),
                    right: Box::new(rhs),
                    op: token_type,
                }),
            ),
        }
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
        matches!(self.peek().token_type, TokenType::EOF)
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

fn postfix_binding_power(token_type: TokenType) -> Option<(usize, usize)> {
    let res = match token_type {
        TokenType::LeftParen => (41, 42),
        TokenType::Dot => (51, 52),
        _ => return None,
    };

    Some(res)
}

fn infix_binding_power(token_type: TokenType) -> Option<(usize, usize)> {
    let res = match token_type {
        TokenType::Equal => (5, 6),
        TokenType::Or => (7, 8),
        TokenType::And => (9, 10),
        TokenType::EqualEqual | TokenType::BangEqual => (13, 14),
        TokenType::Less | TokenType::LessEqual | TokenType::Greater | TokenType::GreaterEqual => {
            (17, 18)
        }
        TokenType::Plus | TokenType::Minus => (21, 22),
        TokenType::Star | TokenType::Slash => (31, 32),
        _ => return None,
    };

    Some(res)
}

fn prefix_binding_power(token_type: TokenType) -> ((), usize) {
    match token_type {
        TokenType::Minus | TokenType::Plus => ((), 90),
        TokenType::Bang => ((), 100),
        _ => panic!("invalid prefix token_type: '{}'", token_type),
    }
}
