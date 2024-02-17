use std::ops::Range;
use xva_span::SourceSpan;

use crate::has_node_id;
use crate::node_id::{NodeId, ERROR_NODE_ID};
use crate::traits::HasNodeId;

#[derive(Debug)]
pub struct Brick {
    pub items: Vec<Item>,
}

impl Brick {
    pub fn new_from_items(items: Vec<Item>) -> Self {
        Self { items }
    }
}

#[derive(Debug)]
pub struct Item {
    pub id: NodeId,
    pub kind: ItemKind,

    /// The item's start (inclusive) and end (exclusive) range, in byte offsets
    /// from the source text.
    pub span: SourceSpan,
}

impl Item {
    pub const fn error(span: chumsky::span::SimpleSpan<usize>, err: String) -> Self {
        Self {
            id: ERROR_NODE_ID,
            kind: ItemKind::Error(err),
            span,
        }
    }
}
#[derive(Debug)]
pub enum ItemKind {
    Expression(Expression),
    Module(Module),
    Error(String),
}

#[derive(Debug)]
pub struct Module {
    pub items: Vec<Item>,
}

#[derive(Debug)]
pub struct Expression {
    pub id: NodeId,
    pub kind: ExpressionKind,
    pub span: SourceSpan,
}

#[derive(Debug)]
pub enum ExpressionKind {
    Literal(LiteralKind),
    Identifier(String),
    Unary(UnaryOperator, Box<Expression>),
    Binary(BinaryOperator, Box<Expression>, Box<Expression>),
}

#[derive(Debug)]
pub enum LiteralKind {
    Integer(i128),
    Boolean(bool),
    Char(char),
    Float(f64),
    String(String), // TODO PLEASE change this to interned strings bruh
}

#[derive(Debug)]
pub enum UnaryOperator {
    Negation,
    Not,
}

#[derive(Debug)]
pub enum BinaryOperator {
    LogicalAnd,
    LogicalOr,
    BitwiseAnd,
    BitwiseOr,
    BitwiseXor,
    Add,
    Subtract,
    Multiply,
    Divide,
    Modulo,
    Power,
    LeftShift,
    RightShift,
    LessThan,
    GreaterThan,
    Equal,
    NotEqual,
    GreaterThanEqual,
    LessThanEqual,
}

#[derive(Debug)]
pub enum LiteralIntegerKind {
    Signed,
    Unsigned,
}

has_node_id!(Item, Expression);
