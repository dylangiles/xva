use xva_ast::ast;

use crate::{hir::next_hir_id, Expression, ExpressionKind, HirContext};

impl HirContext {
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

    pub(super) fn maybe_lower_expr(&self, ast: Option<ast::Expression>) -> Option<Expression> {
        ast.map(|expr| self.lower_expr(expr))
    }
}
