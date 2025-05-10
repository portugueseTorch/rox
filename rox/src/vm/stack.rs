use std::{array, ptr};

use crate::chunks::value::Value;

const STACK_SIZE: usize = 4096;

pub struct Stack {
    stack: Box<[Value; STACK_SIZE]>,
    /// Pointer to the next chunk of memory in stack where the next item can be inserted
    /// If the stack is at capacity, the pointer will be pointing to invalid memory
    top: *mut Value,
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

    /// Attempts to push v onto the stack. If the stack size has been reach, push panics
    /// Internally, the pointer to the top of the stack is updated
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

    /// Pops the top-most value of the stack or None if the stack is empty
    /// Internally it iterates the pointer to the top of the stack.
    pub fn pop(&mut self) -> Option<Value> {
        // --- check if the stack is empty
        if self.top_offset() <= 0 {
            return None;
        }

        // --- move pointer back and move the item out of memory - we always want top to point to
        // the next valid position in the stack
        let value = unsafe {
            self.top = self.top.offset(-1);
            ptr::read(self.top)
        };

        Some(value)
    }

    pub fn reset(&mut self) {
        self.top = self.stack.as_mut_ptr();
    }

    pub fn trace(&self) {
        print!("  stack:\t[");
        let mut iter = self.stack.as_ptr();
        let start = self.stack.as_ptr();

        while iter < self.top {
            let is_last = unsafe { iter.offset(1) } == self.top;
            print!("{}", unsafe { &*iter });
            if !is_last {
                print!(", ")
            }

            iter = unsafe { iter.offset(1) };
        }
        print!("]\n");
    }

    fn top_offset(&mut self) -> usize {
        unsafe {
            self.top
                .offset_from(self.stack.as_mut_ptr())
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
