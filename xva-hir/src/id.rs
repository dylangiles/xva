use std::ops::AddAssign;

use xva_ast::node_id::NodeId;

#[derive(Clone, Copy)]
pub struct HirId(pub i64);

impl std::fmt::Debug for HirId {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", self.0)
    }
}

impl PartialEq for HirId {
    fn eq(&self, other: &Self) -> bool {
        // self.0 == other.0
        let Self(this) = self;
        let Self(other) = other;
        this == other
    }
}

impl Eq for HirId {}

impl From<i64> for HirId {
    fn from(value: i64) -> Self {
        Self(value)
    }
}

impl From<NodeId> for HirId {
    fn from(value: NodeId) -> Self {
        let NodeId(v) = value;
        Self(v)
    }
}

impl AddAssign<i64> for HirId {
    fn add_assign(&mut self, rhs: i64) {
        self.0 += rhs
    }
}
