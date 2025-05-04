use chunks::{opcodes::OpCode, value::Value, Chunk};

mod chunks;

fn main() {
    let mut chunk = Chunk::new();
    chunk.write_constant(Value::Number(42.0));
    chunk.write_constant(Value::Number(1337.0));
    chunk.write_constant(Value::Number(12.0));
    chunk.write(OpCode::Return);
    chunk.disassemble("Test chunk");
}
