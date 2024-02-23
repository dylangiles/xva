use xva_ast::ast::LiteralKind;
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
pub enum ExpressionKind<'hir> {
    Binary(BinaryOp, &'hir Expression<'hir>, &'hir Expression<'hir>),
    Unary(UnaryOp, &'hir Expression<'hir>),
    Literal(LiteralKind),
}

#[derive(Debug)]
pub struct Expression<'hir> {
    id: HirId,
    span: SourceSpan,
    kind: ExpressionKind<'hir>,
}
