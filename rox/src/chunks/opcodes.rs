use std::fmt::Display;

use num_enum::{IntoPrimitive, TryFromPrimitive};

#[derive(Copy, Clone, Debug, IntoPrimitive, TryFromPrimitive)]
#[repr(u8)]
pub enum OpCode {
    Return,
    //
    Constant,
    ConstantLong,
    //
    Negate,
}

impl Display for OpCode {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let display_data: &str = match self {
            OpCode::Return => "RET",
            OpCode::Constant => "CONST",
            OpCode::ConstantLong => "CONST_LONG",
            OpCode::Negate => "NEGATE",
        };
        write!(f, "{}", display_data)
    }
}
