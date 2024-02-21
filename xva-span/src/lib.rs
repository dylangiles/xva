#![deny(unused_crate_dependencies)]

pub(crate) mod monotonic;
pub mod source;

pub use source::{CheapRange, SourceId, SourceMap, SourceSpan, TokenSpan};

#[cfg(test)]
mod tests {}
