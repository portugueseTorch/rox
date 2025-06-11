use anyhow::bail;

use crate::scanner::token::{Token, TokenType};

use super::ast::ExprNode;

#[derive(Clone)]
pub struct BinaryExpr<'a> {
    pub op: TokenType,
    pub left: Box<ExprNode<'a>>,
    pub right: Box<ExprNode<'a>>,
}

#[derive(Clone)]
pub struct UnaryExpr<'a> {
    pub op: TokenType,
    pub operand: Box<ExprNode<'a>>,
}

#[derive(Clone)]
pub struct AssignmentExpr<'a> {
    pub name: Token<'a>,
    pub expr: Box<ExprNode<'a>>,
}

#[derive(Clone)]
pub struct CallExpr<'a> {
    pub calee: Box<ExprNode<'a>>,
    pub args: Vec<ExprNode<'a>>,
}

#[derive(Clone)]
pub struct PropertyAccessExpr<'a> {
    pub object: Box<ExprNode<'a>>,
    pub property: Token<'a>,
}

// --- may be subject to constant folding
#[derive(Clone)]
pub enum Value {
    StringLiteral(String),
    Number(i32),
    Bool(bool),
    Nil,
}

#[derive(Clone)]
pub enum Expr<'a> {
    // --- expressions
    /// Literals, containing
    ///   - string literals as a slice into the source code
    ///   - number as an i32
    ///   - booleans
    ///   - nil
    ///   ```
    /// //   "Hello, World!"
    /// //  1337
    /// //  true
    /// //  nil
    ///   ```
    Constant(Value),

    /// Variable identifier, containing the name of the identifier as a slice into the source code
    /// ```
    /// // myVar
    /// ```
    Var(&'a str),

    /// Binary operations
    /// ```
    /// // a + b * 42
    /// // --- Resulting tree:
    ///
    /// //        +
    /// //       / \
    /// //      a   *
    /// //         / \
    /// //        b  42
    /// ```
    BinOp(BinaryExpr<'a>),

    /// Unary operation:
    ///   - first element of the typle holds the token for the unary operator
    ///   - second element of the tuple is the operand
    ///  ```
    ///  // -1337
    ///  // !boolean
    ///  ```
    Unary(UnaryExpr<'a>),

    /// Asssignment operation:
    ///   - first element of the tuple holds the token for the name of the variable
    ///   - second element of the tuple holds the node to be assigned
    /// ```
    /// // var myVar = a + b * 42;
    /// ```
    Assignment(AssignmentExpr<'a>),

    /// Grouping around an expression
    /// ```
    /// // (a + b)
    /// ```
    Grouping(Box<ExprNode<'a>>),

    /// Call expression:
    ///   - first element of the tuple holds the node for the calle
    ///   - second element of the tuple holds a vector of args
    /// ```
    /// // method(42)
    /// ```
    Call(CallExpr<'a>),

    /// Property access expression:
    ///   - first element of the tuple holds the node for the calle
    ///   - second element of the tuple holds a vector of args
    /// ```
    /// // obj.property
    /// ```
    PropertyAccess(PropertyAccessExpr<'a>),

    /// Represents an error
    Error,
}

impl<'a> Expr<'a> {
    pub fn fold_constants(c1: Value, c2: Value, op: TokenType) -> Expr<'a> {
        let computed_value = Value::compute(c1, c2, op);

        match computed_value {
            Ok(v) => Expr::Constant(v),
            Err(_) => Expr::Error,
        }
    }

    pub fn is_error(&self) -> bool {
        if matches!(self, Expr::Error) {
            return true;
        }

        false
    }

    pub fn to_yaml(&self, level: usize) -> String {
        let spaces = " ".repeat(level * 2);
        let next_level = level + 1;
        let indent = " ".repeat(next_level * 2);

        match self {
            Expr::Error => format!("{}ERROR", spaces),

            Expr::Var(var) => format!("{}Var: {}", spaces, var),

            Expr::Call(call) => {
                let mut s = format!("{}Call:\n", spaces);
                s += &format!(
                    "{}Calee:\n{}",
                    indent,
                    call.calee.node.to_yaml(next_level + 1)
                );
                if call.args.is_empty() {
                    s += &format!("\n{}Args: []", indent);
                } else {
                    s += &format!("\n{}Args: [", indent);
                    for arg in call.args.iter() {
                        s += &format!("\n{}", arg.node.to_yaml(next_level + 1).trim_end());
                    }
                    s += &format!("\n{}]", indent);
                }
                s.trim_end().to_string()
            }

            Expr::Constant(val) => {
                let val_as_string = match val {
                    Value::StringLiteral(l) => format!("{}", l),
                    Value::Nil => "Nil".to_string(),
                    Value::Bool(b) => format!("{}", b),
                    Value::Number(n) => format!("{}", n),
                };
                format!("{}Constant: {}", spaces, val_as_string)
            }

            Expr::Unary(unary) => {
                let mut s = format!("{}Unary:\n", spaces);
                s += &format!("{}Op: '{}'\n", indent, unary.op);
                s += &format!(
                    "{}Expr:\n{}",
                    indent,
                    unary.operand.node.to_yaml(next_level + 1)
                );
                s
            }

            Expr::Grouping(expr) => {
                format!("{}Group:\n{}", spaces, expr.node.to_yaml(next_level))
            }

            Expr::Assignment(a) => {
                let mut s = format!("{}Assignment:\n", spaces);
                s += &format!(
                    "{}Name: {}\n",
                    indent,
                    a.name.lexeme.as_deref().unwrap_or("")
                );
                s += &format!("{}Val:\n{}", indent, a.expr.node.to_yaml(next_level + 1));
                s
            }

            Expr::BinOp(bin) => {
                let mut s = format!("{}BinOp:\n", spaces);
                s += &format!("{}Op: '{}'", indent, bin.op);
                s += &format!(
                    "\n{}Lhs:\n{}",
                    indent,
                    bin.left.node.to_yaml(next_level + 1)
                );
                s += &format!(
                    "\n{}Rhs:\n{}",
                    indent,
                    bin.right.node.to_yaml(next_level + 1)
                );
                s
            }

            Expr::PropertyAccess(prop) => {
                let mut s = format!("{}PropAccess:\n", spaces);
                s += &format!(
                    "{}Obj:\n{}",
                    indent,
                    prop.object.node.to_yaml(next_level + 1)
                );
                s += &format!("\n{}Prop: {}", indent, prop.property.lexeme.unwrap());
                s
            }
        }
    }
}

impl<'a> std::fmt::Display for Expr<'a> {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "\n{}\n", self.to_yaml(0))
    }
}

impl Value {
    pub fn compute(lhs: Value, rhs: Value, op: TokenType) -> anyhow::Result<Value> {
        match op {
            TokenType::Plus => match (lhs, rhs) {
                (Value::Number(l), Value::Number(r)) => Ok(Value::Number(l + r)),
                (Value::StringLiteral(l), Value::StringLiteral(r)) => {
                    Ok(Value::StringLiteral(format!("{}{}", l, r)))
                }
                _ => bail!("invalid op for numbers"),
            },
            TokenType::Minus => match (lhs, rhs) {
                (Value::Number(l), Value::Number(r)) => Ok(Value::Number(l - r)),
                _ => bail!("invalid op for numbers"),
            },
            TokenType::Star => match (lhs, rhs) {
                (Value::Number(l), Value::Number(r)) => Ok(Value::Number(l * r)),
                _ => bail!("invalid op for numbers"),
            },
            TokenType::Slash => match (lhs, rhs) {
                (Value::Number(l), Value::Number(r)) => Ok(Value::Number(l / r)),
                _ => bail!("invalid op for numbers"),
            },
            TokenType::EqualEqual => match (lhs, rhs) {
                (Value::Number(l), Value::Number(r)) => Ok(Value::Bool(l == r)),
                (Value::Bool(l), Value::Bool(r)) => Ok(Value::Bool(l == r)),
                (Value::StringLiteral(l), Value::StringLiteral(r)) => Ok(Value::Bool(l == r)),
                _ => bail!("invalid op for numbers"),
            },
            TokenType::GreaterEqual => match (lhs, rhs) {
                (Value::Number(l), Value::Number(r)) => Ok(Value::Bool(l >= r)),
                _ => bail!("invalid op"),
            },
            TokenType::LessEqual => match (lhs, rhs) {
                (Value::Number(l), Value::Number(r)) => Ok(Value::Bool(l <= r)),
                _ => bail!("invalid op"),
            },
            _ => unreachable!(),
        }
    }
}

#[cfg(test)]
mod tests {
    use crate::{
        parser::{expressions::Expr, parser::Parser},
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
    fn parse_number() {
        let tokens = scan("42;");
        assert_eq!(tokens.len(), 3, "Should have 3 tokens");
        let mut it = tokens.iter();
        assert_eq!(it.next().unwrap().token_type, TokenType::Number);
        assert_eq!(it.next().unwrap().token_type, TokenType::Semicolon);
        assert_eq!(it.next().unwrap().token_type, TokenType::EOF);

        let mut parser = Parser::new(tokens);
        let node = parser.parse_expression(true);

        assert_eq!(parser.has_errors(), false, "Should not have parsing errors");
        assert!(matches!(node.node, Expr::Constant(_)));
    }

    #[test]
    fn parse_identifier() {
        let tokens = scan("myVar;");
        let mut parser = Parser::new(tokens);
        let node = parser.parse_expression(true);

        assert_eq!(parser.has_errors(), false, "Should not have parsing errors");
        assert!(matches!(node.node, Expr::Var(_)));
    }

    #[test]
    fn parse_binop() {
        let tokens = scan("2 + 3;");
        let mut parser = Parser::new(tokens);
        let node = parser.parse_expression(true);

        assert_eq!(parser.has_errors(), false, "Should not have parsing errors");
        assert!(matches!(node.node, Expr::BinOp(_)));
    }

    #[test]
    fn parse_complex_binop() {
        let tokens = scan("2 + 3 * 4 + 5 * 6;");
        let mut parser = Parser::new(tokens);
        let node = parser.parse_expression(true);

        assert_eq!(parser.has_errors(), false, "Should not have parsing errors");
        assert!(matches!(node.node, Expr::BinOp(_)));
    }

    #[test]
    fn parse_incorrect_binop() {
        let tokens = scan("3 +");
        let mut parser = Parser::new(tokens);
        let node = parser.parse_expression(true);

        assert_eq!(
            parser.has_errors(),
            true,
            "3 + 2 - is not a valid expression"
        );

        match &node.node {
            Expr::BinOp(bin) => {
                assert!(matches!(bin.right.node, Expr::Error))
            }
            _ => panic!("Should be binop"),
        }
    }

    #[test]
    fn parse_with_group() {
        let tokens = scan("(3 + 2) * 10;");
        let mut parser = Parser::new(tokens);
        let node = parser.parse_expression(true);

        assert!(!parser.has_errors());
        match &node.node {
            Expr::BinOp(bin) => {
                assert!(matches!(bin.left.node, Expr::Grouping(_)));
                assert!(matches!(bin.right.node, Expr::Constant(_)));
            }
            _ => panic!("Should be binop"),
        }
    }

    #[test]
    fn parse_simple_unary() {
        let tokens = scan("-42;");
        let mut parser = Parser::new(tokens);
        let node = parser.parse_expression(true);

        assert!(!parser.has_errors());
        match &node.node {
            Expr::Unary(unary) => {
                assert!(matches!(unary.op, TokenType::Minus));
                assert!(matches!(unary.operand.node, Expr::Constant(_)));
            }
            _ => panic!("Should be unary"),
        }
    }

    #[test]
    fn parse_multi_unary() {
        let tokens = scan("--42;");
        let mut parser = Parser::new(tokens);
        let node = parser.parse_expression(true);

        assert!(!parser.has_errors());
        match &node.node {
            Expr::Unary(unary) => {
                assert!(matches!(unary.op, TokenType::Minus));
                assert!(matches!(unary.operand.node, Expr::Unary(_)));
            }
            _ => panic!("Should be unary"),
        }
    }

    #[test]
    fn parse_grouped_unary() {
        let tokens = scan("-(42 + 10);");
        let mut parser = Parser::new(tokens);
        let node = parser.parse_expression(true);

        assert!(!parser.has_errors());
        match &node.node {
            Expr::Unary(unary) => {
                assert!(matches!(unary.op, TokenType::Minus));
                assert!(matches!(unary.operand.node, Expr::Grouping(_)));
            }
            _ => panic!("Should be unary"),
        }
    }

    #[test]
    fn parse_complex() {
        let tokens = scan("-(42 + 10) + 27 / (10 + (b * myVar));");
        let parser = Parser::new(tokens);

        assert!(!parser.has_errors());
    }

    #[test]
    fn parse_assignment_to_expression() {
        let tokens = scan("myVar = -(42 + 10);");
        let mut parser = Parser::new(tokens);
        let node = parser.parse_expression(true);

        assert!(!parser.has_errors());
        assert!(matches!(node.node, Expr::Assignment(_)));
    }

    #[test]
    fn parse_logical_expression() {
        let tokens = scan("true or false and 42;");
        let mut parser = Parser::new(tokens);
        let node = parser.parse_expression(true);

        assert!(!parser.has_errors());
        assert!(matches!(node.node, Expr::BinOp(_)));
    }

    #[test]
    fn parse_equality_expression() {
        let tokens = scan("32 == 27;");
        let mut parser = Parser::new(tokens);
        let node = parser.parse_expression(true);

        assert!(!parser.has_errors());
        assert!(matches!(node.node, Expr::BinOp(_)));
    }

    #[test]
    fn parse_equality_expression_2() {
        let tokens = scan("32 != 27;");
        let mut parser = Parser::new(tokens);
        let node = parser.parse_expression(true);

        assert!(!parser.has_errors());
        assert!(matches!(node.node, Expr::BinOp(_)));
    }

    #[test]
    fn parse_comparison_expression() {
        let tokens = scan("32 >= 27 and 10 < 11 or 9 <= 6 and 8 > 2;");
        let mut parser = Parser::new(tokens);
        let node = parser.parse_expression(true);

        assert!(!parser.has_errors());
        assert!(matches!(node.node, Expr::BinOp(_)));
    }

    #[test]
    fn parse_property_access() {
        let tokens = scan("user.data.email;");
        let mut parser = Parser::new(tokens);
        let node = parser.parse_expression(true);
        node.log();

        assert!(!parser.has_errors());
        assert!(matches!(node.node, Expr::PropertyAccess(_)));
    }

    #[test]
    fn parse_call_expression() {
        let tokens = scan("obj.myFunc(42, hello);");
        let mut parser = Parser::new(tokens);
        let node = parser.parse_expression(true);

        assert!(!parser.has_errors());
        assert!(matches!(node.node, Expr::Call(_)));
    }

    #[test]
    fn parse_call_expression_multiple_args() {
        let tokens = scan("myFunc(42, hello + 3);");
        let mut parser = Parser::new(tokens);
        let node = parser.parse_expression(true);

        assert!(!parser.has_errors());
        assert!(matches!(node.node, Expr::Call(_)));
    }

    #[test]
    fn parse_call_prop_access() {
        let tokens = scan("obj.methodOne(42).methodTwo(hello, goodbye)();");
        let mut parser = Parser::new(tokens);
        let node = parser.parse_expression(true);
        node.log();

        assert!(!parser.has_errors());
        // assert!(matches!(node.node, NodeType::Call(_)));
    }
}
