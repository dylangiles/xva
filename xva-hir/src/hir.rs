use std::sync::atomic::{AtomicI64, Ordering};
use xva_middle::TypeContext;

pub mod expr;
pub mod item;
pub mod name;
pub mod stmt;
pub mod ty;

use crate::{error::HirError, id::HirId};

pub(self) static NODE_ID_SEED: AtomicI64 = AtomicI64::new(0);
pub(super) fn next_hir_id() -> HirId {
    NODE_ID_SEED.fetch_add(1, Ordering::SeqCst).into()
}

pub type HirResult<T> = Result<T, HirError>;
pub struct HirContext {
    pub(super) tcx: TypeContext,
}

impl HirContext {
    pub fn new() -> Self {
        Self {
            tcx: TypeContext::default(),
        }
    }
}
