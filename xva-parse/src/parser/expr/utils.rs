use chumsky::prelude::*;
use xva_ast::ast::{BinaryOperator, Expression, ExpressionKind};
use xva_span::{CheapRange, SourceSpan};

use crate::parser::next_node_id;

/// Helper function to reduce the amount of noise in the folding calls while parsing binary expressions.
///
/// Means you can do
/// ```ignore
/// .foldl(..., binary_expr_from_left_fold)
/// ```
///
/// without having to write a big closure in a `.map()`.
pub(super) fn left_fold_into_binary_expr(
    left: Expression,
    other: (BinaryOperator, Expression),
) -> Expression {
    let (op, right) = other;
    let (start, src_id) = (left.span.start(), left.span.src());
    let end = right.span.end();
    Expression {
        id: next_node_id(),
        kind: ExpressionKind::Binary(op, Box::from(left), Box::from(right)),
        span: SourceSpan::new(src_id, CheapRange::new(start, end)),
    }
}
