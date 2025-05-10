use crate::chunks::value::Value;
use crate::chunks::{opcodes::OpCode, Chunk};
use crate::{ip_advance, ptr_offset};

use super::stack::Stack;

macro_rules! trace {
    ($chunk:expr, $idx:expr) => {{
        #[cfg(feature = "trace")]
        $chunk.disassembleInstruction($idx);
    }};
}

pub struct VM {
    pub stack: Stack,
}
impl VM {
    pub fn new() -> Self {
        Self {
            stack: Stack::new(),
        }
    }

    pub fn interpret(&self, chunk: &Chunk) -> VMResult {
        let mut ip = chunk.code.as_ptr();
        let start = chunk.code.as_ptr();

        unsafe {
            while ip < start.add(chunk.code.len()) {
                trace!(chunk, ptr_offset!(start, ip));

                let op_code = *ip;
                ip_advance!(ip);

                match OpCode::try_from(op_code).unwrap() {
                    OpCode::Return => {}
                    OpCode::Constant => {
                        ip_advance!(ip);
                    }
                    OpCode::ConstantLong => {
                        ip_advance!(ip, 3);
                    }
                }
            }
        }

        VMResult::Ok
    }
}

pub enum VMResult {
    Ok,
    CompileError,
    RuntimeError,
}
