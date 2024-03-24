pub mod context;
pub mod error;
pub mod expr;

pub mod ty;
pub mod var;

// pub use crate::{
//     context::TypeContext,
//     expr::{Expression, Literal},
// };

// pub fn check(hir: Vec<Item>) -> TypeResult<Vec<Item>> {
//     let mut tcx = TypeContext::default();

//     let mir = hir
//         .into_iter()
//         .map(|mut item| match item.kind {
//             ItemKind::Statement(stmt) => match stmt.kind {
//                 StatementKind::Local(mut local) => {
//                     if let BindingKind::Inited(expr) = local.binding_kind {
//                         match expr.kind {
//                             ExpressionKind::Literal(lit) => {
//                                 local.type_anno = match tcx.synthesise(&TypeExpr::Literal(lit)) {
//                                     Ok(ty) => Some(hir_ty_into_ty(ty)),
//                                     Err(e) => panic!("{e}"),
//                                 }
//                             }
//                         }
//                     }
//                 }
//             },
//         })
//         .collect();

//     Ok(mir)
//     // for item in hir {
//     //     match item.kind {
//     //         ItemKind::Statement(stmt) => match stmt.kind {
//     //             StatementKind::Local(mut local) => {
//     //                 if let BindingKind::Inited(expr) = local.binding_kind {
//     //                     match expr.kind {
//     //                         ExpressionKind::Literal(lit) => local.ty = None,
//     //                         // _ => todo!(),
//     //                     }
//     //                 }
//     //             }
//     //         },
//     //         // _ => todo!(),
//     //     }
//     // }
//     // Ok(hir)
// }

// pub(crate) fn ty_into_hir_ty(ty: Type) -> hir::ty::Type {}
// pub(crate) fn hir_ty_into_ty(hir_ty: hir::ty::Type) -> Type {
//     match hir_ty.kind {
//         hir::ty::TypeKind::Unit => Type::Unit,
//         hir::ty::TypeKind::Infer => todo!(),
//         hir::ty::TypeKind::Named(name) => Type::Variable(Variable(name.normalise())),
//     }
// }
#[cfg(test)]
mod tests {}
