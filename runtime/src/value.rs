use std::mem::size_of;

/// Must be cheap to clone.
#[derive(Clone)]
pub enum Value {
    Unit,
    Integer(i64),
    Float(f64),
    Char(char),
    Pointer(*const u8),
}

const UNIT_SIZE: usize = 0;
const BYTE_SIZE: usize = 1;
const SHORT_SIZE: usize = 2;
const INT_SIZE: usize = 4;
const LONG_SIZE: usize = 8;
const FLOAT_SIZE: usize = 4;
const DOUBLE_SIZE: usize = 8;
const CHAR_SIZE: usize = 4;

impl Value {
    pub(crate) fn discriminant_eq(&self, other: &Self) -> bool {
        core::mem::discriminant(self) == core::mem::discriminant(other)
    }
}

#[cfg(test)]
mod tests {}
