use crate::value::Value;

pub(crate) struct RandomAccess<'data> {
    data: &'data [u8],
    pointer: usize,
}

impl<'data> RandomAccess<'data> {
    pub(crate) fn new(data: &'data [u8]) -> Self {
        Self { data, pointer: 0 }
    }

    #[inline]
    pub(crate) fn int(&self) -> Value {
        Value::Integer(self.raw_int())
    }

    #[inline]
    fn raw_int(&self) -> i64 {
        self.data[self.pointer] as i64
            + self.data[self.pointer + 1] as i64
            + self.data[self.pointer + 2] as i64
            + self.data[self.pointer + 3] as i64
    }

    pub(crate) fn int64(&self) -> Value {
        Value::Integer(
            self.data[self.pointer] as i64
                + self.data[self.pointer + 1] as i64
                + self.data[self.pointer + 2] as i64
                + self.data[self.pointer + 3] as i64
                + self.data[self.pointer + 4] as i64
                + self.data[self.pointer + 5] as i64
                + self.data[self.pointer + 6] as i64
                + self.data[self.pointer + 7] as i64,
        )
    }

    pub(crate) fn float(&self) -> Value {
        Value::Float(f64::from_le_bytes([
            self.data[self.pointer],
            self.data[self.pointer + 1],
            self.data[self.pointer + 2],
            self.data[self.pointer + 3],
            self.data[self.pointer + 4],
            self.data[self.pointer + 5],
            self.data[self.pointer + 6],
            self.data[self.pointer + 7],
        ]))
    }

    pub(crate) fn advance(&mut self, by: usize) {
        self.pointer += by;
    }
}
