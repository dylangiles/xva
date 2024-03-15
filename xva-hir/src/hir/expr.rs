use xva_ast::ast::{BindingFlags, BindingKind, BindingPattern, LiteralKind, Type};
use xva_span::SourceSpan;

use crate::id::HirId;

#[derive(Debug, Clone, Copy)]
pub enum BinaryOp {
    Add,
    Subtract,
    Multiply,
    Divide,
    Modulo,
    Power,
}

#[derive(Debug, Copy, Clone)]
pub enum UnaryOp {
    Not,
    Negate,
}

#[derive(Debug)]
pub struct Expression {
    pub id: HirId,
    pub span: SourceSpan,
    pub kind: ExpressionKind,
}

#[derive(Debug)]
pub enum ExpressionKind {
    Literal(LiteralKind),
}
