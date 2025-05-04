use super::opcodes::OpCode;

pub struct Chunk {
    code: Vec<OpCode>,
}

impl Chunk {
    pub fn new() -> Self {
        Self { code: vec![] }
    }

    pub fn write(&mut self, op: OpCode) {
        self.code.push(op)
    }

    pub fn disassemble(&self, name: &str) {
        println!("--- {} ---", name);
        self.code
            .iter()
            .enumerate()
            .for_each(|(idx, op)| println!("0x{number:0>6}: {op_code}", number = idx, op_code = op))
    }
}
