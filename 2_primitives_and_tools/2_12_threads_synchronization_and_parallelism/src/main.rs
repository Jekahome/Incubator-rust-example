extern crate crossbeam;
extern crate rand;
extern crate rayon;
#[macro_use]
extern crate crossbeam_channel;

use rand::thread_rng;
use rand::Rng;
use rayon::prelude::*;
use std::collections::HashMap;
use std::io::Write;
use std::sync::mpsc;
use std::thread;

/// # Parallel matrix counting.
///
/// The life cycle consists of the generation of square matrices by a single `Producer`
/// and the calculation of these matrices by two `Consumer`.
///
/// ## Examples
///
/// Basic usage:
///
/// ```rust
///
///    use threads_synchronization_and_parallelism::*;
///
///    let (tx, rx): (
///        crossbeam_channel::Sender<HashMap<(i32, i32), u8>>,
///        crossbeam_channel::Receiver<HashMap<(i32, i32), u8>>,
///    ) = crossbeam_channel::unbounded();
///
///    let rx_2 = rx.clone();
///
///    crossbeam::scope(|scope_| {
///        scope_.spawn(move || loop {
///            tx.send(Producer::generate_matrix());
///        });
///
///        scope_.spawn(move || {
///            for _i in rx {
///                Consumer::sum_matrix(_i);
///            }
///        });
///
///        scope_.spawn(move || {
///            for _i in rx_2 {
///                Consumer::sum_matrix(_i);
///            }
///        });
///
///    });
/// ```
mod threads_synchronization_and_parallelism {
    use super::*;

    /// `Producer` continuously generates square matrixes of random `u8` elements and size `4096`.
    pub struct Producer;
    /// Implement Producer.
    impl Producer {
        /// Implement generates square matrixes.
        pub fn generate_matrix() -> HashMap<(i32, i32), u8> {
            let mut matrix: HashMap<(i32, i32), u8> = HashMap::with_capacity(4096);
            let mut rng = thread_rng();
            for x in (1..65) {
                for y in (1..65) {
                    matrix.insert((x, y), rng.gen::<u8>());
                }
            }
            matrix
        }
    }

    /// `Consumer` takes generated matrix, counts sum of all its elements and prints the sum to STDOUT.
    #[derive(Debug)]
    pub struct Consumer;
    /// Implement Consumer.
    impl Consumer {
        /// Implement the calculation of the sum of a square matrix.
        /// The matrix is counted in parallel.
        pub fn sum_matrix(matrix: HashMap<(i32, i32), u8>) {
            let sum: u32 = matrix.par_iter().map(|(&k, &val)| val as u32).sum();
            writeln!(std::io::stdout(), "Matrix sum:{}", sum);
        }
    }

}

fn main() {
    use threads_synchronization_and_parallelism::*;

    let (tx, rx): (
        crossbeam_channel::Sender<HashMap<(i32, i32), u8>>,
        crossbeam_channel::Receiver<HashMap<(i32, i32), u8>>,
    ) = crossbeam_channel::unbounded();

    let rx_2 = rx.clone();

    crossbeam::scope(|scope_| {
        scope_.spawn(move || loop {
            tx.send(Producer::generate_matrix());
        });

        scope_.spawn(move || {
            for _i in rx {
                Consumer::sum_matrix(_i);
            }
        });

        scope_.spawn(move || {
            for _i in rx_2 {
                Consumer::sum_matrix(_i);
            }
        });
    });

}
