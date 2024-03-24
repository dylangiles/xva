use chumsky::{input::Emitter, prelude::*, primitive::select};
use xva_ast::ast::{BindingFlags, BindingPattern, Item, ItemKind, Local, Statement, StatementKind};
use xva_span::SourceSpan;

use crate::{
    error::SyntaxErrorKind,
    token::{Token, TokenKind},
    utils::intern_str,
    SyntaxError,
};

use super::{
    expr::expression_inner,
    ident::ident,
    next_node_id,
    sigil::{just_operator, Op},
    ty::type_anno,
    ParserExtras,
};

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub(crate) enum Kw {
    Let,
    Var,
}

fn keyword<'src>(
    kind: Kw,
) -> impl Parser<'src, &'src [Token], (Kw, SourceSpan), ParserExtras> + Clone {
    select(move |tok: Token, _| {
        let matched = match tok.kind() {
            TokenKind::Let => Kw::Let,
            TokenKind::Var => Kw::Var,
            _ => return None,
        };

        if matched == kind {
            Some((matched, tok.span))
        } else {
            None
        }
    })
}

fn variable<'src>() -> impl Parser<'src, &'src [Token], Statement, ParserExtras> + Clone {
    keyword(Kw::Var)
        .then(ident())
        .then(
            just_operator(Op::Colon)
                .then(type_anno())
                .or_not()
                .map(|x| x.map(|(_, ty)| ty)),
        )
        .then(
            just_operator(Op::Assign)
                .ignored()
                .then(expression_inner())
                .or_not()
                .map(|x| x.map(|(_, expr)| expr)),
        )
        .map(|((((_, kw_span), ident), maybe_ty), maybe_expr)| {
            let span = kw_span.copy_from_ending_at(ident.span.end());
            Statement {
                id: next_node_id(),
                kind: StatementKind::Local(Local {
                    id: next_node_id(),
                    expr: maybe_expr,
                    span,
                    binding_flags: BindingFlags { mutable: true },
                    pattern: BindingPattern::Identifier(ident),
                    ty: maybe_ty,
                }),
                span,
            }
        })
}

fn local<'src>() -> impl Parser<'src, &'src [Token], Statement, ParserExtras> + Clone {
    let immutable_binding = keyword(Kw::Let)
        .map(|(_, kw_span)| kw_span)
        .then(ident())
        .then(
            just_operator(Op::Colon)
                .then(type_anno())
                .or_not()
                .map(|x| x.map(|(_, ty)| ty)),
        )
        .then(just_operator(Op::Assign).then(expression_inner()).or_not())
        .map(|(((kw_span, ident), maybe_ty), maybe_expr)| {
            let span = kw_span.copy_from_ending_at(ident.span.end());

            Statement {
                id: next_node_id(),
                kind: StatementKind::Local(Local {
                    id: next_node_id(),
                    expr: maybe_expr.map_or(None, |(_, expr)| Some(expr)),
                    span,
                    binding_flags: BindingFlags { mutable: false },
                    pattern: BindingPattern::Identifier(ident),
                    ty: maybe_ty,
                }),
                span,
            }
        });

    immutable_binding.or(variable())
}

fn validate_local(stmt: Statement, emitter: &mut Emitter<SyntaxError>) -> Item {
    match &stmt.kind {
        StatementKind::Local(local) => {
            let make_stmt = move |s| {
                let span = stmt.span.clone();
                Item {
                    id: next_node_id(),
                    kind: ItemKind::Statement(s),
                    span,
                }
            };

            // If the local was not initialised
            if let None = &local.expr {
                // and it is not declared as mutable (`var`)
                if !local.binding_flags.mutable {
                    // Raise a syntax error
                    let expr_start = stmt.span.copy_from_starting_at(stmt.span.end());

                    emitter.emit(SyntaxError::new(
                        SyntaxErrorKind::UninitedImmutable { expr_start },
                        stmt.span,
                    ));

                    Item::error(stmt.span, intern_str(""))
                } else {
                    make_stmt(stmt)
                }
            } else {
                make_stmt(stmt)
            }
        }
    }
}

pub(super) fn statement<'src>() -> impl Parser<'src, &'src [Token], Item, ParserExtras> + Clone {
    let local = local().validate(|s, _, e| validate_local(s, e));

    choice((local,))
}
