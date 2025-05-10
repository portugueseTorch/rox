use crate::chunks::value::Value;
use crate::chunks::{opcodes::OpCode, Chunk};
use crate::{bitwise, offset_ip, ptr_offset};

use super::stack::Stack;

macro_rules! trace {
    ($vm:expr, $idx:expr) => {{
        #[cfg(feature = "trace")]
        {
            $vm.chunk.disassembleInstruction($idx);
            $vm.stack.trace();
        }
    }};
}

pub struct VM {
    /// current chunk being executed
    chunk: Chunk,
    stack: Stack,
}

impl VM {
    pub fn new(chunk: Chunk) -> Self {
        Self {
            stack: Stack::new(),
            chunk: chunk.clone(),
        }
    }

    pub fn run(&mut self) -> VMResult {
        let chunk = &self.chunk;
        let mut ip = chunk.code.as_ptr();
        let start = chunk.code.as_ptr();

        unsafe {
            while ip < start.add(chunk.code.len()) {
                trace!(self, ptr_offset!(start, ip));

                let op_code = *ip;
                offset_ip!(ip);

                let op_code = OpCode::try_from(op_code).unwrap();
                match op_code {
                    OpCode::Return => {
                        let val = self.stack.pop().unwrap_or(Value::Empty);
                        println!("Returning {}", val);

                        return VMResult::Ok;
                    }
                    OpCode::Constant | OpCode::ConstantLong => {
                        let (constant, offset) = self
                            .read_constant(op_code, ip)
                            .expect("Should have a constant");
                        offset_ip!(ip, offset);

                        self.stack.push(constant.clone())
                    }
                }
            }
        }

        VMResult::Ok
    }

    #[inline]
    fn read_constant(&self, op_code: OpCode, ip: *const u8) -> anyhow::Result<(&Value, usize)> {
        let (const_idx, offset) = match op_code {
            OpCode::Constant => (unsafe { *ip as usize }, 1),
            OpCode::ConstantLong => {
                let constant_idx_as_bytes = unsafe { std::slice::from_raw_parts(ip, 3) };
                (
                    bitwise::u32_from_bytes(constant_idx_as_bytes.try_into().unwrap()) as usize,
                    3,
                )
            }
            _ => panic!("invalid op_code for read_constant: {}", op_code),
        };

        Ok((self.chunk.constants.get(const_idx).unwrap(), offset))
    }
}

pub enum VMResult {
    Ok,
    CompileError,
    RuntimeError,
}
