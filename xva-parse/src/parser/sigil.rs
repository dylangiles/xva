//! Sigil parsing module
//!
//! Parses things that aren't words, like control characters, operators, formatting characters, etc.

use chumsky::{prelude::*, primitive::select};
use xva_ast::ast::{BinaryOperator, UnaryOperator};
use xva_span::SourceSpan;

use crate::token::{Delimiter, Token, TokenKind};

use super::ParserExtras;

/// Parses a single unary operator, that is valid for addition expressions:
/// - `not` for logical negation,
/// - `-` for arithmetic negation,
pub(super) fn unary_op<'src>(
) -> impl Parser<'src, &'src [Token], UnaryOperator, ParserExtras> + Clone {
    select(move |tok: Token, _| match tok.kind() {
        TokenKind::Minus => Some(UnaryOperator::Negation),
        TokenKind::Not => Some(UnaryOperator::Not),
        _ => None,
    })
}

/// Parses a single binary operator, that is valid for product expressions
pub(super) fn product_op<'src>(
) -> impl Parser<'src, &'src [Token], BinaryOperator, ParserExtras> + Clone {
    select(move |tok: Token, _| match tok.kind() {
        TokenKind::Star => Some(BinaryOperator::Multiply),
        TokenKind::Slash => Some(BinaryOperator::Divide),
        TokenKind::Percent => Some(BinaryOperator::Modulo),
        TokenKind::DoubleStar => Some(BinaryOperator::Power),
        _ => None,
    })
}

/// Parses a single binary operator, that is valid for addition expressions
pub(super) fn sum_op<'src>(
) -> impl Parser<'src, &'src [Token], BinaryOperator, ParserExtras> + Clone {
    select(move |tok: Token, _| match tok.kind() {
        TokenKind::Plus => Some(BinaryOperator::Add),
        TokenKind::Minus => Some(BinaryOperator::Subtract),
        _ => None,
    })
}

/// Parses a single opening delimiter of the specified kind and produces the span at which it occurred.
pub(super) fn open_delim<'src>(
    kind: Delimiter,
) -> impl Parser<'src, &'src [Token], SourceSpan, ParserExtras> + Clone {
    select(move |tok: Token, _| {
        let matched = if let TokenKind::OpenDelim(delim) = tok.kind() {
            delim
        } else {
            return None;
        };

        if matched == kind {
            Some(tok.span)
        } else {
            None
        }
    })
}

/// Parses a single closing delimiter of the specified kind and produces the span at which it occurred.
pub(super) fn close_delim<'src>(
    kind: Delimiter,
) -> impl Parser<'src, &'src [Token], SourceSpan, ParserExtras> + Clone {
    select(move |tok: Token, _| {
        let matched = if let TokenKind::CloseDelim(delim) = tok.kind() {
            delim
        } else {
            return None;
        };

        if matched == kind {
            Some(tok.span)
        } else {
            None
        }
    })
}

/// Wrapper around [`open_delim`] for parentheses only
pub(super) fn open_paren<'src>(
) -> impl Parser<'src, &'src [Token], SourceSpan, ParserExtras> + Clone {
    open_delim(Delimiter::Parentheses)
}

/// Wrapper around [`close_delim`] for parentheses only
pub(super) fn close_paren<'src>(
) -> impl Parser<'src, &'src [Token], SourceSpan, ParserExtras> + Clone {
    close_delim(Delimiter::Parentheses)
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum Op {
    /// The `=` operator
    Assign,

    /// The `:` symbol
    Colon,
}

pub(super) fn just_operator<'src>(
    kind: Op,
) -> impl Parser<'src, &'src [Token], (Op, SourceSpan), ParserExtras> + Clone {
    select(move |tok: Token, _| {
        let matched = match tok.kind() {
            TokenKind::Equals => Op::Assign,
            TokenKind::Colon => Op::Colon,
            _ => return None,
        };

        if matched == kind {
            Some((matched, tok.span))
        } else {
            None
        }
    })
}
