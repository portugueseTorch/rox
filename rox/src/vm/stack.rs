use crate::chunks::value::Value;

const STACK_SIZE: usize = 256;
pub struct Stack {
    pub values: [Value; STACK_SIZE],
}
