use crate::scanner::token::Token;

struct BinaryOperation<'a> {
    op: Token<'a>,
    left: Box<Node<'a>>,
    right: Box<Node<'a>>,
}

// --- may be subject to constant folding
enum Value<'a> {
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
}
