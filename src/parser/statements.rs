use std::fmt::Display;

use super::ast::ExprNode;

pub struct IfStmt<'a> {
    pub condition: ExprNode<'a>,
    pub if_body: Vec<Stmt<'a>>,
    /// is an empty vector if there is no specified else
    pub else_body: Vec<Stmt<'a>>,
}

pub enum Stmt<'a> {
    /// Single expression
    Expression(ExprNode<'a>),

    /// If statement containing
    ///   - expression for the if
    ///   - list of expressions for the if body
    ///   - list of expressions for the else body (if any)
    If(IfStmt<'a>),
}

impl<'a> Stmt<'a> {
    pub fn log(&self) {
        println!("{}", self);
    }
    fn to_yaml(&self, level: usize) -> String {
        let spaces = " ".repeat(level * 2);
        let next_level = level + 1;
        let indent = " ".repeat(next_level * 2);

        match self {
            Stmt::If(data) => {
                let mut s = format!("{}IfCondition:\n", spaces);
                s += &format!(
                    "{}Condition:\n{}",
                    indent,
                    data.condition.node.to_yaml(next_level + 1)
                );
                s += &format!("\n{}If:", indent);
                for stmt in data.if_body.iter() {
                    s += &format!("\n{}{}\n", indent, stmt.to_yaml(next_level).trim_end());
                }
                if !data.else_body.is_empty() {
                    s += &format!("\n{}Else:", indent);
                    for stmt in data.else_body.iter() {
                        s += &format!("\n{}{}\n", indent, stmt.to_yaml(next_level).trim_end());
                    }
                }
                s.trim_end().to_string()
            }

            Stmt::Expression(expr) => {
                format!(
                    "{}Expr:\n{}",
                    spaces,
                    expr.node.to_yaml(next_level + 1).trim_end()
                )
            }
        }
    }
}

impl<'a> Display for Stmt<'a> {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "\n{}\n", self.to_yaml(0))
    }
}

#[cfg(test)]
mod tests {
    use crate::{
        parser::{ast::Expr, parser::Parser},
        scanner::{
            scanner::Scanner,
            token::{Token, TokenType},
        },
    };

    fn scan<'a>(src: &'a str) -> Vec<Token<'a>> {
        let mut scanner = Scanner::new(src);
        let tokens = scanner.scan().unwrap();
        tokens
    }

    #[test]
    fn parse_if_stmt() {
        let tokens = scan(
            "if (true) {
                42;
            }",
        );
        let mut parser = Parser::new(tokens);
        let statements = parser.parse();

        assert!(!parser.has_errors());
        assert!(statements.len() == 1);
    }

    #[test]
    fn parse_if_else_stmt() {
        let tokens = scan(
            "if (42 + 4 > 10) {
                42;
            } else {
                self.wrong();
            }",
        );
        let mut parser = Parser::new(tokens);
        let statements = parser.parse();
        statements.iter().for_each(|f| println!("{}", f));

        assert!(!parser.has_errors());
        assert!(statements.len() == 1);
    }
}
