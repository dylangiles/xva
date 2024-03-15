use xva_span::SourceSpan;

use super::stmt::Statement;
use crate::id::HirId;

#[derive(Debug)]
pub struct Item {
    pub id: HirId,
    pub kind: ItemKind,

    /// The item's start (inclusive) and end (exclusive) range, in byte offsets
    /// from the source text.
    pub span: SourceSpan,
}

#[derive(Debug)]
pub enum ItemKind {
    Statement(Statement),
}
