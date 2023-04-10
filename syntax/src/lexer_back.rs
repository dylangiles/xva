use chumsky::{
    prelude::Simple,
    primitive::{choice, end},
    recovery::skip_then_retry_until,
    Parser,
};
use std::ops::Range;

mod keyword;
mod literal;
mod operator;

/// A representation of a location within written code,
/// with a start and end.
///
/// This type is an alias of [`std::ops::Range`].
pub type Span = Range<usize>;

/// Contains definitions for all possible tokens that can be produced by the lexer.
#[derive(Debug, PartialEq)]
pub enum TokenType {
    Boolean(bool),
    Integer(i64),
    Float(f64),
    String(String),

    /*  All operators could inherit from an Operator class or we could avoid multiple inhertince by dividing parent operator
        classes into Arithmetic, Assignment, Comparison etc..
    */
    Operator(String),

    LetKeyword,
    DefKeyword,
    EndKeyword,

    // Arithmetic operators
    AdditionOperator,
    SubtractionOperator,
    MultiplicationOperator,
    DivisionOperator,
    ModuloOperator,
    ExponentiationOperator,
}

impl std::fmt::Display for TokenType {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            TokenType::Boolean(b) => write!(f, "{}", b),
            TokenType::Integer(i) => write!(f, "{}", i),
            TokenType::Float(fl) => write!(f, "{}", fl),
            TokenType::String(s) => write!(f, "{}", s),
            TokenType::Operator(o) => write!(f, "{}", o),
            TokenType::LetKeyword => write!(f, "let"),
            TokenType::DefKeyword => write!(f, "def"),
            TokenType::EndKeyword => write!(f, "end"),

            // Arithmetic operators
            TokenType::AdditionOperator => write!(f, "+"),
            TokenType::SubtractionOperator => write!(f, "-"),
            TokenType::MultiplicationOperator => write!(f, "*"),
            TokenType::DivisionOperator => write!(f, "/"),
            TokenType::ModuloOperator => write!(f, "%"),
            TokenType::ExponentiationOperator => write!(f, "**"),
        }
    }
}

pub(super) fn lexer() -> impl Parser<char, Vec<Token>, Error = Simple<char>> {
    choice((literal::literal(), keyword::keywords()))
        .recover_with(skip_then_retry_until([]))
        .repeated()
        .then(end())
        .map(|(res, _)| res)
}

/// Produces a tuple containing a stream of [`Token`]s (or `None`, if the tokeniser failed) and a stream of
/// errors.
///
/// # Arguments
/// * `input` - The input to tokenise
pub fn lex(input: &str) -> (Option<Vec<Token>>, Vec<Simple<char>>) {
    lexer().parse_recovery(input)
}

/// A single unit of tokenisation, containing its variant and its location within the given input.
#[derive(Debug)]
pub struct Token {
    token_type: TokenType,
    span: Span,
}

impl Token {
    pub fn new(token_type: TokenType, span: Span) -> Self {
        Self { token_type, span }
    }
}

impl PartialEq for Token {
    fn eq(&self, other: &Self) -> bool {
        self.token_type == other.token_type && self.span == other.span
    }
}

#[allow(dead_code)]
pub(crate) fn expect_lexer(
    input: &str,
    parser: impl Parser<char, Token, Error = Simple<char>>,
    expect_case: TokenType,
) {
    let (val, _) = parser.parse_recovery(input);
    match val {
        Some(tok) => assert_eq!(tok.token_type, expect_case),
        None => panic!("parse_recovery returned a None case."),
    }
}
