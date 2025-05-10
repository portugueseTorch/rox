use std::{array, ptr};

use crate::chunks::value::Value;

const STACK_SIZE: usize = 10;

pub struct Stack {
    pub stack: Box<[Value; STACK_SIZE]>,
    pub top: *mut Value,
}

impl Stack {
    pub fn new() -> Self {
        let mut stack = Box::new(array::from_fn(|_| Value::default()));
        let top = stack.as_mut_ptr();

        Self { stack, top }
    }

    pub fn len(&mut self) -> usize {
        self.top_offset()
    }

    pub fn push(&mut self, v: Value) {
        // --- assert that there is still enough space in the stack
        assert!(
            self.top_offset() < STACK_SIZE,
            "Stack overflow: maximum stack size of {} reached",
            STACK_SIZE
        );

        // --- write value onto the stack
        unsafe {
            *self.top = v;
            self.top = self.top.offset(1);
        }
    }

    pub fn pop(&mut self) -> Option<Value> {
        // --- check if the stack is empty
        if self.top_offset() <= 0 {
            return None;
        }

        let value = unsafe {
            let top = self.top.offset(-1);
            ptr::read(top)
        };

        unsafe { self.top = self.top.offset(-1) };

        Some(value)
    }

    pub fn reset(&mut self) {
        self.top = self.stack.as_mut_ptr();
    }

    fn top_offset(&mut self) -> usize {
        unsafe {
            dbg!(self.top.offset_from(self.stack.as_mut_ptr()))
                .try_into()
                .unwrap()
        }
    }
}

#[cfg(test)]
mod tests {
    use ordered_float::OrderedFloat;

    use super::*;

    #[test]
    fn push() {
        let mut stack = Stack::new();
        assert_eq!(stack.len(), 0);

        stack.push(Value::Number(OrderedFloat(42.0)));
        assert_eq!(stack.len(), 1);

        stack.push(Value::Literal("Hello, world!"));
        assert_eq!(stack.len(), 2);
    }

    #[test]
    fn pop() {
        let mut stack = Stack::new();
        assert_eq!(stack.len(), 0);

        stack.push(Value::Number(OrderedFloat(42.0)));
        stack.push(Value::Literal("Hello, world!"));

        assert_eq!(stack.pop().unwrap(), Value::Literal("Hello, world!"));
        assert_eq!(stack.pop().unwrap(), Value::Number(OrderedFloat(42.0)));
        assert_eq!(stack.pop(), None);
    }

    #[test]
    fn reset() {
        let mut stack = Stack::new();

        stack.push(Value::Number(OrderedFloat(42.0)));
        stack.push(Value::Literal("Hello, world!"));
        stack.push(Value::Number(OrderedFloat(42.0)));
        stack.push(Value::Literal("Hello, world!"));
        assert_eq!(stack.len(), 4);

        stack.reset();
        assert_eq!(stack.len(), 0);
    }
}
