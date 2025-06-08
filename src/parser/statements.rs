use std::fmt::Display;

use crate::scanner::token::Token;

use super::ast::{AstNode, ExprNode};

#[derive(Clone)]
pub struct IfStmt<'a> {
    pub condition: ExprNode<'a>,
    pub if_body: Vec<Stmt<'a>>,
    /// is an empty vector if there is no specified else
    pub else_body: Vec<Stmt<'a>>,
}

#[derive(Clone)]
pub struct WhileStmt<'a> {
    pub condition: ExprNode<'a>,
    pub body: Vec<Stmt<'a>>,
}

#[derive(Clone)]
pub struct ForStmt<'a> {
    /// optional initializer for the loop
    pub initializer: Option<Box<Stmt<'a>>>,
    /// optional condition for loop stoppage
    pub condition: Option<ExprNode<'a>>,
    /// optional incrementer
    pub increment: Option<ExprNode<'a>>,
    pub body: Vec<Stmt<'a>>,
}

#[derive(Clone)]
pub struct VarDeclStatement<'a> {
    pub var_name: Token<'a>,
    pub initializer: Option<ExprNode<'a>>,
}

#[derive(Clone)]
pub struct ReturnStmt<'a> {
    pub value: Option<ExprNode<'a>>,
}

#[derive(Clone)]
pub struct FuncDeclStatement<'a> {
    pub name: Token<'a>,
    pub parameters: Vec<Token<'a>>,
    pub body: Vec<Stmt<'a>>,
}

#[derive(Clone)]
pub struct ClassDeclStatement<'a> {
    pub name: Token<'a>,
    pub methods: Vec<FuncDeclStatement<'a>>,
}

#[derive(Clone)]
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

    /// Return statement, containing optional return expression
    Return(ReturnStmt<'a>),

    /// Function declaration containing:
    ///   - function name as token
    ///   - paramters as token list
    ///   - functio body as statement list
    FuncDecl(FuncDeclStatement<'a>),

    /// Class declarations containing:
    ///   - name as token
    ///   - methods as list of FuncDeclStatements
    ClassDecl(ClassDeclStatement<'a>),

    Error,
}

impl<'a> AstNode for Stmt<'a> {
    fn count_nodes(&self) -> usize {
        match self {
            Stmt::Error => 1,
            Stmt::Expression(expr) => expr.count_nodes(),
            Stmt::FuncDecl(func) => func.body.iter().map(|m| m.count_nodes()).sum(),
            Stmt::ClassDecl(class) => class
                .methods
                .iter()
                .map(|m| m.body.iter().map(|m| m.count_nodes()).sum::<usize>())
                .sum(),
            Stmt::VarDecl(var) => match &var.initializer {
                Some(i) => i.count_nodes(),
                None => 0,
            },
            Stmt::Return(ret) => match &ret.value {
                Some(r) => r.count_nodes(),
                None => 0,
            },
            Stmt::While(wh) => {
                let condition = wh.condition.count_nodes();
                let body = wh.body.iter().map(|m| m.count_nodes()).sum::<usize>();
                condition + body
            }
            Stmt::For(payload) => {
                let initializer = match &payload.initializer {
                    Some(init) => init.count_nodes(),
                    None => 0,
                };

                let condition = match &payload.condition {
                    Some(cond) => cond.count_nodes(),
                    None => 0,
                };

                let increment = match &payload.increment {
                    Some(inc) => inc.count_nodes(),
                    None => 0,
                };

                let body = payload.body.iter().map(|m| m.count_nodes()).sum::<usize>();

                initializer + condition + increment + body
            }
            Stmt::If(payload) => {
                let condition = payload.condition.count_nodes();
                let if_body = payload
                    .if_body
                    .iter()
                    .map(|m| m.count_nodes())
                    .sum::<usize>();
                let else_body = payload
                    .else_body
                    .iter()
                    .map(|m| m.count_nodes())
                    .sum::<usize>();
                condition + if_body + else_body
            }
        }
    }
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
                s += &format!("{}Var: {}", indent, data.var_name.lexeme.unwrap());
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

            Stmt::Return(ret) => {
                let mut s = format!("{}Return:\n", spaces);
                s += &format!(
                    "{}",
                    ret.value
                        .as_ref()
                        .map_or(format!("{}  None", indent), |node| node
                            .node
                            .to_yaml(next_level))
                );
                s.trim_end().to_string()
            }

            Stmt::FuncDecl(func) => {
                let mut s = format!("{}Func:\n", spaces);
                s += &format!("{}Name: {}", indent, func.name.lexeme.unwrap());
                if func.parameters.is_empty() {
                    s += &format!("\n{}Params: []", indent);
                } else {
                    s += &format!("\n{}Params: [ ", indent);
                    for (idx, param) in func.parameters.iter().enumerate() {
                        s += &format!("{}", param.lexeme.unwrap());
                        if idx + 1 < func.parameters.len() {
                            s += &format!(", ");
                        }
                    }
                    s += &format!(" ]");
                }
                if func.body.is_empty() {
                    s += &format!("\n{}Body: []", indent);
                } else {
                    s += &format!("\n{}Body: [", indent);
                    for stmt in func.body.iter() {
                        s += &format!("\n{},\n", stmt.to_yaml(next_level + 1).trim_end());
                    }
                    s += &format!("{}]", indent);
                }
                s.trim_end().to_string()
            }

            Stmt::ClassDecl(class) => {
                let mut s = format!("{}Class:\n", spaces);
                s += &format!("{}Name: {}", indent, class.name.lexeme.unwrap());
                if class.methods.is_empty() {
                    s += &format!("\n{}Methods: []", indent);
                } else {
                    s += &format!("\n{}Methods: [", indent);
                    for method in class.methods.iter() {
                        s += &format!(
                            "\n{},\n",
                            Stmt::FuncDecl(method.clone())
                                .to_yaml(next_level + 1)
                                .trim_end()
                        );
                    }
                    s += &format!("{}]", indent);
                }
                s.trim_end().to_string()
            }

            Stmt::Error => format!("{}ERROR", spaces),
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

        assert!(!parser.has_errors());
        assert!(statements.len() == 1);
        assert!(matches!(statements.get(0).unwrap(), Stmt::For(_)));
    }

    #[test]
    fn parse_var_decl() {
        let tokens = scan(
            "
            var myVar = 42 + 31 * 4;
            ",
        );
        let mut parser = Parser::new(tokens);
        let statements = parser.parse();

        assert!(!parser.has_errors());
        assert!(statements.len() == 1);
        assert!(matches!(statements.get(0).unwrap(), Stmt::VarDecl(_)));
    }

    #[test]
    fn parse_return() {
        let tokens = scan("return 42 + 1337;");
        let mut parser = Parser::new(tokens);
        let statements = parser.parse();

        assert!(!parser.has_errors());
        assert!(statements.len() == 1);
        assert!(matches!(statements.get(0).unwrap(), Stmt::Return(_)));
    }

    #[test]
    fn parse_empty_function_decl() {
        let tokens = scan("fun myFunc() {}");
        let mut parser = Parser::new(tokens);
        let statements = parser.parse();

        assert!(!parser.has_errors());
        assert!(statements.len() == 1);
        assert!(matches!(statements.get(0).unwrap(), Stmt::FuncDecl(_)));
    }

    #[test]
    fn parse_function_decl() {
        let tokens = scan("fun myFunc(a, b) { var myVar = a; return a + 42;}");
        let mut parser = Parser::new(tokens);
        let statements = parser.parse();

        assert!(!parser.has_errors());
        assert!(statements.len() == 1);
        assert!(matches!(statements.get(0).unwrap(), Stmt::FuncDecl(_)));
    }

    #[test]
    fn parse_class_decl() {
        let tokens =
            scan("class Nice {fun methodOne() {} fun methodTwo(name, age) { return name + age; }}");
        let mut parser = Parser::new(tokens);
        let statements = parser.parse();
        statements.iter().for_each(|f| println!("{}", f));

        assert!(!parser.has_errors());
        assert!(statements.len() == 1);
        assert!(matches!(statements.get(0).unwrap(), Stmt::ClassDecl(_)));
    }
}
