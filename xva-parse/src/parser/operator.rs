use chumsky::{prelude::*, primitive::select};
use xva_ast::ast::{BinaryOperator, UnaryOperator};

use crate::token::{Delimiter, Token, TokenKind};

use super::ParserExtras;

pub(super) fn unary_op<'src>(
) -> impl Parser<'src, &'src [Token], UnaryOperator, ParserExtras> + Clone {
    select(move |tok: Token, _| match tok.kind() {
        TokenKind::Minus => Some(UnaryOperator::Negation),
        TokenKind::Not => Some(UnaryOperator::Not),
        _ => None,
    })
}
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

pub(super) fn sum_op<'src>(
) -> impl Parser<'src, &'src [Token], BinaryOperator, ParserExtras> + Clone {
    select(move |tok: Token, _| match tok.kind() {
        TokenKind::Plus => Some(BinaryOperator::Add),
        TokenKind::Minus => Some(BinaryOperator::Subtract),
        _ => None,
    })
}

pub(super) fn open_paren<'src>() -> impl Parser<'src, &'src [Token], (), ParserExtras> + Clone {
    select(move |tok: Token, _| {
        if let TokenKind::OpenDelim(delim) = tok.kind() {
            if delim == Delimiter::Parentheses {
                Some(())
            } else {
                None
            }
        } else {
            None
        }
    })
}

pub(super) fn close_paren<'src>() -> impl Parser<'src, &'src [Token], (), ParserExtras> + Clone {
    select(move |tok: Token, _| {
        if let TokenKind::CloseDelim(delim) = tok.kind() {
            if delim == Delimiter::Parentheses {
                Some(())
            } else {
                None
            }
        } else {
            None
        }
    })
}
