use std::error::Error;

use super::error::OpcodeError;

// const OPCODE_NOP: u8 = 0;
// const OPCODE_LOAD_INT_MINUS_ONE: u8 = 0x1;
// const OPCODE_LOAD_INT_ZERO: u8 = 0x2;
// const OPCODE_LOAD_INT_ONE: u8 = 0x3;
// const OPCODE_LOAD_INT: u8 = 0x4;
// const OPCODE_LOAD_FLOAT: u8 = 0x5;
// const OPCODE_DUPLICATE: u8 = 0x6;
// const OPCODE_INT_ADD: u8 = 0x7;
// const OPCODE_INT_SUBTRACT: u8 = 0x8;
// const OPCODE_INT_MULTIPLY: u8 = 0x9;
// const OPCODE_INT_DIVIDE: u8 = 0xA;
// const OPCODE_INT_MODULO: u8 = 0xB;
// const OPCODE_FLOAT_ADD: u8 = 0xC;
// const OPCODE_FLOAT_SUBTRACT: u8 = 0xD;
// const OPCODE_FLOAT_MULTIPLY: u8 = 0xE;
// const OPCODE_FLOAT_DIVIDE: u8 = 0xF;
// const OPCODE_FLOAT_MODULO: u8 = 0x10;

#[repr(u8)]
pub(crate) enum Opcode {
    Nop,
    Breakpoint,
    LoadIntMinusOne,
    LoadIntZero,
    LoadIntOne,
    LoadInt,
    LoadFloat,
    Duplicate,
    IntAdd,
    IntSubtract,
    IntMultiply,
    IntDivide,
    IntModulo,
    FloatAdd,
    FloatSubtract,
    FloatMultiply,
    FloatDivide,
    FloatModulo,
}

impl std::convert::TryFrom<u8> for Opcode {
    type Error = OpcodeError;

    fn try_from(value: u8) -> Result<Self, Self::Error> {
        match value {
            OPCODE_NOP => Ok(Self::Nop),
            OPCODE_LOAD_INT_MINUS_ONE => Ok(Self::LoadIntMinusOne),
            x => Err(OpcodeError(x)),
        }
    }
}

impl std::convert::TryFrom<&u8> for Opcode {
    type Error = OpcodeError;

    fn try_from(value: &u8) -> Result<Self, Self::Error> {
        Opcode::try_from(*value)
    }
}
