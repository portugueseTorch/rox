use std::fmt::Display;

use itertools::Itertools;

use crate::scanner::token::Token;

use super::expressions::{AssignmentExpr, CallExpr, Expr, PropertyAccessExpr, UnaryExpr};

pub trait AstNode {
    fn count_nodes(&self) -> usize;
    fn optimize(&self) -> Self;
}

#[derive(Clone)]
pub struct ExprNode<'a> {
    pub token: Token<'a>,
    pub node: Expr<'a>,
}

impl<'a> ExprNode<'a> {
    pub fn new(token: Token<'a>, node: Expr<'a>) -> Self {
        Self { token, node }
    }

    pub fn log(&self) {
        println!("{}", self.node);
    }
}

impl<'a> Display for ExprNode<'a> {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", self.node)
    }
}

impl<'a> AstNode for ExprNode<'a> {
    fn count_nodes(&self) -> usize {
        let nodes_in_subtrees = match &self.node {
            Expr::Error | Expr::Var(_) | Expr::Constant(_) => 0,
            Expr::Assignment(assignment) => assignment.expr.count_nodes(),
            Expr::Unary(unary) => unary.operand.count_nodes(),
            Expr::Grouping(group) => group.count_nodes(),
            Expr::PropertyAccess(prop) => prop.object.count_nodes(),
            Expr::BinOp(binop) => {
                let left = binop.left.count_nodes();
                let right = binop.right.count_nodes();
                left + right
            }
            Expr::Call(call) => {
                let calee_nodes = call.calee.count_nodes();
                let arg_nodes = call.args.iter().map(|m| m.count_nodes()).sum::<usize>();
                calee_nodes + arg_nodes
            }
        };

        nodes_in_subtrees + 1
    }

    fn optimize(&self) -> Self {
        let expr = match &self.node {
            Expr::BinOp(binop) => {
                let optimized_left = binop.left.optimize();
                let optimized_right = binop.right.optimize();

                // --- if both the subtrees evaluated to constants, fold them
                match (optimized_left.node, optimized_right.node) {
                    (Expr::Constant(c1), Expr::Constant(c2)) => {
                        Expr::fold_constants(c1, c2, binop.op)
                    }
                    _ => self.node.clone(),
                }
            }
            Expr::Unary(unary) => {
                let optimized_operand = unary.operand.optimize();

                Expr::Unary(UnaryExpr {
                    op: unary.op,
                    operand: Box::new(optimized_operand),
                })
            }
            Expr::Assignment(assignment) => {
                let optimized_expr = assignment.expr.optimize();

                Expr::Assignment(AssignmentExpr {
                    name: assignment.name.clone(),
                    expr: Box::new(optimized_expr),
                })
            }
            Expr::Call(call) => {
                let optimized_args = call.args.iter().map(ExprNode::optimize).collect_vec();
                let optimized_calee = call.calee.optimize();

                Expr::Call(CallExpr {
                    calee: Box::new(optimized_calee),
                    args: optimized_args,
                })
            }
            Expr::PropertyAccess(prop) => {
                let optimized_object = prop.object.optimize();

                Expr::PropertyAccess(PropertyAccessExpr {
                    object: Box::new(optimized_object),
                    property: prop.property.clone(),
                })
            }
            Expr::Grouping(group) => {
                let optimized = group.optimize();

                match optimized.node {
                    Expr::Constant(val) => Expr::Constant(val),
                    _ => Expr::Grouping(Box::new(optimized)),
                }
            }
            Expr::Error | Expr::Var(_) | Expr::Constant(_) => self.node.clone(),
        };

        Self {
            node: expr,
            token: self.token.clone(),
        }
    }
}
