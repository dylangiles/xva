use chumsky::{prelude::*, primitive::select, Parser};
use xva_ast::ast::{Expression, ExpressionKind, Item, ItemKind, LiteralKind};

mod utils;

use self::utils::left_fold_into_binary_expr;

use super::{
    next_node_id,
    operator::{close_paren, open_paren, product_op, sum_op, unary_op},
    ParserExtras,
};

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
}

pub(crate) fn expression<'src>() -> impl Parser<'src, &'src [Token], Item, ParserExtras> + Clone {
    recursive(|expr| {
        let atom = literal().or(expr.clone().delimited_by(open_paren(), close_paren()));

        let unary = unary_op().repeated().foldr(atom.clone(), |op, rhs| {
            let span = rhs.span.clone();
            Expression {
                id: next_node_id(),
                kind: ExpressionKind::Unary(op, Box::from(rhs)),
                span,
            }
        });

        // We take a unary parser, then a binary operator and the unary parser repeated, continously folding
        // that onto itself and producing an expression each time.
        let product = unary.clone().foldl(
            product_op().then(unary).repeated(),
            left_fold_into_binary_expr,
        );

        // Sums are lower precedence than products, so we do the same thing as products except looking for products
        // instead of unaries. Take a product, then a sum operator and a product repeated, continuously folding and
        // producing an expression.
        let sum = product.clone().foldl(
            sum_op().then(product).repeated(),
            left_fold_into_binary_expr,
        );

        sum
    })
    .map(|expr| {
        let span = expr.span.clone();
        Item {
            id: next_node_id(),
            kind: ItemKind::Expression(expr),
            span,
        }
    })
}
