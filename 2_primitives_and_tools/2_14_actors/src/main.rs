extern crate actix;
extern crate futures;
extern crate tokio;
extern crate rand;
extern crate rayon;

use actix::prelude::*;
use std::collections::HashMap;
use rand::thread_rng;
use rand::Rng;
use rayon::prelude::*;
use std::io::Write;
use std::time::Duration;

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
///    fn main() {
///    use actor_matrix::*;
///
///    System::run(|| {
///            let addr_1: actix::Addr<Consumer> = Consumer.start();
///            let addr_2: actix::Addr<Consumer> = addr_1.clone();
///            Producer {
///                subscribers: vec![addr_1.recipient(), addr_2.recipient()],
///            }.start();
///        });
///    }
/// ```
mod actor_matrix {

    use super::*;

    /// Message type for subscribers.
    #[derive(Message)]
    pub struct Signal(HashMap<(i32, i32), u8>);

    /// Actor `Consumer`.
    /// `Consumer` takes generated matrix, counts sum of all its elements and prints the sum to STDOUT.
    pub struct Consumer;
    /// Implement Consumer.
    impl Actor for Consumer {
        type Context = Context<Self>;
    }
    /// Receiving and processing messages like `Signal`.
    impl Handler<Signal> for Consumer {
        type Result = ();
        /// Implement the calculation of the sum of a square matrix.
        /// The matrix is counted in parallel.
        fn handle(&mut self, msg: Signal, _: &mut Self::Context) {
            let sum: u32 = msg.0.par_iter().map(|(&_k, &val)| val as u32).sum();
            writeln!(std::io::stdout(), "Matrix sum:{}", sum);
        }
    }

    /// Actor `Producer` continuously generates square matrixes of random `u8` elements and size `4096`.
    pub struct Producer {
        pub subscribers: Vec<actix::Recipient<Signal>>,
    }
    /// Implement Producer.
    impl Producer {
        /// Implement generates square matrixes.
        pub fn generate_matrix() -> HashMap<(i32, i32), u8> {
            let mut matrix: HashMap<(i32, i32), u8> = HashMap::with_capacity(4096);
            let mut rng = thread_rng();
            for x in 1..65 {
                for y in 1..65 {
                    matrix.insert((x, y), rng.gen::<u8>());
                }
            }
            matrix
        }

        /// Sending Signal Type Messages.
        /// Send signal to all subscribers.
        fn send_signal(&mut self) {
            for subscr in &self.subscribers {
                subscr.do_send(Signal(Producer::generate_matrix()));
            }
        }
    }

    /// Implement Actor for Producer.
    impl actix::Actor for Producer {
        type Context = actix::Context<Self>;
        /// Interval alert subscribers.
        fn started(&mut self, ctx: &mut Self::Context) {
            ctx.run_interval(Duration::from_millis(110), |actor, _ctx| {
                actor.send_signal();
            });
        }
    }

}

fn main() {
    use actor_matrix::*;

    System::run(|| {
        let addr_1: actix::Addr<Consumer> = Consumer.start();
        let addr_2: actix::Addr<Consumer> = addr_1.clone();
        Producer {
            subscribers: vec![addr_1.recipient(), addr_2.recipient()],
        }.start();
    });
}
