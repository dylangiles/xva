use xva_span::{Name, SourceSpan};

use crate::hir::HirId;

// use

/// A type annotation in the high intermediate representation.
#[derive(Debug, Clone)]
pub struct TypeAnno {
    pub id: HirId,
    pub kind: TypeKind,
    pub span: SourceSpan,
}

#[derive(Debug, Clone)]
pub enum TypeKind {
    /// The unit type: `unit`
    Unit,

    // /// The never type: `never`
    // Never,
    /// The placeholder type for inference: `_`.
    /// Note that this is a separate but related concept to **not** providing a type annotation - both
    /// cases will trigger type inference.
    Infer,

    /// The type referred to by the contained [`Name`]
    Named(Name),
}
