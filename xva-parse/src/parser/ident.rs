use chumsky::{prelude::*, primitive::select};
use xva_ast::ast::Identifier;

use crate::token::{Token, TokenKind};

use super::ParserExtras;

pub(super) fn ident<'src>() -> impl Parser<'src, &'src [Token], Identifier, ParserExtras> + Clone {
    select(move |tok: Token, _| {
        if let TokenKind::Identifier(_) = tok.kind() {
            Some(tok)
        } else {
            None
        }
    })
    .map(|tok| match tok.kind() {
        TokenKind::Identifier(name) => Identifier {
            name,
            span: tok.span,
        },
        _ => unreachable!(),
    })
}
