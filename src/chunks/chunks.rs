use crate::{bitwise, ptr_offset};

use super::{opcodes::OpCode, value::Value};

/// Advances ip to the next instruction to process
#[macro_export]
macro_rules! offset_ip {
    ($ip:expr) => {
        $ip = $ip.add(1);
    };
    ($ip:expr, $offset:expr) => {
        $ip = $ip.add($offset)
    };
}

#[derive(Debug, Clone)]
pub struct Chunk {
    /// bytecode instruction - defined as a general byte array to allow instructions to have
    /// operands (e.g., constants). Although more laborious this approach is prefered over having
    /// OpCodes hold inner values in favor of less memory expenditure and cache locality
    pub code: Vec<u8>,
    pub constants: Vec<Value>,
    pub line_info: Vec<LineInfo>,
}

impl Chunk {
    pub fn new() -> Self {
        Self {
            code: vec![],
            constants: vec![],
            line_info: vec![LineInfo {
                op_offset: 0,
                line: 1,
            }],
        }
    }

    pub fn new_line(&mut self, offset: usize) {
        let current_line = self.line_info.last().map_or(0, |l| l.line);
        self.line_info.push(LineInfo {
            line: current_line + 1,
            op_offset: offset,
        })
    }

    pub fn write<T>(&mut self, byte: T)
    where
        T: Into<u8>,
    {
        self.code.push(byte.into())
    }

    fn write_24b(&mut self, val: u32) {
        let (b4, b3, b2, b1) = bitwise::get_bytes(val);
        assert_eq!(
            b4, 0,
            "attempting to write a value of more than 24bits into constants"
        );

        self.write(b3);
        self.write(b2);
        self.write(b1);
    }

    pub fn write_constant(&mut self, value: Value) {
        // --- write value to the constants pool
        let idx = self.write_constant_aux(value);

        // --- if the returned index is lower than 256, write Constant instruction
        // otherwise we need to write a ConstantLong and store the index as a 32-bit number
        match u8::try_from(idx) {
            Ok(idx_as_u8) => {
                self.write(OpCode::Load);
                self.write(idx_as_u8);
            }
            Err(_) => {
                self.write(OpCode::LoadLong);
                self.write_24b(idx);
            }
        }
    }

    /// pushes value into constant and returns the index into which it was pushed
    fn write_constant_aux(&mut self, value: Value) -> u32 {
        self.constants.push(value);
        (self.constants.len() - 1) as u32
    }

    /// self contained disassembler for a chunk - it is pure and can be used to log the generated
    /// bytecode of the current chunk
    pub fn disassemble(&self, name: &str) {
        log::debug!("------ {} ------", name);
        log::debug!("offset    line\top");
        let mut i = 0;

        while i < self.code.len() {
            i = self.disassembleInstruction(i);
        }
    }

    /// self contained instruction disassembler - it is pure and returns the index of the next operation to be
    /// executed.
    pub fn disassembleInstruction(&self, mut idx: usize) -> usize {
        let raw_byte = self.code.get(idx).unwrap();
        let op = OpCode::try_from(*raw_byte).unwrap();
        let op_idx = idx;
        let line_info = self.get_line_info_from_offset(idx);
        idx += 1;

        let op_data: Option<String> = match op {
            OpCode::Load => {
                let operand_idx = self.code.get(idx).unwrap();
                idx += 1;
                let operand = self
                    .constants
                    .get(*operand_idx as usize)
                    .expect("invalid idx for constant data");
                Some(operand.to_string())
            }
            OpCode::LoadLong => {
                // --- the index of the operand will be the next 24 bits
                let idx_as_bytes = self
                    .code
                    .get(idx..=idx + 2)
                    .expect("missing constant index for long constant");
                let operand_idx = bitwise::u32_from_bytes(
                    idx_as_bytes
                        .try_into()
                        .expect("should be an array of 3 bytes"),
                );
                let operand = self
                    .constants
                    .get(operand_idx as usize)
                    .expect("invalid idx for long constant data");

                Some(operand.to_string())
            }
            OpCode::Return
            | OpCode::Negate
            | OpCode::Add
            | OpCode::Subtract
            | OpCode::Multiply
            | OpCode::Divide => None,
        };

        log::debug!(
            "0x{:0>6} {:>5}\t{}{}",
            op_idx,
            line_info.line,
            op.to_string(),
            op_data.map_or(String::new(), |s| format!(" ({})", s))
        );

        idx
    }

    fn get_line_info_from_offset(&self, offset: usize) -> &LineInfo {
        let mut low = 0;
        let mut high = self.line_info.len();

        while low < high {
            let mid = low + (high - low) / 2;
            if self.line_info[mid].op_offset <= offset {
                low = mid + 1;
            } else {
                high = mid
            }
        }

        self.line_info
            .get(low - 1)
            .expect("should always provide a valid line")
    }
}

#[derive(Debug, Clone, Copy)]
struct LineInfo {
    /// offset into Chunk::code
    op_offset: usize,
    /// line number of the operation at op_offset
    line: usize,
}
