use chumsky::{prelude::*, primitive::select};
use xva_ast::ast::{Name, NameSegment, Type, TypeKind};
use xva_span::{CheapRange, SourceSpan};

use crate::token::{Delimiter, Token, TokenKind};

use super::{
    ident::ident,
    next_node_id,
    sigil::{close_delim, open_delim},
    ParserExtras,
};

fn dot<'src>() -> impl Parser<'src, &'src [Token], (), ParserExtras> + Clone {
    select(move |tok: Token, _| match tok.kind() {
        TokenKind::Dot => Some(()),
        _ => None,
    })
}

fn ty_named<'src>() -> impl Parser<'src, &'src [Token], Type, ParserExtras> + Clone {
    let name_segment = ident().map(|ident| NameSegment {
        id: next_node_id(),
        ident,
    });

    name_segment
        .clone()
        .then(
            dot()
                .ignore_then(name_segment)
                .repeated()
                .at_least(1)
                .collect::<Vec<_>>()
                .or_not(),
        )
        .map(|(first, rest)| match rest {
            Some(segments) => {
                let (first_span, last_span) = (
                    segments.first().unwrap().ident.span,
                    segments.last().unwrap().ident.span,
                );

                let span = SourceSpan::new(
                    first_span.src(),
                    CheapRange::new(first_span.start(), last_span.start()),
                );

                Type {
                    id: next_node_id(),
                    kind: TypeKind::Named(Name {
                        id: next_node_id(),
                        span: span.clone(),
                        segments: {
                            let mut temp = Vec::with_capacity(1 + segments.len());
                            temp.push(first);
                            for seg in segments {
                                temp.push(seg);
                            }
                            temp
                        },
                    }),
                    span,
                }
            }
            None => {
                let span = first.ident.span.clone();
                Type {
                    id: next_node_id(),
                    kind: TypeKind::Named(Name {
                        id: next_node_id(),
                        span: span.clone(),
                        segments: vec![first; 1],
                    }),
                    span,
                }
            }
        })
}

fn ty_builtin<'src>() -> impl Parser<'src, &'src [Token], Type, ParserExtras> + Clone {
    let unit = open_delim(Delimiter::Parentheses)
        .then(close_delim(Delimiter::Parentheses))
        .map(|(start, end)| Type {
            id: next_node_id(),
            kind: TypeKind::Unit,
            span: SourceSpan::from_start_end(start, end),
        });

    // TODO: Never type, diverging code.
    choice((unit,))
}

pub(super) fn ty<'src>() -> impl Parser<'src, &'src [Token], Type, ParserExtras> + Clone {
    choice((ty_builtin(), ty_named()))
}

#[cfg(test)]
mod tests {
    use chumsky::Parser;
    use xva_ast::ast::TypeKind;

    use crate::lexer::lex;

    #[test]
    fn unit_type() {
        let input = "()";
        let (tokens, _) = lex(input, 0u32.into(), false);
        let (tree, _) = super::ty().parse(tokens.as_slice()).into_output_errors();
        let ty = tree.unwrap();
        match ty.kind {
            TypeKind::Unit => (),
            _ => panic!("type: {:#?}", ty.kind),
        }
    }

    #[test]

    fn named_type_multiple_segments() {
        let input = "std.module.item";
        let (tokens, _) = lex(input, 0u32.into(), false);
        let (tree, _) = super::ty().parse(tokens.as_slice()).into_output_errors();
        let ty = tree.unwrap();
        match ty.kind {
            TypeKind::Named(name) => {
                if name.segments[0].ident.name.as_str() != "std"
                    || name.segments[1].ident.name.as_str() != "module"
                    || name.segments[2].ident.name.as_str() != "item"
                {
                    panic!("")
                }
            }
            _ => panic!("type: {:#?}", ty.kind),
        }
    }

    #[test]
    fn named_type_one_segment() {
        let input = "bool";
        let (tokens, _) = lex(input, 0u32.into(), false);
        let (tree, _) = super::ty().parse(tokens.as_slice()).into_output_errors();
        let ty = tree.unwrap();
        match ty.kind {
            TypeKind::Named(name) => {
                if name.segments[0].ident.name.as_str() != "bool" {
                    panic!("")
                }
            }
            _ => panic!("type: {:#?}", ty.kind),
        }
    }
}
