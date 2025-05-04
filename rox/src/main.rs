use chunks::{opcodes::OpCode, Chunk};

mod chunks;

fn main() {
    let mut chunk = Chunk::new();
    chunk.write(OpCode::Return);
    chunk.disassemble("Test chunk");
}
