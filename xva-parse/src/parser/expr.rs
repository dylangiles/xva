use chumsky::{extra::ParserExtra, prelude::*, primitive::select, Parser};
use xva_ast::ast::{Expression, ExpressionKind, Item, ItemKind, LiteralKind, UnaryOperator};
use xva_span::{CheapRange, SourceSpan};

use super::{next_node_id, ParserError};
use crate::{
    error::SyntaxError,
    token::{Token, TokenKind},
};

fn literal<'src>() -> impl Parser<'src, &'src [Token], Expression, extra::Err<SyntaxError>> + Clone
{
    select(move |token: Token, _| match token.kind {
        TokenKind::Boolean(b) => Some((LiteralKind::Boolean(b), token.span)),
        TokenKind::Char(c) => Some((LiteralKind::Char(c), token.span)),
        TokenKind::Integer(i) => Some((LiteralKind::Integer(i), token.span)),
        TokenKind::Float(f) => Some((LiteralKind::Float(f), token.span)),
        _ => None,
    })
    .map(|(lit, span)| Expression {
        id: next_node_id(),
        kind: ExpressionKind::Literal(lit),
        span,
    })
    // .map(|(lit, span)| Item {
    //     id: next_node_id(),
    //     kind: ItemKind::Expression(Expression {
    //         id: next_node_id(),
    //         kind: ExpressionKind::Literal(lit),
    //         span,
    //     }),
    //     span,
    // })
}

fn unary_operator<'src>(
) -> impl Parser<'src, &'src [Token], UnaryOperator, extra::Err<SyntaxError>> + Clone {
    select(|token: Token, _| match token.kind() {
        TokenKind::Not => Some(UnaryOperator::Not),
        TokenKind::Minus => Some(UnaryOperator::Negation),
        _ => None,
    })
}

// fn unary<'src>() -> impl Parser<'src, &'src [Token], Item, ParserError> {
//     unary_operator().then()
// }

fn just_kind<'src>(
    kind: TokenKind,
) -> impl Parser<'src, &'src [Token], TokenKind, ParserError> + Clone {
    select(move |x: Token, _| {
        if core::mem::discriminant(&x.kind()) == core::mem::discriminant(&kind) {
            Some(kind)
        } else {
            None
        }
    })
}

fn unary<'src>() -> impl Parser<'src, &'src [Token], Expression, ParserError> + Clone {
    let atom = literal();

    select(move |tok: Token, _| match tok.kind() {
        TokenKind::Minus => Some((UnaryOperator::Negation, tok)),
        TokenKind::Not => Some((UnaryOperator::Not, tok)),
        _ => None,
    })
    .then(atom.map(|x| x))
    .map(|((op, op_tok), expr)| {
        let src_id = op_tok.span.src();
        let start = op_tok.span.start();
        let end = expr.span.end();
        Expression {
            id: next_node_id(),
            kind: ExpressionKind::Unary(op, Box::from(expr)),
            span: SourceSpan::new(src_id, CheapRange::new(start, end)),
        }
    })
}

pub(crate) fn expression<'src>() -> impl Parser<'src, &'src [Token], Item, ParserError> + Clone {
    choice((literal(), unary())).map(|expr| {
        let span = expr.span.clone();
        Item {
            id: next_node_id(),
            kind: ItemKind::Expression(expr),
            span,
        }
    })
}

// pub(crate) fn expression<'src>() -> impl Parser<'src, &'src [Token], Item, ParserError> + Clone {
//     recursive(|expr| {
//         let atom = choice((literal(),));

//         let unary_op = select(|token: Token, _| match token.kind {
//             TokenKind::Not => Some(UnaryOperator::Not),
//             TokenKind::Minus => Some(UnaryOperator::Negation),
//             _ => None,
//         });

//         let unary = just(unary_op);
//         // let unary = just("-").then(atom)

//         choice((unary, atom))
//     })
// }
