#[macro_use]
extern crate serde_derive;
#[macro_use]
extern crate envy;
extern crate dotenv;
#[macro_use]
extern crate lazy_static;
use dotenv::dotenv;

use std::collections::HashMap;
use std::env;
use std::path::Path;
/// # Example environment variables.
///
/// The simple program which lookups for 3 environment variables
/// (`ENV_VAR_ONE`, `ENV_VAR_TWO`, `ENV_VAR_THREE`) and for each variable prints:
/// - `<absent>` if variable is not defined;
/// - `<empty>` if variable is defined but is empty string;
/// - value of variable in other cases.
///
/// The module contains two functions: variant_envy and variant_env.
/// The function of variant_envy uses crate envy.
/// The function of variant_envy uses crate env.
/// The test is used crate dotenv.
///
/// ## Examples
///
/// Basic usage:
///
/// ```rust
///
/// use environment_variables::*;
///
/// if let Ok(path) = env::current_dir().and_then(|a| Ok(a.join(".env"))) {
///  dotenv::from_path(path);
/// } else {
///  assert!(false);
/// }
///
/// variant_envy();
/// ```
mod environment_variables {
    use super::*;

    /// The structure contains the expected environment variables.
    #[derive(Deserialize, Debug, PartialEq)]
    #[serde(default)]
    pub struct Config {
        pub one: String,
        pub two: String,
        pub three: String,
        #[serde(skip)]
        pub index: i32,
    }

    /// Implement an iterator to enumerate environment variables.
    impl Iterator for Config {
        type Item = (String, String);
        fn next(&mut self) -> Option<Self::Item> {
            self.index -= 1;
            match self.index {
                0 => Some((String::from("ENV_VAR_ONE"), self.one.to_string())),
                1 => Some((String::from("ENV_VAR_TWO"), self.two.to_string())),
                2 => Some((String::from("ENV_VAR_THREE"), self.three.to_string())),
                _ => None,
            }
        }
    }

    /// Implement a trait Default.
    impl Default for Config {
        fn default() -> Self {
            Config {
                one: Default::default(),
                two: Default::default(),
                three: Default::default(),
                index: 3,
            }
        }
    }

    /// The function that uses the crate env.
    /// Outputs the environment variables to the `stdout` stream.
    ///
    /// ## Examples
    ///
    /// Basic usage:
    ///
    /// ```rust
    ///
    /// use environment_variables::*;
    ///
    /// if let Ok(path) = env::current_dir().and_then(|a| Ok(a.join(".env"))) {
    ///  dotenv::from_path(path);
    /// } else {
    ///  assert!(false);
    /// }
    ///
    /// variant_env();
    /// ```
    pub fn variant_env() {
        let mut vars: [&str; 3] = ["ENV_VAR_THREE", "ENV_VAR_TWO", "ENV_VAR_ONE"];

        let vars_map: HashMap<String, String> = vars
            .into_iter()
            .map(|&var| {
                env::var_os(var).map_or((var.to_uppercase(), String::from("<absent>")), |value| {
                    (
                        var.to_uppercase(),
                        value
                            .into_string()
                            .and_then(|string| {
                                if string.is_empty() {
                                    Ok(String::from("<empty>"))
                                } else {
                                    Ok(string)
                                }
                            })
                            .unwrap(),
                    )
                })
            })
            .collect();

        for (key, val) in vars_map.iter() {
            println!("{}: {}", key, val);
        }
    }

    /// The function that uses the crate envy.
    /// Outputs the environment variables to the `stdout` stream.
    ///
    /// ## Examples
    ///
    /// Basic usage:
    ///
    /// ```rust
    ///
    /// use environment_variables::*;
    ///
    /// if let Ok(path) = env::current_dir().and_then(|a| Ok(a.join(".env"))) {
    ///  dotenv::from_path(path);
    /// } else {
    ///  assert!(false);
    /// }
    ///
    /// variant_envy();
    /// ```
    pub fn variant_envy() {
        let vars_map: HashMap<String, String> = envy::prefixed("ENV_VAR_")
            .from_env::<Config>()
            .and_then(|vars_map| {
                Ok(vars_map
                    .into_iter()
                    .map(|(key, value)| {
                        env::var_os(&key).map_or(
                            (key.to_uppercase(), String::from("<absent>")),
                            |value| {
                                (
                                    key.to_uppercase(),
                                    value
                                        .into_string()
                                        .and_then(|string| {
                                            if string.is_empty() {
                                                Ok(String::from("<empty>"))
                                            } else {
                                                Ok(string)
                                            }
                                        })
                                        .unwrap(),
                                )
                            },
                        )
                    })
                    .collect())
            })
            .unwrap();

        for (key, val) in vars_map.iter() {
            println!("{}: {}", key, val);
        }
    }

    #[cfg(test)]
    mod test {
        use super::*;

        #[test]
        fn test_env() {
            if let Ok(path) = env::current_dir().and_then(|a| Ok(a.join(".env"))) {
                dotenv::from_path(path);
            } else {
                assert!(false);
            }

            variant_env();
            assert!(true);
        }

        #[test]
        fn test_envy() {
            if let Ok(path) = env::current_dir().and_then(|a| Ok(a.join(".env"))) {
                dotenv::from_path(path);
            } else {
                assert!(false);
            }

            variant_envy();
            assert!(true);
        }
    }

}

fn main() {
    use environment_variables::*;

    if let Ok(path) = env::current_dir().and_then(|a| Ok(a.join(".env"))) {
        dotenv::from_path(path);
    } else {
        assert!(false);
    }

    println!("Variant crate envy:\n");

    variant_envy();

    println!("\nVariant std::env:\n");

    variant_env();
}
