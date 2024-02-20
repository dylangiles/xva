use crate::utils::intern_str;

use super::{LexerExtra, TokenKind};
use chumsky::prelude::*;

const KEYWORD_LET: &str = "let";
const KEYWORD_VAR: &str = "var";

pub(crate) fn ident_or_keyword<'src>() -> impl Parser<'src, &'src str, TokenKind, LexerExtra> {
    text::ident().map(|ident| match ident {
        KEYWORD_LET => TokenKind::Let,
        KEYWORD_VAR => TokenKind::Var,
        _ => TokenKind::Identifier(intern_str(ident)),
    }) //.and_is(keyword().not()) TODO
}
