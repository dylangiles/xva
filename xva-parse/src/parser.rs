//! The parser, or syntactical analysis. Compilation stage 2
//!
//! The parser is, like the lexer, also a parser combinator. The combinators are again provided by [`chumsky`], but
//! unlike the lexer, the parser operates on complete tokens (produced by the lexer) rather than slices of the
//! source text.
//!
//! The four magic Chumsky-related types for the parser are:
//! - Lifetime: `'src`, the lifetime of the input, same as the lexer,
//! - Input: `&'src [Token]`, a slice of [`Token`]s that muse live at least as long as the parser (hence `'src``),
//! - Output: [`Item`], a single node of the abstract syntax tree
//! - Extra: [`SyntaxError`], wrapped in Chumsky's [`extra::Err`](chumsky::extra::Err). To avoid writing out
//! the whole thing, there is a type alias for the wrapped type: [`ParserExtra`]
//!
//! The ultimate output of this module is an [`Item`] - an abstract syntax "tree" of constructs in the language.

use chumsky::prelude::*;
use std::sync::atomic::{AtomicI64, Ordering};
use xva_ast::{ast::Item, node_id::NodeId};
use xva_span::SourceId;

mod expr;

use crate::{
    error::{ErrorPattern, SyntaxError, SyntaxErrorKind},
    lexer::lex,
    token::Token,
};

use self::expr::expression;

pub(self) static NODE_ID_SEED: AtomicI64 = AtomicI64::new(0);
pub(self) fn next_node_id() -> NodeId {
    NODE_ID_SEED.fetch_add(1, Ordering::SeqCst).into()
}

pub(crate) type ParserError = extra::Err<SyntaxError>;
pub fn parse<'src>(
    input: &'src str,
    src_id: SourceId,
    debug_lexer: bool,
) -> (Item, Vec<SyntaxError>) {
    let (tokens, lex_errors) = lex(input, src_id, debug_lexer);

    let (tree, parse_errors) = parser().parse(tokens.as_slice()).into_output_errors();

    // SAFETY: the parser is infallible - it will always produce a tree, even if the tree is empty.
    (
        tree.unwrap(),
        lex_errors
            .into_iter()
            .chain(parse_errors.into_iter())
            .collect(),
    )
}

pub(crate) fn parser<'src>() -> impl Parser<'src, &'src [Token], Item, extra::Err<SyntaxError>> {
    expression().or(any().validate(|tok: Token, _extra, emitter| {
        emitter.emit(SyntaxError::new(
            SyntaxErrorKind::UnexpectedPattern(ErrorPattern::Token(tok.kind)),
            tok.span,
        ));

        Item::error(tok.span, tok.original.into())
    }))
}

#[cfg(test)]
mod tests {

    // use crate::{lexer::lex, parser::parser};

    // #[test]
    // fn tree() {
    //     let input = "keyword";

    //     let len = input.bytes().len();
    //     let eoi = SourceSpan::new(0, len);
    //     let (tokens, lex_errors) = lex(input, false);

    //     let (ast, parse_errs) = parser()
    //         .map_with(|ast, e| (ast, e.span()))
    //         .parse(tokens.as_slice().spanned((input.len()..input.len()).into()))
    //         .into_output_errors();

    //     println!("tree: {:#?}", ast);
    // }
}
