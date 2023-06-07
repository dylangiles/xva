use self::{error::MachineError, frame::CodeFrame, opcode::Opcode, rax::RandomAccess};
use crate::value::Value;
use core::mem::size_of;

mod error;
mod frame;
mod opcode;
mod rax;

pub(crate) type MachineResult = Result<(), MachineError>;
pub(crate) type ValueResult = Result<Value, MachineError>;

pub(crate) struct Machine<'machine> {
    instructions: &'machine [u8],
    inst_rax: RandomAccess<'machine>,
    frames: Vec<CodeFrame>,
    current_frame: usize,
    inst_pointer: usize,
}

impl<'machine> Machine<'machine> {
    pub(crate) fn new(instructions: &'machine [u8]) -> Self {
        Self {
            inst_rax: RandomAccess::new(instructions),
            instructions,
            frames: vec![],
            current_frame: 0,
            inst_pointer: 0,
        }
    }

    #[inline]
    pub(crate) fn new_frame(&mut self, stack: usize) {
        self.frames.push(CodeFrame::with_stack(stack));
        self.current_frame += 1;
    }

    #[inline]
    pub(crate) fn interpret(&mut self) -> MachineResult {
        self.new_frame(8); // TODO

        for inst in self.instructions {
            let op = match Opcode::try_from(inst) {
                Ok(o) => o,
                Err(e) => return Err(MachineError::Opcode(e)),
            };

            match op {
                Opcode::Nop => continue,
                Opcode::Breakpoint => todo!(),
                Opcode::LoadIntMinusOne => self.push_value(Value::Integer(-1))?,
                Opcode::LoadIntZero => self.push_value(Value::Integer(0))?,
                Opcode::LoadIntOne => self.push_value(Value::Integer(1))?,
                Opcode::LoadInt => {
                    let val = self.next_int32();
                    self.push_value(val)?;
                }
                Opcode::LoadFloat => {
                    let val = self.next_float64();
                    self.push_value(val)?;
                }
                Opcode::Duplicate => match self.stack_top_as_ref() {
                    Value::Unit => self.push_value(Value::Unit)?,
                    v => self.push_value(v.clone())?,
                },
                Opcode::IntAdd => {
                    let (right, left) = (self.pop_value()?, self.pop_value()?);

                    if let Value::Integer(l) = left {
                        if let Value::Integer(r) = right {
                            let result = l + r;
                            self.push_value(Value::Integer(result))?;
                        } else {
                            return Err(MachineError::TypeError(
                                "Integer arithmetic attempted on non-integer type".into(),
                            ));
                        }
                    } else {
                        return Err(MachineError::TypeError(
                            "Integer arithmetic attempted on non-integer type".into(),
                        ));
                    }
                }
                Opcode::IntSubtract => todo!(),
                Opcode::IntMultiply => todo!(),
                Opcode::IntDivide => todo!(),
                Opcode::IntModulo => todo!(),
                Opcode::FloatAdd => todo!(),
                Opcode::FloatSubtract => todo!(),
                Opcode::FloatMultiply => todo!(),
                Opcode::FloatDivide => todo!(),
                Opcode::FloatModulo => todo!(),
            }
        }

        Ok(())
    }

    pub(crate) fn push_value(&mut self, value: Value) -> MachineResult {
        let frame = &mut self.frames[self.current_frame];
        frame.push_value(value)
    }

    pub(crate) fn pop_value(&mut self) -> Result<Value, MachineError> {
        (self.frames[self.current_frame]).pop_value()
    }

    fn pop_two(&mut self) -> Result<(Value, Value), MachineError> {
        Ok((self.pop_value()?, self.pop_value()?))
    }

    fn unwrap_two_as_int(left: ) -> Result<(i64, i64), MachineError> {

    }

    fn next_int32(&mut self) -> Value {
        let val = self.inst_rax.int();
        self.inst_rax.advance(size_of::<i32>());
        val
    }

    fn next_int64(&mut self) -> Value {
        let val = self.inst_rax.int64();
        self.inst_rax.advance(size_of::<i64>());
        val
    }

    fn next_float32(&mut self) -> Value {
        let val = self.inst_rax.float32();
        self.inst_rax.advance(size_of::<f32>());
        val
    }

    fn next_float64(&mut self) -> Value {
        let val = self.inst_rax.float64();
        self.inst_rax.advance(size_of::<f64>());
        val
    }

    fn stack_top_as_ref(&self) -> &Value {
        self.frames[self.current_frame].stack_top_as_ref()
    }
}
