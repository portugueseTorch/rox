use crate::chunks::{opcodes::OpCode, Chunk};

pub struct VM;

impl VM {
    pub fn interpret(&self, chunk: &Chunk) -> VMResult {
        let mut ip = chunk.code.as_ptr();
        let start = chunk.code.as_ptr();

        unsafe {
            while ip < start.add(chunk.code.len()) {
                let op_code = *ip;
                ip = ip.add(1);

                match OpCode::try_from(op_code).unwrap() {
                    OpCode::Return => {}
                    OpCode::Constant => {}
                    OpCode::ConstantLong => {}
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
