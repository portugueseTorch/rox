use std::fmt::{Display, Pointer};

use crate::scanner::token::Token;

pub struct BinaryOperation<'a> {
    pub op: Token<'a>,
    pub left: Box<Node<'a>>,
    pub right: Box<Node<'a>>,
}

// --- may be subject to constant folding
pub enum Value<'a> {
    StringLiteral(&'a str),
    Number(i32),
    Bool(bool),
    Nil,
}

pub enum Node<'a> {
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
    Literal(Value<'a>),

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
    BinOp(BinaryOperation<'a>),

    /// Unary operation:
    ///   - first element of the typle holds the token for the unary operator
    ///   - second element of the tuple is the operand
    ///  ```
    ///  -1337
    ///  !boolean
    ///  ```
    Unary(Token<'a>, Box<Node<'a>>),

    /// Asssignment operation:
    ///   - first element of the tuple holds the token for the name of the variable
    ///   - second element of the tuple holds the node to be assigned
    /// ```
    /// var myVar = a + b * 42;
    /// ```
    Assignment(Token<'a>, Box<Node<'a>>),

    /// Grouping around an expression
    /// ```
    /// (a + b)
    /// ```
    Grouping(Box<Node<'a>>),

    /// Call expression:
    ///   - first element of the tuple holds the node for the calle
    ///   - second element of the tuple holds a vector of args
    /// ```
    /// obj.funcs.method(42)
    /// ```
    Call(Box<Node<'a>>, Vec<Node<'a>>),

    /// Represents an error
    Error,
}

impl<'a> Node<'a> {
    pub fn is_error(&self) -> bool {
        if matches!(self, Node::Error) {
            return true;
        }

        false
    }

    pub fn format(&self) -> String {
        match self {
            Node::Error => format!("ERROR"),
            Node::Var(var) => format!("VAR ({})", var),
            Node::Call(calee, args) => {
                format!("CALL:\n  calee: {}\n  args: {}", calee.format(), args.len())
            }
            Node::Literal(val) => {
                let val_as_string = match val {
                    Value::StringLiteral(l) => format!("String({})", l),
                    Value::Nil => "Nil".to_string(),
                    Value::Bool(b) => format!("Boolean({})", b),
                    Value::Number(n) => format!("Number({})", n),
                };
                format!("LITERAL: {}", val_as_string)
            }
            Node::Unary(op, expr) => {
                format!("UNARY:\n  Op: {}\n  Expr: {}", op.token_type, expr.format())
            }
            Node::Grouping(expr) => {
                format!("GROUP: {}", expr.format())
            }
            Node::Assignment(name, val) => {
                format!(
                    "ASSIGNMENT:\n  name: {}\n  val: {}",
                    name.lexeme.unwrap(),
                    val.format()
                )
            }
            Node::BinOp(bin) => {
                format!(
                    "BINOP:\n  op: {}\n  lhs: {}\n  rhs: {}",
                    bin.op.token_type,
                    bin.left.format(),
                    bin.right.format()
                )
            }
        }
    }
}

impl<'a> Display for Node<'a> {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", self.format())
    }
}
