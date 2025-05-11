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
                    OpCode::Negate => {
                        match self.stack.pop() {
                            Some(Value::Number(n)) => self.stack.push(Value::Number(-n)),
                            _ => return VMResult::RuntimeError,
                        };
                    }
                    OpCode::Add | OpCode::Subtract | OpCode::Multiply | OpCode::Divide => {
                        // --- if parsing is correct, this should never happen
                        let rhs = self.stack.pop().unwrap();
                        let lhs = self.stack.pop().unwrap();

                        let value = match op_code {
                            OpCode::Add => lhs.add(rhs),
                            OpCode::Subtract => lhs.sub(rhs),
                            OpCode::Multiply => lhs.mult(rhs),
                            OpCode::Divide => lhs.div(rhs),
                            _ => unreachable!(),
                        };

                        if let Err(e) = value {
                            log::error!("{}", e);
                            return VMResult::RuntimeError;
                        }

                        self.stack.push(value.unwrap());
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

#[derive(Debug, Eq, PartialEq, Clone, Copy)]
pub enum VMResult {
    Ok,
    CompileError,
    RuntimeError,
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn negation_without_value() {
        let mut chunk = Chunk::new();
        chunk.write(OpCode::Negate);
        let mut vm = VM::new(chunk);
        assert_eq!(vm.run(), VMResult::RuntimeError);
    }

    #[test]
    fn negation_with_value() {
        let mut chunk = Chunk::new();
        chunk.write_constant(Value::Number(ordered_float::OrderedFloat(42.0)));
        chunk.write(OpCode::Negate);
        chunk.write(OpCode::Return);

        let mut vm = VM::new(chunk);
        assert_eq!(vm.run(), VMResult::Ok);
    }

    #[test]
    fn arithmetic_ops() {
        let mut chunk = Chunk::new();
        chunk.write_constant(Value::Number(ordered_float::OrderedFloat(1.0)));
        chunk.write_constant(Value::Number(ordered_float::OrderedFloat(2.0)));
        chunk.write(OpCode::Add);

        let mut vm = VM::new(chunk);
    }
}
