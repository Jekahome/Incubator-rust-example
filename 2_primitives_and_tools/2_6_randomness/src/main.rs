extern crate rand;

/// # Functions of working with random numbers
/// The module contains a set of functions (`new_access_token`, `generate_password`, `select_rand_val`)
/// that work with random number generators.
///
/// ## Examples
///
/// Basic usage:
///
/// ```rust
///
///  use rand_mod::generate_password;
///
///  let password_ten:String = generate_password(10);
///
///  assert_eq!(10, password_ten.len());
/// ```
mod rand_mod {

    use rand::distributions::{Alphanumeric, Distribution};
    use rand::prng::isaac64::Isaac64Rng;
    use rand::rngs::EntropyRng;
    use rand::rngs::SmallRng;
    use rand::{FromEntropy, Rng, RngCore};

    /// Generate unique cryptographically secure random value in `a-zA-Z0-9`
    /// symbols set and has exactly `64` symbols.
    ///
    /// ## Examples
    ///
    /// Basic usage:
    ///
    /// ```rust
    ///
    ///  use rand_mod::new_access_token;
    ///
    ///  let password_64:String = new_access_token();
    ///
    ///  assert_eq!(64, password_64.len());
    /// ```
    pub fn new_access_token() -> String {
        let mut Isaac64Rng = Isaac64Rng::new_from_u64(EntropyRng::new().next_u64());
        Alphanumeric.sample_iter(&mut Isaac64Rng).take(64).collect()
    }

    /// Generate random password of given length and symbols set.
    ///
    /// ## Examples
    ///
    /// Basic usage:
    ///
    /// ```rust
    ///
    ///  use rand_mod::generate_password;
    ///
    ///  let password_ten:String = generate_password(10);
    ///
    ///  assert_eq!(10, password_ten.len());
    /// ```
    pub fn generate_password(length: usize) -> String {
        let mut Isaac64Rng = Isaac64Rng::new_from_u64(EntropyRng::new().next_u64());
        Isaac64Rng.sample_iter(&Alphanumeric).take(length).collect()
    }

    /// Retrieve random element of given slice.
    ///
    /// ## Examples
    ///
    /// Basic usage:
    ///
    /// ```rust
    ///
    ///  use rand_mod::select_rand_val;
    ///
    ///  let vector: Vec<i32> = vec![1, 2, 3, 4, 5, 6, 7, 8, 9, 10];
    ///
    ///  assert!(vector.contains(&select_rand_val(vector.as_slice())));
    /// ```
    pub fn select_rand_val(slice: &[i32]) -> i32 {
        let mut small_rng = SmallRng::from_entropy();
        let index: usize = small_rng.gen_range(0, slice.len());
        slice[index]
    }

    #[cfg(test)]
    mod test {
        use rand_mod::*;
        #[test]
        fn test_new_access_token() {
            assert_eq!(64, new_access_token().len());
        }
        #[test]
        fn test_generate_password() {
            assert_eq!(10, generate_password(10).len());
        }
        #[test]
        fn test_select_rand_val() {
            let vector: Vec<i32> = vec![1, 2, 3, 4, 5, 6, 7, 8, 9, 10];
            assert!(vector.contains(&select_rand_val(vector.as_slice())));
        }
    }
}

fn main() {
    use rand_mod::*;

    let vector: Vec<i32> = vec![1, 2, 3, 4, 5, 6, 7, 8, 9, 10];
    assert!(vector.contains(&select_rand_val(vector.as_slice())));
}
