extern crate crossbeam;

use std::cell::{Cell, RefCell};
use std::sync::Mutex;
use std::sync::{Arc, RwLock};
use std::thread;
use std::borrow::BorrowMut;

/// An example implementation of the type notsync.
/// The std::cell::Cell type does not provide data security when shared across
/// multiple threads, because it allows mutation of its contents even through a
/// permanent shared reference.
mod notsync {
    use super::*;

    /// Struct with Cell data.
    #[derive(Debug, Clone)]
    pub struct Point {
        x: Cell<i32>,
        y: Cell<i32>,
    }

    /// Point methods.
    impl Point {
        /// New Point object.
        pub fn new(x: Cell<i32>, y: Cell<i32>) -> Point {
            Point { x: x, y: y }
        }
        /// Set method for value x.
        pub fn set_x(&mut self, x: i32) {
            self.x.set(x);
        }
        /// Set method for value y.
        pub fn set_y(&mut self, y: i32) {
            self.y.set(y);
        }
        /// Return value point x.
        pub fn get_x(&self) -> i32 {
            self.x.get()
        }
    }

    #[cfg(test)]
    mod test {

        use notsync::*;
        #[test]
        fn test() {
            let mut point: Point = Point::new(Cell::new(3), Cell::new(3));
            {
                let mut ref_point: &mut Point = &mut point;
                //let point_clone:Point_send = point.clone();
                crossbeam::scope(|scope_| {
                    scope_
                        .spawn(move || {
                            //ref_point.x.set(0);
                            //point_clone.x.set(0);
                            ref_point.set_x(0);
                            // println!("point_clone={:#?}",point_clone);
                        })
                        .join();
                });
            }
            thread::sleep_ms(50);
            // println!("point={:#?}",point);
            assert_eq!(point.get_x(), 0);
        }
    }
}

/// An example implementation of type SyncAndSend.
/// For which it is safe to move the value to a stream and exchange a reference to the data.
mod sync_and_send {
    use super::*;

    /// Struct Point data.
    #[derive(Debug, Clone)]
    pub struct Point {
        pub x: i32,
        pub y: i32,
    }

    /// Point methods.
    impl Point {
        /// New Point object.
        pub fn new(x: i32, y: i32) -> Arc<Mutex<Point>> {
            Arc::new(Mutex::new(Point { x: x, y: y }))
        }
    }

    #[cfg(test)]
    mod test {
        #[test]
        fn test() {
            use sync_and_send::*;

            let mut point: Arc<Mutex<Point>> = Point::new(3, 3);
            let clone_point = Arc::clone(&point);

            crossbeam::scope(|scope_| {
                scope_
                    .spawn(move || {
                        let mut mut_point = clone_point.lock();
                        match mut_point {
                            Ok(mut _mut_point) => {
                                _mut_point.x = 0;
                            }
                            Err(e) => {}
                        }
                    })
                    .join();
            });
            thread::sleep_ms(50);

            assert_eq!(point.lock().unwrap().x, 0);
        }
    }

}

/// An example of an implementation of the type onlysync.
/// Not safe operation with raw pointers does not implement
/// the Send and Sync traits by default.
pub mod only_sync {
    use super::*;
    #[derive(Debug)]
    pub struct OnlySync {
        pub field: *mut i32,
    }
    /// Implements Sync trait.
    unsafe impl Sync for OnlySync {}

    /// OnlySync methods.
    impl OnlySync {
        /// New OnlySync object.
        pub fn new() -> Arc<Mutex<OnlySync>> {
            Arc::new(Mutex::new(OnlySync { field: &mut 1 }))
        }
    }
    /// Implements Drop trait.
    impl Drop for OnlySync {
        fn drop(&mut self) {}
    }

}

fn main() {
    use only_sync::*;

    let mut onlySync: Arc<Mutex<OnlySync>> = OnlySync::new();

    use notsync::{self, Point as Point_send};
    let mut point: Point_send = Point_send::new(Cell::new(3), Cell::new(3));
    {
        let mut ref_point: &mut Point_send = &mut point;

        crossbeam::scope(|scope_| {
            scope_
                .spawn(move || {
                    //ref_point.x.set(0);
                    //point_clone.x.set(0);
                    ref_point.set_x(0);
                    // println!("point_clone={:#?}",point_clone);
                })
                .join();
        });
    }
    thread::sleep_ms(50);

    assert_eq!(point.get_x(), 0);
}
