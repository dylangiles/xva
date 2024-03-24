use xva_span::{LiteralKind, SourceSpan};

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

#[derive(Debug, Clone)]
pub struct Expression {
    pub id: HirId,
    pub span: SourceSpan,
    pub kind: ExpressionKind,
}

#[derive(Debug, Clone)]
pub enum ExpressionKind {
    Literal(LiteralKind),
}
