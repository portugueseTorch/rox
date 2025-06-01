use std::fmt::Display;

use super::ast::ExprNode;

pub struct IfStmt<'a> {
    pub condition: ExprNode<'a>,
    pub if_body: Vec<Stmt<'a>>,
    /// is an empty vector if there is no specified else
    pub else_body: Vec<Stmt<'a>>,
}

pub struct WhileStmt<'a> {
    pub condition: ExprNode<'a>,
    pub body: Vec<Stmt<'a>>,
}

pub struct ForStmt<'a> {
    /// optional initializer for the loop
    pub initializer: Option<Box<Stmt<'a>>>,
    /// optional condition for loop stoppage
    pub condition: Option<ExprNode<'a>>,
    /// optional incrementer
    pub increment: Option<ExprNode<'a>>,
    pub body: Vec<Stmt<'a>>,
}

pub struct VarDeclStatement<'a> {
    pub var_name: ExprNode<'a>,
    pub initializer: Option<ExprNode<'a>>,
}

pub enum Stmt<'a> {
    /// Single expression
    Expression(ExprNode<'a>),

    /// If statement containing
    ///   - expression for the if
    ///   - list of expressions for the if body
    ///   - list of expressions for the else body (if any)
    If(IfStmt<'a>),

    /// While statement containing
    ///   - expression for ending the loop
    ///   - body of the while loop
    While(WhileStmt<'a>),

    /// For statement containing
    ///   - initializer statement
    ///   - condition
    ///   - increment
    ///   - body
    For(ForStmt<'a>),

    /// Variable declaration
    ///   - var name as an expression node
    ///   - optional initializer
    VarDecl(VarDeclStatement<'a>),
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
                let mut s = format!("{}IfStmt:\n", spaces);
                s += &format!(
                    "{}Condition:\n{}",
                    indent,
                    data.condition.node.to_yaml(next_level + 1)
                );
                s += &format!("\n{}If:", indent);
                for stmt in data.if_body.iter() {
                    s += &format!("\n{}\n", stmt.to_yaml(next_level).trim_end());
                }
                if !data.else_body.is_empty() {
                    s += &format!("\n{}Else:", indent);
                    for stmt in data.else_body.iter() {
                        s += &format!("\n{}\n", stmt.to_yaml(next_level).trim_end());
                    }
                }
                s.trim_end().to_string()
            }

            Stmt::Expression(expr) => {
                format!("{}", expr.node.to_yaml(next_level).trim_end())
            }

            Stmt::While(data) => {
                let mut s = format!("{}WhileStmt:\n", spaces);
                s += &format!(
                    "{}Condition:\n{}",
                    indent,
                    data.condition.node.to_yaml(next_level + 1)
                );
                s += &format!("\n{}Body:", indent);
                for stmt in data.body.iter() {
                    s += &format!("\n{}\n", stmt.to_yaml(next_level).trim_end());
                }
                s.trim_end().to_string()
            }

            Stmt::For(data) => {
                let mut s = format!("{}ForStmt:\n", spaces);
                s += &format!(
                    "{}Initializer:\n{}",
                    indent,
                    data.initializer
                        .as_ref()
                        .map_or(format!("{}  None", indent), |node| node.to_yaml(next_level))
                );
                s += &format!(
                    "\n{}Condition:\n{}",
                    indent,
                    data.condition
                        .as_ref()
                        .map_or(format!("{}  None", indent), |node| node
                            .node
                            .to_yaml(next_level + 1))
                );
                s += &format!(
                    "\n{}Increment:\n{}",
                    indent,
                    data.increment
                        .as_ref()
                        .map_or(format!("{}  None", indent), |node| node
                            .node
                            .to_yaml(next_level + 1))
                );
                s += &format!("\n{}Body:", indent);
                for stmt in data.body.iter() {
                    s += &format!("\n{}\n", stmt.to_yaml(next_level).trim_end());
                }
                s.trim_end().to_string()
            }

            Stmt::VarDecl(data) => {
                let mut s = format!("{}VarDeclStmt:\n", spaces);
                s += &format!("{}Var: {}", indent, data.var_name.token.lexeme.unwrap());
                s += &format!(
                    "\n{}Initializer:\n{}",
                    indent,
                    data.initializer
                        .as_ref()
                        .map_or(format!("{}  None", indent), |node| node
                            .node
                            .to_yaml(next_level + 1))
                );
                s.trim_end().to_string()
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
        parser::{ast::Expr, parser::Parser, statements::Stmt},
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
                self.wrong(false, hello);
            }",
        );
        let mut parser = Parser::new(tokens);
        let statements = parser.parse();

        assert!(!parser.has_errors());
        assert!(statements.len() == 1);
        assert!(matches!(statements.get(0).unwrap(), Stmt::If(_)));
    }

    #[test]
    fn parse_while() {
        let tokens = scan(
            "
            while (i < 10) {
                i = i + 1;
            }",
        );
        let mut parser = Parser::new(tokens);
        let statements = parser.parse();

        assert!(!parser.has_errors());
        assert!(statements.len() == 1);
        assert!(matches!(statements.get(0).unwrap(), Stmt::While(_)));
    }

    #[test]
    fn parse_for() {
        let tokens = scan(
            "
            for (i = 1; i < 10; i = i + 1) {
                42 + a;
            }
            ",
        );
        let mut parser = Parser::new(tokens);
        let statements = parser.parse();

        assert!(!parser.has_errors());
        assert!(statements.len() == 1);
        assert!(matches!(statements.get(0).unwrap(), Stmt::For(_)));
    }

    #[test]
    fn parse_for_empty() {
        let tokens = scan(
            "
            for (;;) {
                42 + a;
            }
            ",
        );
        let mut parser = Parser::new(tokens);
        let statements = parser.parse();

        assert!(!parser.has_errors());
        assert!(statements.len() == 1);
        assert!(matches!(statements.get(0).unwrap(), Stmt::For(_)));
    }

    #[test]
    fn parse_for_first_empty() {
        let tokens = scan(
            "
            for (;i < 10; i = i + 1) {
                42 + a;
            }
            ",
        );
        let mut parser = Parser::new(tokens);
        let statements = parser.parse();
        statements.iter().for_each(|f| println!("{}", f));

        assert!(!parser.has_errors());
        assert!(statements.len() == 1);
        assert!(matches!(statements.get(0).unwrap(), Stmt::For(_)));
    }
}
