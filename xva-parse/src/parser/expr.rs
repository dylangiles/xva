use chumsky::{prelude::*, primitive::select, Parser};
use xva_ast::ast::{Expression, ExpressionKind, Item, ItemKind, LiteralKind};

mod utils;

use self::utils::left_fold_into_binary_expr;

use super::{
    next_node_id,
    sigil::{close_paren, open_paren, product_op, sum_op, unary_op},
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

pub(super) fn expression_inner<'src>(
) -> impl Parser<'src, &'src [Token], Expression, ParserExtras> + Clone {
    recursive(|expr| {
        // An atom is a completely unambigious expression:
        let atom = literal() // Literals, or
            .or(expr.clone().delimited_by(open_paren(), close_paren())); // expressions enclosed in parentheses

        // With parser combinators, precedence is done by defining a parser in terms of the parser with the
        // next highest precedence, and so on. To get recursive expressions, such as 1 + 2 * 3 / 4, we start with
        // an initial parse of something, then continously fold an operator and another "something" onto itself,
        // producing an Expression node after each fold. The direction in which we fold is the same as the
        // associativity of the expression.

        // Unary expressions are right-associative: a repeated unary operator, right-folded on to an atom.
        let unary = unary_op().repeated().foldr(atom.clone(), |op, rhs| {
            let span = rhs.span.clone();
            Expression {
                id: next_node_id(),
                kind: ExpressionKind::Unary(op, Box::from(rhs)),
                span,
            }
        });

        // Binary expressions are similar to unaries, but they are left-associative. The first expression type with
        // a higher precedence are unaries,  so we define products in terms of unaries: unary, followed by a
        // product operator, repeating, folding left.
        let product = unary.clone().foldl(
            product_op().then(unary).repeated(),
            left_fold_into_binary_expr,
        );

        // Same as products, but now we're in **terms of** products
        let sum = product.clone().foldl(
            sum_op().then(product).repeated(),
            left_fold_into_binary_expr,
        );

        sum
    })
}
pub(crate) fn expression<'src>() -> impl Parser<'src, &'src [Token], Item, ParserExtras> + Clone {
    // recursive(|expr| {
    //     // An atom is a completely unambigious expression:
    //     let atom = literal() // Literals, or
    //         .or(expr.clone().delimited_by(open_paren(), close_paren())); // expressions enclosed in parentheses

    //     // With parser combinators, precedence is done by defining a parser in terms of the parser with the
    //     // next highest precedence, and so on. To get recursive expressions, such as 1 + 2 * 3 / 4, we start with
    //     // an initial parse of something, then continously fold an operator and another "something" onto itself,
    //     // producing an Expression node after each fold. The direction in which we fold is the same as the
    //     // associativity of the expression.

    //     // Unary expressions are right-associative: a repeated unary operator, right-folded on to an atom.
    //     let unary = unary_op().repeated().foldr(atom.clone(), |op, rhs| {
    //         let span = rhs.span.clone();
    //         Expression {
    //             id: next_node_id(),
    //             kind: ExpressionKind::Unary(op, Box::from(rhs)),
    //             span,
    //         }
    //     });

    //     // Binary expressions are similar to unaries, but they are left-associative. The first expression type with
    //     // a higher precedence are unaries,  so we define products in terms of unaries: unary, followed by a
    //     // product operator, repeating, folding left.
    //     let product = unary.clone().foldl(
    //         product_op().then(unary).repeated(),
    //         left_fold_into_binary_expr,
    //     );

    //     // Same as products, but now we're in **terms of** products
    //     let sum = product.clone().foldl(
    //         sum_op().then(product).repeated(),
    //         left_fold_into_binary_expr,
    //     );

    //     sum
    // })
    expression_inner().map(|expr| {
        let span = expr.span.clone();
        Item {
            id: next_node_id(),
            kind: ItemKind::Expression(expr),
            span,
        }
    })
}
