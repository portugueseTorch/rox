use crate::chunks::{opcodes::OpCode, Chunk};
use crate::{ip_advance, ptr_offset};

pub struct VM;

impl VM {
    pub fn new() -> Self {
        Self
    }

    pub fn interpret(&self, chunk: &Chunk) -> VMResult {
        let mut ip = chunk.code.as_ptr();
        let start = chunk.code.as_ptr();

        unsafe {
            while ip < start.add(chunk.code.len()) {
                let op_code = *ip;
                let idx = ptr_offset!(start, ip);
                ip_advance!(ip);

                match OpCode::try_from(op_code).unwrap() {
                    OpCode::Return => {
                        chunk.disassembleInstruction(idx);
                    }
                    OpCode::Constant => {
                        chunk.disassembleInstruction(idx);
                        ip_advance!(ip);
                    }
                    OpCode::ConstantLong => {
                        chunk.disassembleInstruction(idx);
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
