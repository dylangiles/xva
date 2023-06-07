use crate::value::Value;

use super::{error::MachineError, MachineResult, ValueResult};

pub(crate) struct CodeFrame {
    stack: Box<[Value]>,
    stack_pointer: usize,
}

impl CodeFrame {
    pub(crate) fn with_stack(size: usize) -> Self {
        Self {
            stack: Vec::with_capacity(size).into_boxed_slice(),
            stack_pointer: 0,
        }
    }

    pub(crate) fn push_value(&mut self, value: Value) -> MachineResult {
        if self.stack_pointer > self.stack.len() + 1 {
            Err(MachineError::StackOverflow)
        } else {
            self.stack[self.stack_pointer] = value;
            self.stack_pointer += 1;
            Ok(())
        }
    }

    pub(crate) fn pop_value(&mut self) -> ValueResult {
        if self.stack_pointer < 0 {
            Err(MachineError::StackUnderflow)
        } else {
            let value = &self.stack[self.stack_pointer];

            if self.stack_pointer != 0 {
                self.stack_pointer -= 1;
            }

            Ok(value.clone())
        }
    }

    pub(crate) fn stack_top_as_ref(&self) -> &Value {
        &self.stack[self.stack_pointer]
    }
}
