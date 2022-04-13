/// # The simple `Queue<T>` collection of fixed size `n`.

/// `Queue<T>` collection  can be used both as
/// a single type collection (without performance penalty for dynamic dispatch)
/// and as a heterogeneous collection (which can hold dierent concrete types).
///
/// ## Examples
///
/// Basic usage:
///
/// ```rust
///  use queue::*;
///
///  let arr: [i32; SIZE_ARRAY] = [Default::default(); SIZE_ARRAY];
///  let mut buffer: Queue<i32> = Queue::new(arr);
///
///  assert!(buffer.push(4));
///
///  if let Some(var) = buffer.pop() {
///     assert_eq!(4, var);
///  } else {
///     assert!(false);
///  }
/// ```
mod queue {

    /// The collection Queue works with an array and uses a constant for a fixed size.
    pub const SIZE_ARRAY: usize = 5;

    /// The simple `Queue<T>` collection of fixed size `n`.
    #[derive(Debug)]
    pub struct Queue<T> {
        pub value: [T; SIZE_ARRAY],
        index: usize,
    }

    /// The work methods are based on the principle of "first entered first came out".
    impl<T> Queue<T> {
        /// Adds items to the end of the queue with a pointer pointer to the next cell.
        /// In case of success, returns `true`, in case of failure `false`.
        ///
        /// ## Examples
        ///
        /// Basic usage:
        ///
        /// ```rust
        ///  let arr: [i32; SIZE_ARRAY] = [Default::default(); SIZE_ARRAY];
        ///  let mut buffer: Queue<i32> = Queue::new(arr);
        ///
        ///  assert!(buffer.push(4));
        /// ```
        pub fn push(&mut self, value: T) -> bool {
            if self.index < SIZE_ARRAY - 1 {
                self.value[self.index] = value;
                self.index += 1;
                return true;
            }
            return false;
        }

        /// Returns an element from the beginning of the queue.
        /// Moves the index back to the position.
        ///
        /// ## Examples
        ///
        /// Basic usage:
        ///
        /// ```rust
        ///  let arr: [i32; SIZE_ARRAY] = [Default::default(); SIZE_ARRAY];
        ///  let mut buffer: Queue<i32> = Queue::new(arr);
        ///
        ///  assert!(buffer.push(4));
        ///
        ///  if let Some(var) = buffer.pop() {
        ///     assert_eq!(4, var);
        ///  } else {
        ///     assert!(false);
        ///  }
        /// ```
        pub fn pop(&mut self) -> Option<T>
        where
            T: Clone,
        {
            if self.index > 0 {
                self.index -= 1;
                return Some(self.value[self.index].clone());
            }
            return None;
        }

        /// Creates new `Queue<T>`.
        /// The index of the array begins by default for the type usize with 0.
        pub fn new(value: [T; SIZE_ARRAY]) -> Self {
            Queue {
                value: value,
                index: Default::default(),
            }
        }
    }

    #[cfg(test)]
    mod test {

        use super::*;
        use std::fmt::Debug;
        trait Base: Debug {}

        impl Base for i32 {}
        impl Base for bool {}

        #[derive(Debug)]
        struct Item {
            data: i32,
        }
        impl Base for Item {}

        #[test]
        fn test_queue() {
            // Test dynamic dispatch
            let arr: [&Base; SIZE_ARRAY] = [&false; SIZE_ARRAY];
            let mut buffer: Queue<&Base> = Queue::new(arr);

            buffer.push(&true);
            buffer.push(&Item { data: 4 });
            buffer.push(&5i32);

            if let Some(_var) = buffer.pop() {
                assert!(true);
            } else {
                assert!(false);
            }

            // Test static dispatch
            let arr: [i32; SIZE_ARRAY] = [0i32; SIZE_ARRAY];
            let mut buffer: Queue<i32> = Queue::new(arr);

            buffer.push(4);
            buffer.push(5);
            if let Some(var) = buffer.pop() {
                assert_eq!(5, var);
            } else {
                assert!(false);
            }
        }

    }

}

fn main() {
    use queue::*;

    // Example static dispatch

    let arr: [i32; SIZE_ARRAY] = [Default::default(); SIZE_ARRAY];
    let mut buffer: Queue<i32> = Queue::new(arr);
    buffer.push(4);
    buffer.push(5);
    if let Some(var) = buffer.pop() {
        assert_eq!(5, var);
    } else {
        assert!(false);
    }
}
