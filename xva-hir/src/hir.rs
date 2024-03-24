use std::sync::atomic::{AtomicI64, Ordering};

use xva_ast::ast::{self};
use xva_middle::{typechk::expr::TypeExpr, Type, TypeContext};
use xva_span::SourceSpan;

pub mod expr;
pub mod item;
pub mod name;
pub mod stmt;
pub mod ty;

// use self::expr::{Expression, ExpressionKind};

use self::{
    expr::{Expression, ExpressionKind},
    item::Item,
    stmt::{
        BindingFlags, BindingKind, BindingPattern, Identifier, Local, Statement, StatementKind,
    },
    ty::{TypeAnno, TypeKind},
};
use crate::{error::HirError, id::HirId, ItemKind};

pub(self) static NODE_ID_SEED: AtomicI64 = AtomicI64::new(0);
pub(self) fn next_hir_id() -> HirId {
    NODE_ID_SEED.fetch_add(1, Ordering::SeqCst).into()
}

pub type HirResult<T> = Result<T, HirError>;
pub struct HirContext {
    tcx: TypeContext,
}

impl HirContext {
    pub fn new() -> Self {
        Self {
            tcx: TypeContext::default(),
        }
    }

    pub fn lower(&self, ast: ast::Item) -> HirResult<Item> {
        let item = match ast.kind {
            ast::ItemKind::Statement(stmt) => {
                let kind = match stmt.kind {
                    ast::StatementKind::Local(local) => {
                        let type_anno = if let Some(ast_anno) = &local.ty {
                            Some(self.lower_type_anno(ast_anno))
                        } else {
                            None
                        };

                        let expr = match local.binding_kind {
                            ast::BindingKind::Declared => None,
                            ast::BindingKind::Inited(expr) => Some(self.lower_expr(*expr)),
                        };

                        StatementKind::Local(Local {
                            id: next_hir_id(),
                            span: local.span,
                            binding_kind: match expr.clone() {
                                Some(e) => BindingKind::Inited(e.into()),
                                None => BindingKind::Declared,
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
                            type_anno: type_anno.clone(),
                            ty: self.type_check(local.span, type_anno, expr)?,
                        })
                    }
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

            ast::ItemKind::Error(e) => Item {
                id: next_hir_id(),
                kind: ItemKind::Error(e),
                span: ast.span,
            },

            unknown => panic!("Unknown AST variant: {unknown:#?}"),
        };

        Ok(item)
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

    fn type_check(
        &self,
        start: SourceSpan,
        anno: Option<TypeAnno>,
        expr: Option<Expression>,
    ) -> HirResult<Type> {
        match anno {
            Some(anno) => match anno.kind {
                TypeKind::Unit => Ok(Type::Unit),
                TypeKind::Infer => self.type_infer(start, expr),
                TypeKind::Named(name) => {
                    let var = Type::Variable(name.normalise().into());
                    match expr {
                        Some(expr) => {
                            println!("anno.span: {:?}", anno.span);
                            match self.tcx.check(&self.lower_expr_to_type_expr(&expr), &var) {
                                Ok(ty) => Ok(ty),
                                Err(err) => Err(HirError::type_error(start, err, anno.span)),
                            }
                        }
                        None => todo!(),
                    }
                }
            },
            None => self.type_infer(start, expr),
        }
    }

    fn type_infer(&self, start: SourceSpan, expr: Option<Expression>) -> HirResult<Type> {
        match expr {
            Some(expr) => {
                let ty_expr = match expr.kind {
                    ExpressionKind::Literal(lit) => TypeExpr::Literal(lit),
                };

                match self.tcx.synthesise(&ty_expr) {
                    Ok(ty) => Ok(ty),
                    Err(err) => Err(HirError::type_error(start, err, expr.span)),
                }
            }
            None => Err(HirError::type_anno_needed(start, None)),
        }
    }

    fn lower_expr_to_type_expr(&self, expr: &Expression) -> TypeExpr {
        match expr.kind {
            ExpressionKind::Literal(lit) => TypeExpr::Literal(lit),
        }
    }

    fn lower_type_anno(&self, anno: &ast::TypeAnno) -> TypeAnno {
        let id = anno.id.into();
        println!("lowering type anno, span: {:?}", anno.span);
        let span = anno.span;

        TypeAnno {
            id,
            kind: match &anno.kind {
                ast::TypeKind::Unit => TypeKind::Unit,
                ast::TypeKind::Infer => TypeKind::Infer,
                ast::TypeKind::Named(name) => TypeKind::Named(name.clone()),
            },
            span,
        }
    }

    // fn lower_ty(&self, ty: Option<ast::Type>) -> Option<
}
