use std::sync::atomic::{AtomicU64, Ordering};

use xva_ast::ast::{self};

pub mod expr;
pub mod item;
pub mod stmt;

// use self::expr::{Expression, ExpressionKind};

use self::{
    expr::{Expression, ExpressionKind},
    item::Item,
    stmt::{
        BindingFlags, BindingKind, BindingPattern, Identifier, Local, Statement, StatementKind,
    },
};
use crate::id::HirId;

pub(self) static NODE_ID_SEED: AtomicU64 = AtomicU64::new(0);
pub(self) fn next_hir_id() -> HirId {
    NODE_ID_SEED.fetch_add(1, Ordering::SeqCst).into()
}

pub struct HirContext {}

impl HirContext {
    pub fn new() -> Self {
        Self {}
    }

    pub fn lower(&self, ast: ast::Item) -> Item {
        match ast.kind {
            ast::ItemKind::Statement(stmt) => {
                let kind = match stmt.kind {
                    ast::StatementKind::Local(local) => StatementKind::Local(Local {
                        id: next_hir_id(),
                        span: local.span,
                        binding_kind: match local.binding_kind {
                            ast::BindingKind::Declared => BindingKind::Declared,
                            ast::BindingKind::Inited(expr) => {
                                BindingKind::Inited(Box::new(self.lower_expr(*expr)))
                            }
                        },
                        binding_flags: BindingFlags {
                            mutable: local.binding_flags.mutable,
                        },
                        pattern: match local.pattern {
                            ast::BindingPattern::Identifier(ident) => {
                                BindingPattern::Identifier(Identifier {
                                    name: ident.name,
                                    span: ident.span,
                                })
                            }
                        },
                        ty: local.ty,
                    }),
                };

                Item {
                    id: next_hir_id(),
                    kind: item::ItemKind::Statement(Statement {
                        id: next_hir_id(),
                        span: ast.span.clone(),
                        kind,
                    }),
                    span: ast.span,
                }
            }
            _ => todo!(),
        }
    }

    fn lower_expr(&self, ast: ast::Expression) -> Expression {
        let kind = match ast.kind {
            ast::ExpressionKind::Literal(lit) => ExpressionKind::Literal(lit),
            _ => todo!(),
        };

        Expression {
            id: next_hir_id(),
            span: ast.span,
            kind,
        }
    }

    // fn lower_ty(&self, ty: Option<ast::Type>) -> Option<
}
