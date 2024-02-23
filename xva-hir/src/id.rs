use std::ops::AddAssign;

#[derive(Clone, Copy)]
pub struct HirId(pub u64);

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

impl From<u64> for HirId {
    fn from(value: u64) -> Self {
        Self(value)
    }
}

impl AddAssign<u64> for HirId {
    fn add_assign(&mut self, rhs: u64) {
        self.0 += rhs
    }
}
