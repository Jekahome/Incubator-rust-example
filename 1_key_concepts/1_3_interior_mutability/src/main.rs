#![allow(dead_code)]

use interior_mutability::Stack;
use std::clone::Clone;
use std::io::{Error, ErrorKind};
use std::result::Result;

mod interior_mutability {

    use super::*;
    const N: usize = 5;
    #[derive(Debug)]
    pub struct Stack<T> {
        maxsize: usize,
        top: usize,
        pub items: [T; N],
    }

    impl<T> Stack<T> {
        pub fn new(value: T) -> Self
        where
            T: Clone + Copy,
        {
            Stack {
                items: [value; N],
                top: 0usize,
                maxsize: N,
            }
        }

        pub fn push(&mut self, i: T) -> Result<bool, Error> {
            if self.top >= self.maxsize {
                Err(Error::new(ErrorKind::Other, "Full stack"))
            } else {
                self.items[self.top] = i;
                self.top += 1;
                Ok(true)
            }
        }

        pub fn pop(&mut self) -> Result<T, Error>
        where
            T: Clone,
        {
            if self.top == 0 {
                Err(Error::new(ErrorKind::Other, "Empty stack"))
            } else {
                self.top -= 1;
                Ok(self.items[self.top].clone())
            }
        }
    }

    #[cfg(test)]
    pub mod test {
        use super::*;

        #[test]
        fn test() {
            let value_type = 0i32;
            let stack: Stack<i32> = <Stack<i32>>::new(0i32);
            let _stack = std::cell::RefCell::new(stack);

            let stack_clone_1 = &_stack;
            let stack_clone_2 = &_stack;

            // verification empty stack
            let result_pop = stack_clone_1.borrow_mut().pop();

            if let Ok(_) = result_pop {
                assert!(false);
            } else {
                assert!(true);
            }

            // verification full stack
            for _i in value_type as usize..N {
                assert_eq!(true, stack_clone_1.borrow_mut().push(10).unwrap_or(false));
            }

            if let Ok(_) = stack_clone_2.borrow_mut().push(3) {
                assert!(false);
            } else {
                assert!(true);
            }

            // verification last value
            stack_clone_1.borrow_mut().pop();
            if let Ok(_) = stack_clone_1.borrow_mut().push(33) {
                assert!(true);
            } else {
                assert!(false);
            }

            let result_pop = stack_clone_2.borrow_mut().pop();

            if let Ok(result) = result_pop {
                assert_eq!(33, result);
            } else {
                assert!(false);
            }
        }
    }

}

fn main() {
    let stack: Stack<&str> = <Stack<&str>>::new("");
    let _stack = std::cell::RefCell::new(stack);

    let stack_clone_1 = &_stack;

    let stack_clone_2 = &_stack;

    stack_clone_2.borrow_mut().push("1").is_ok();

    stack_clone_1.borrow_mut().pop().is_ok();
    stack_clone_1.borrow_mut().push("2").is_ok();

    stack_clone_1.borrow_mut().push("2").is_ok();
    stack_clone_2.borrow_mut().push("3").is_ok();
    stack_clone_2.borrow_mut().pop().is_ok();
}
