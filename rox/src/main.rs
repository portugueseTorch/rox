use chunks::{opcodes::OpCode, value::Value, Chunk};
use std::io::Write;
use vm::vm::VM;

mod bitwise;
mod chunks;
mod vm;

#[allow(unused_must_use)]
fn init_logger() {
    env_logger::builder()
        .format(|buf, record| writeln!(buf, "[{}] {}", record.level(), record.args()))
        .try_init();
}

fn main() {
    init_logger();
    let mut chunk = Chunk::new();
    for i in 1..3 {
        chunk.write_constant(Value::Number(ordered_float::OrderedFloat(i as f64)));
    }
    chunk.write(OpCode::Add);

    let mut vm = VM::new(chunk);
    vm.run();
}
