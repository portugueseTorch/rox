use std::fmt::Display;

use num_enum::{IntoPrimitive, TryFromPrimitive};

#[derive(Copy, Clone, Debug, IntoPrimitive, TryFromPrimitive)]
#[repr(u8)]
pub enum OpCode {
    Return,
    //
    Load,
    LoadLong,
    //
    Negate,
    Add,
    Subtract,
    Multiply,
    Divide,
}

impl Display for OpCode {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let display_data: &str = match self {
            OpCode::Return => "RET",
            OpCode::Load => "LOAD",
            OpCode::LoadLong => "LOAD_LONG",
            OpCode::Negate => "NEGATE",
            OpCode::Add => "ADD",
            OpCode::Subtract => "SUBTRACT",
            OpCode::Multiply => "MULTIPLY",
            OpCode::Divide => "DIVIDE",
        };
        write!(f, "{}", display_data)
    }
}
