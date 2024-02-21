use chumsky::{prelude::*, primitive::select};
use xva_ast::ast::{
    BindingFlags, BindingKind, BindingPattern, Item, ItemKind, Local, Statement, StatementKind,
};
use xva_span::{CheapRange, SourceSpan};

use crate::token::{Token, TokenKind};

use super::{
    expr::{expression, expression_inner},
    ident::ident,
    next_node_id,
    operator::{just_operator, Op},
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
            just_operator(Op::Assign)
                .ignored()
                .then(expression_inner())
                .or_not()
                .map(|x| x.map(|(_, expr)| expr)),
        )
        .map(|(((_, kw_span), ident), maybe_expr)| {
            let span = kw_span.copy_from_ending_at(ident.span.end());
            Statement {
                id: next_node_id(),
                kind: StatementKind::Local(Local {
                    id: next_node_id(),
                    kind: maybe_expr.map_or_else(
                        || BindingKind::Declared,
                        |expr| BindingKind::Inited(Box::from(expr)),
                    ),
                    span,
                    binding_flags: BindingFlags { mutable: true },
                    pattern: BindingPattern::Identifier(ident),
                }),
                span,
            }
        })
}

fn local<'src>() -> impl Parser<'src, &'src [Token], Statement, ParserExtras> + Clone {
    let immutable_binding = keyword(Kw::Let)
        .map(|(_, kw_span)| kw_span)
        .then(ident())
        .then_ignore(just_operator(Op::Assign))
        .then(expression_inner())
        .map(|((kw_span, ident), expr)| {
            let span = kw_span.copy_from_ending_at(ident.span.end());

            Statement {
                id: next_node_id(),
                kind: StatementKind::Local(Local {
                    id: next_node_id(),
                    kind: BindingKind::Inited(Box::from(expr)),
                    span,
                    binding_flags: BindingFlags { mutable: false },
                    pattern: BindingPattern::Identifier(ident),
                }),
                span,
            }
        });

    immutable_binding.or(variable())
}

pub(super) fn statement<'src>() -> impl Parser<'src, &'src [Token], Item, ParserExtras> + Clone {
    choice((local(),)).map(|stmt| {
        let span = stmt.span.clone();
        Item {
            id: next_node_id(),
            kind: ItemKind::Statement(stmt),
            span,
        }
    })
}
