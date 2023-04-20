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

    fn pop(&mut self) -> Result<u16, &str> {
        // todo: encode errors as types?
        if self.sp == 0 {
            Err("Empty stack")
        } else {
            let tmp = self.stack[self.sp - 1];
            self.sp -= 1;
            Ok(tmp)
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

    #[test]
    fn test_pop_should_err_if_empty() {
        let mut stack = Stack::default();
        assert!(
            stack.pop().is_err(),
            "Expected error when popping empty stack"
        )
    }

    #[test]
    fn test_pop() {
        let mut stack = Stack::default();
        stack.push(1).unwrap();
        assert_eq!(stack.pop(), Ok(1))
    }
}
