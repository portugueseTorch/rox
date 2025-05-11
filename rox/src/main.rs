use chunks::{opcodes::OpCode, value::Value, Chunk};
use vm::vm::VM;

mod bitwise;
mod chunks;
mod vm;

fn init_logger() {
    env_logger::builder()
        .format(|buf, record| writeln!(buf, "[{}] {}", record.level(), record.args()))
        .try_init();
}

fn main() {
    init_logger();
    let mut chunk = Chunk::new();
    for i in 0..256 {
        chunk.write_constant(Value::Number(ordered_float::OrderedFloat(i as f64)));
    }
    chunk.write_constant(Value::Number(ordered_float::OrderedFloat(1337.0)));
    chunk.write(OpCode::Return);

    let mut vm = VM::new(chunk);
    vm.run();
}
