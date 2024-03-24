#![deny(unused_crate_dependencies)]

pub mod literal;
pub(crate) mod monotonic;
pub mod name;
pub mod source;

pub use crate::{
    literal::LiteralKind,
    name::{Identifier, Name, NameSegment},
    source::{CheapRange, SourceId, SourceMap, SourceSpan, TokenSpan},
};

#[cfg(test)]
mod tests {}
