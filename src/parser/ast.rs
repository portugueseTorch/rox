use std::fmt::Display;

use crate::scanner::token::{Token, TokenType};

pub struct BinaryExpr<'a> {
    pub op: TokenType,
    pub left: Box<Node<'a>>,
    pub right: Box<Node<'a>>,
}

pub struct UnaryExpr<'a> {
    pub op: TokenType,
    pub operand: Box<Node<'a>>,
}

pub struct AssignmentExpr<'a> {
    pub name: Token<'a>,
    pub expr: Box<Node<'a>>,
}

pub struct CallExpr<'a> {
    pub calee: Box<Node<'a>>,
    pub args: Vec<Node<'a>>,
}

pub struct PropertyAccessExpr<'a> {
    pub object: Box<Node<'a>>,
    pub property: Token<'a>,
}

// --- may be subject to constant folding
pub enum Value<'a> {
    StringLiteral(&'a str),
    Number(i32),
    Bool(bool),
    Nil,
}

pub struct Node<'a> {
    pub token: Token<'a>,
    pub node: NodeType<'a>,
}

impl<'a> Node<'a> {
    pub fn new(token: Token<'a>, node: NodeType<'a>) -> Self {
        Self { token, node }
    }

    pub fn log(&self) {
        println!("{}", self.node);
    }
}

pub enum NodeType<'a> {
    // --- expressions
    /// Literals, containing
    ///   - string literals as a slice into the source code
    ///   - number as an i32
    ///   - booleans
    ///   - nil
    ///   ```
    ///   "Hello, World!"
    ///   1337
    ///   true
    ///   nil
    ///   ```
    Constant(Value<'a>),

    /// Variable identifier, containing the name of the identifier as a slice into the source code
    /// ```
    /// myVar
    /// ```
    Var(&'a str),

    /// Binary operations
    /// ```
    /// a + b * 42
    /// // --- Resulting tree:
    ///
    ///           +
    ///          / \
    ///         a   *
    ///            / \
    ///           b  42
    /// ```
    BinOp(BinaryExpr<'a>),

    /// Unary operation:
    ///   - first element of the typle holds the token for the unary operator
    ///   - second element of the tuple is the operand
    ///  ```
    ///  -1337
    ///  !boolean
    ///  ```
    Unary(UnaryExpr<'a>),

    /// Asssignment operation:
    ///   - first element of the tuple holds the token for the name of the variable
    ///   - second element of the tuple holds the node to be assigned
    /// ```
    /// var myVar = a + b * 42;
    /// ```
    Assignment(AssignmentExpr<'a>),

    /// Grouping around an expression
    /// ```
    /// (a + b)
    /// ```
    Grouping(Box<Node<'a>>),

    /// Call expression:
    ///   - first element of the tuple holds the node for the calle
    ///   - second element of the tuple holds a vector of args
    /// ```
    /// method(42)
    /// ```
    Call(CallExpr<'a>),

    /// Property access expression:
    ///   - first element of the tuple holds the node for the calle
    ///   - second element of the tuple holds a vector of args
    /// ```
    /// obj.property
    /// ```
    PropertyAccess(PropertyAccessExpr<'a>),

    /// Represents an error
    Error,
}

impl<'a> NodeType<'a> {
    pub fn is_error(&self) -> bool {
        if matches!(self, NodeType::Error) {
            return true;
        }

        false
    }

    fn to_yaml(&self, level: usize) -> String {
        let spaces = " ".repeat(level * 2);
        let next_level = level + 1;
        let indent = " ".repeat(next_level * 2);

        match self {
            NodeType::Error => format!("{}ERROR", spaces),

            NodeType::Var(var) => format!("{}Var: {}", spaces, var),

            NodeType::Call(call) => {
                let mut s = format!("{}Call:\n", spaces);
                s += &format!(
                    "{}Calee:\n{}",
                    indent,
                    call.calee.node.to_yaml(next_level + 1)
                );
                s += &format!("{}Args:\n", indent);
                for arg in call.args.iter() {
                    s += &format!(
                        "{}- {}\n",
                        " ".repeat((next_level + 1) * 2),
                        arg.node.to_yaml(next_level + 2).trim_end()
                    );
                }
                s.trim_end().to_string()
            }

            NodeType::Constant(val) => {
                let val_as_string = match val {
                    Value::StringLiteral(l) => format!("{}", l),
                    Value::Nil => "Nil".to_string(),
                    Value::Bool(b) => format!("{}", b),
                    Value::Number(n) => format!("{}", n),
                };
                format!("{}Constant: {}", spaces, val_as_string)
            }

            NodeType::Unary(unary) => {
                let mut s = format!("{}Unary:\n", spaces);
                s += &format!("{}Op: '{}'\n", indent, unary.op);
                s += &format!(
                    "{}Expr:\n{}",
                    indent,
                    unary.operand.node.to_yaml(next_level + 1)
                );
                s
            }

            NodeType::Grouping(expr) => {
                format!("{}Group:\n{}", spaces, expr.node.to_yaml(next_level))
            }

            NodeType::Assignment(a) => {
                let mut s = format!("{}Assignment:\n", spaces);
                s += &format!(
                    "{}Name: {}\n",
                    indent,
                    a.name.lexeme.as_deref().unwrap_or("")
                );
                s += &format!("{}Val:\n{}", indent, a.expr.node.to_yaml(next_level + 1));
                s
            }

            NodeType::BinOp(bin) => {
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

            NodeType::PropertyAccess(prop) => {
                let mut s = format!("{}PropAccess:\n", spaces);
                s += &format!(
                    "{}Obj: '{}'\n",
                    indent,
                    prop.object.node.to_yaml(next_level + 1)
                );
                s += &format!("{}Prop:\n{}", indent, prop.property.lexeme.unwrap());
                s
            }
        }
    }
}

impl<'a> Display for NodeType<'a> {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "\n{}\n", self.to_yaml(0))
    }
}
