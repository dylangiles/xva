use xva_ast::ast;
use xva_middle::{typechk::expr::TypeExpr, Type};
use xva_span::SourceSpan;

mod expr;
mod stmt;

use crate::hir::{next_hir_id, HirResult};
use crate::{error::HirError, ItemKind};
use crate::{
    hir::{
        expr::{Expression, ExpressionKind},
        item::Item,
        stmt::BindingPattern,
        ty::{TypeAnno, TypeKind},
    },
    HirContext,
};

impl HirContext {
    pub fn lower(&mut self, ast: ast::Item) -> HirResult<Item> {
        let item = match ast.kind {
            ast::ItemKind::Statement(stmt) => self.lower_stmt(stmt)?,

            ast::ItemKind::Error(e) => Item {
                id: next_hir_id(),
                kind: ItemKind::Error(e),
                span: ast.span,
            },

            unknown => panic!("Unknown AST variant: {unknown:#?}"),
        };

        Ok(item)
    }

    fn type_check_local(
        &mut self,
        start: SourceSpan,
        binding: BindingPattern,
        anno: Option<TypeAnno>,
        expr: Option<Expression>,
    ) -> HirResult<Type> {
        match anno {
            Some(anno) => match binding {
                BindingPattern::Identifier(ident) => match self
                    .tcx
                    .annotate(ident.name, self.type_anno_into_type(anno.clone()))
                {
                    Ok(ty) => match expr {
                        Some(expr) => {
                            match self.tcx.check(&self.lower_expr_to_type_expr(&expr), &ty) {
                                Ok(ty) => Ok(ty),
                                Err(err) => Err(HirError::type_error(start, err, anno.span)),
                            }
                        }
                        None => unreachable!(),
                    },
                    Err(err) => Err(HirError::type_error(anno.span, err, anno.span)),
                },
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

    fn type_anno_into_type(&self, anno: TypeAnno) -> Type {
        match anno.kind {
            TypeKind::Unit => Type::Unit,
            TypeKind::Infer => todo!(),
            TypeKind::Named(name) => Type::var(name.normalise().to_string()),
        }
    }

    fn lower_binding_pattern(&self, ast: ast::BindingPattern) -> BindingPattern {
        match ast {
            ast::BindingPattern::Identifier(ident) => BindingPattern::Identifier(ident),
        }
    }
}
