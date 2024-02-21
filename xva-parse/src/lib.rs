#![deny(unused_crate_dependencies)]

pub mod error;

pub(crate) mod lexer;
pub(crate) mod macros;
pub mod parser;
pub mod token;
pub(crate) mod traits;
pub(crate) mod utils;

pub use error::SyntaxError;

#[cfg(test)]
mod tests {}
