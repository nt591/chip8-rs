#[derive(Debug, Copy, Clone, Default)]
struct Stack {
    // limited to 16 2-byte entries
    stack: [u16; 16],
    sp: usize, // index
}

impl Stack {
    fn push(&mut self, val: u16) -> Result<(), &str> {
        //todo, encode errors better?
        if self.sp == 15 {
            // pushing would overflow
            Err("Stack Overflow")
        } else {
            self.stack[self.sp] = val;
            self.sp += 1;
            Ok(())
        }
    }
}

#[cfg(test)]
mod tests {
    use crate::stack::Stack;

    #[test]
    fn test_push_catches_stack_overflow() {
        let mut stack = Stack::default();
        for i in 0..15 {
            stack.push(i).unwrap();
        }

        assert!(
            stack.push(1).is_err(),
            "Expected error when pushing more than 16 2-byte entries"
        )
    }
}
