use xva_ast::ast;

use crate::{
    hir::{
        next_hir_id,
        stmt::{BindingFlags, Local},
        HirResult,
    },
    HirContext, Item, ItemKind, Statement, StatementKind,
};

impl HirContext {
    pub fn lower_stmt(&mut self, stmt: ast::Statement) -> HirResult<Item> {
        let kind = match stmt.kind {
            ast::StatementKind::Local(local) => {
                let type_anno = if let Some(ast_anno) = &local.ty {
                    Some(self.lower_type_anno(ast_anno))
                } else {
                    None
                };

                let expr = self.maybe_lower_expr(local.expr);
                let pattern = self.lower_binding_pattern(local.pattern);
                let ty =
                    self.type_check_local(local.span, pattern, type_anno.clone(), expr.clone())?;

                StatementKind::Local(Local {
                    id: next_hir_id(),
                    span: local.span,
                    expr,
                    binding_flags: BindingFlags {
                        mutable: local.binding_flags.mutable,
                    },
                    pattern,
                    type_anno,
                    ty,
                })
            }
        };

        Ok(Item {
            id: next_hir_id(),
            kind: ItemKind::Statement(Statement {
                id: next_hir_id(),
                span: stmt.span.clone(),
                kind,
            }),
            span: stmt.span,
        })
    }
}
