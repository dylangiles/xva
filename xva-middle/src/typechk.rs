use xva_hir::{hir::stmt::BindingKind, ExpressionKind, Item, ItemKind, StatementKind};

use self::{context::TypeContext, error::TypeResult, expr::TypeExpr};

pub mod context;
pub mod error;
pub mod expr;

mod ty;
mod var;

// pub use crate::{
//     context::TypeContext,
//     expr::{Expression, Literal},
// };

pub fn check(hir: Vec<Item>) -> TypeResult<Vec<Item>> {
    let mut tcx = TypeContext::default();
    let mut hir = hir;

    for mut item in hir {
        match item.kind {
            ItemKind::Statement(stmt) => match stmt.kind {
                StatementKind::Local(mut local) => {
                    if let BindingKind::Inited(expr) = local.binding_kind {
                        match expr.kind {
                            ExpressionKind::Literal(lit) => {
                                local.ty = Some(tcx.synthesise(&TypeExpr::Literal(lit))?)
                            }
                            _ => todo!(),
                        }
                    }
                }
            },
            _ => todo!(),
        }
    }
    Ok(hir)
}
#[cfg(test)]
mod tests {}
