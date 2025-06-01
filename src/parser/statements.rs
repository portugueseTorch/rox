use super::ast::Expr;

pub struct IfStatement<'a> {
    condition: Box<Expr<'a>>,
}

#[cfg(test)]
mod tests {
    use crate::{
        parser::{ast::ExprType, parser::Parser},
        scanner::{
            scanner::Scanner,
            token::{Token, TokenType},
        },
    };
}
