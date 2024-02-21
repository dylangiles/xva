use xva_span::SourceSpan;

use super::{Expression, Identifier, Type};
use crate::node_id::NodeId;

#[derive(Debug)]
pub struct Statement {
    pub id: NodeId,
    pub kind: StatementKind,
    pub span: SourceSpan,
}

#[derive(Debug)]
pub enum StatementKind {
    Local(Local),
}

/// Represents a binding of a name, i.e. a variable declaration
///
/// For example: `let x = 5` or `var x: bool = false`
#[derive(Debug)]
pub struct Local {
    pub id: NodeId,
    pub span: SourceSpan,
    pub binding_kind: BindingKind,
    pub binding_flags: BindingFlags,
    pub pattern: BindingPattern,

    /// The type annotation. If `None`, no type annotation was provided.
    ///
    /// Note that `None` is a distinct but related concept to [`TypeKind::Infer`]
    pub ty: Option<Type>,
}

#[derive(Debug)]
pub enum BindingKind {
    Declared,
    Inited(Box<Expression>),
}

#[derive(Debug)]
pub enum BindingPattern {
    Identifier(Identifier),
}

/// "Settings" for a declared local.
#[derive(Debug, Clone, Copy)]
pub struct BindingFlags {
    /// The local was declared as mutable, i.e. the `var` keyword.
    pub mutable: bool,
}
