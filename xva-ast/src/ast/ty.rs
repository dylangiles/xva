use xva_span::{Name, SourceSpan};

use crate::node_id::NodeId;

/// A type in the abstract syntax tree.
#[derive(Debug, Clone)]
pub struct TypeAnno {
    pub id: NodeId,
    pub kind: TypeKind,
    pub span: SourceSpan,
}

#[derive(Debug, Clone)]
pub enum TypeKind {
    /// The unit type: `unit`
    Unit,

    /// The never type: `never`
    // Never,

    /// The placeholder type for inference: `_`.
    /// Note that this is a separate but related concept to **not** providing a type annotation - both
    /// cases will trigger type inference.
    Infer,

    /// The type referred to by the contained [`Name`]
    Named(Name),
}
