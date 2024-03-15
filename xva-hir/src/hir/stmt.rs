use internment::Intern;
use xva_ast::ast::Type;
use xva_span::SourceSpan;

use super::expr::Expression;
use crate::id::HirId;

#[derive(Debug)]
pub struct Statement {
    pub id: HirId,
    pub span: SourceSpan,
    pub kind: StatementKind,
}

#[derive(Debug)]
pub enum StatementKind {
    Local(Local),
}

#[derive(Debug)]
pub struct Local {
    pub id: HirId,
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

#[derive(Debug, Clone)]
pub struct Identifier {
    pub name: Intern<String>,
    pub span: SourceSpan,
}
