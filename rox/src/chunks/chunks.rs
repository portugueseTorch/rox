use super::{opcodes::OpCode, value::Value};

pub struct Chunk {
    /// bytecode instruction - defined as a general byte array to allow instructions to have
    /// operands (e.g., constants). Although more laborious this approach is prefered over having
    /// OpCodes hold inner values in favor of less memory expenditure
    code: Vec<u8>,
    constants: Vec<Value>,
    line_info: Vec<LineInfo>,
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

    pub fn write_constant(&mut self, value: Value) {
        // --- write value to the constants pool
        let idx = self.write_constant_aux(value);
        self.write(OpCode::Constant);
        self.write(idx);
    }

    // TODO: having this return a u8 means that the amount of constants we can store is inherently
    // capped at 256. Probably fine for now, but can always be extended in the future
    fn write_constant_aux(&mut self, value: Value) -> u8 {
        self.constants.push(value);
        (self.constants.len() - 1) as u8
    }

    pub fn disassemble(&self, name: &str) {
        println!("--- {} ---", name);
        let mut i = 0;
        let mut op_number = 0;
        while i < self.code.len() {
            op_number += 1;
            let raw_byte = self.code[i];
            let op: OpCode = raw_byte.try_into().expect("invalid opcode");
            i += 1;

            let op_data: Option<String> = match op {
                OpCode::Return => None,
                OpCode::Constant => {
                    let operand_index = self.code.get(i).expect("missing operand for constant");
                    i += 1;
                    let operand = self
                        .constants
                        .get(*operand_index as usize)
                        .expect("invalid idx for constant data");
                    Some(operand.to_string())
                }
            };

            println!(
                "0x{:0>6}: {}{}",
                op_number,
                op.to_string(),
                op_data.map_or(String::new(), |s| format!("({})", s))
            );
        }
    }

    fn get_line_info_from_offset(&self, offset: usize) -> Option<&LineInfo> {
        let mut low = 0;
        let mut high = self.code.len();

        while low < high {
            let mid = low + (high - low) / 2;
            let item_at_mid = self.line_info.get(mid).expect("m should be a valid index");

            if item_at_mid.op_offset > offset {
                high = mid;
            } else {
                low = mid + 1;
            }
        }

        if low == 0 {
            None
        } else {
            self.line_info.get(low - 1)
        }
    }
}

#[derive(Debug, Clone, Copy)]
struct LineInfo {
    /// offset into Chunk::code
    op_offset: usize,
    /// line number of the operation at op_offset
    line: usize,
}
