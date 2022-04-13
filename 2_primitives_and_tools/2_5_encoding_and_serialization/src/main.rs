#[macro_use]
extern crate serde_derive;
extern crate serde;
extern crate serde_json;
extern crate serde_yaml;
extern crate toml;

/// # Deserialization of the JSON in the readable TOML and YAML formats
///
/// The module deserializes the json file into the `Request` object.
/// The `Request` object can be printed in YAML and TOML [formats]:https://serde.rs/index.html#data-formats
///
///
/// ## Examples
///
/// Basic usage:
///
/// ```rust
///  use json::*;
///
///  use request::*;
///
///  if let Ok(request) = deserialized_to_request("request.json") {
///
///    println!("Format YAML:");
///    print_yaml(&request);
///
///  }
/// ```
mod request {
    use super::*;

    use std::error;
    use std::fmt;
    use std::io;
    use std::result;

    use serde::ser::{Serialize, SerializeStruct, Serializer};
    use std::fs::File;
    use std::path::Path;

    /// The structures representing the object `Request`.
    /// The reserved name `type` will be deserialized in the field `req_type`.
    #[derive(Deserialize)]
    pub struct Request {
        #[serde(rename(deserialize = "type"))]
        pub req_type: String,
        pub stream: Stream,
        pub gifts: Vec<Block>,
    }

    #[derive(Serialize, Deserialize)]
    pub struct Stream {
        pub model_id: i32,
        pub is_private: bool,
        pub erotic: i32,
        pub places: i32,
        pub shard_url: String,
        pub public_tariff: PublicTariff,
        pub private_tariff: PrivateTariff,
    }

    #[derive(Serialize, Deserialize)]
    pub struct PublicTariff {
        #[serde(flatten)]
        pub block: Block,
        pub duration: i32,
    }

    #[derive(Serialize, Deserialize)]
    pub struct PrivateTariff {
        #[serde(flatten)]
        pub block: Block,
        pub duration: i32,
    }
    /// Used for repetitive data structures.
    #[derive(Serialize, Deserialize)]
    pub struct Block {
        pub id: i32,
        pub model_price: i32,
        pub client_price: i32,
        pub description: String,
    }

    /// Alias for result::Result with FormatError ,
    /// combining types of format errors.
    type Result<T> = result::Result<T, FormatError>;

    /// The `FormatError` contains all types of errors necessary
    /// for the operation of the module with formats.
    #[derive(Debug)]
    pub enum FormatError {
        Io(io::Error),
        Json(serde_json::Error),
        Yaml(serde_yaml::Error),
        Toml(toml::ser::Error),
    }

    /// Implementation trait std::fmt::Display for FormatError
    impl fmt::Display for FormatError {
        fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
            match *self {
                FormatError::Io(ref err) => write!(f, "IO error: {}", err),
                FormatError::Json(ref err) => write!(f, "JSON error: {};", err),
                FormatError::Yaml(ref err) => write!(f, "YAML error: {}", err),
                FormatError::Toml(ref err) => write!(f, "TOML error: {}", err),
            }
        }
    }

    /// Implementation trait std::error::Error for FormatError
    impl error::Error for FormatError {
        fn description(&self) -> &str {
            match *self {
                FormatError::Io(ref err) => err.description(),
                FormatError::Json(ref err) => err.description(),
                FormatError::Yaml(ref err) => err.description(),
                FormatError::Toml(ref err) => err.description(),
            }
        }
        fn cause(&self) -> Option<&error::Error> {
            match *self {
                FormatError::Io(ref err) => Some(err),
                FormatError::Json(ref err) => Some(err),
                FormatError::Yaml(ref err) => Some(err),
                FormatError::Toml(ref err) => Some(err),
            }
        }
    }
    /// Type conversion io::Error in FormatError.
    impl From<io::Error> for FormatError {
        fn from(err: io::Error) -> FormatError {
            FormatError::Io(err)
        }
    }
    /// Type conversion serde_json::Error in FormatError.
    impl From<serde_json::Error> for FormatError {
        fn from(err: serde_json::Error) -> FormatError {
            FormatError::Json(err)
        }
    }
    /// Type conversion serde_yaml::Error in FormatError.
    impl From<serde_yaml::Error> for FormatError {
        fn from(err: serde_yaml::Error) -> FormatError {
            FormatError::Yaml(err)
        }
    }
    /// Type conversion toml::ser::Error in FormatError.
    impl From<toml::ser::Error> for FormatError {
        fn from(err: toml::ser::Error) -> FormatError {
            FormatError::Toml(err)
        }
    }

    /// Implementation trait Serialize
    /// to replace the reserved name `req_type` with` type`
    impl Serialize for Request {
        fn serialize<S>(&self, serializer: S) -> result::Result<S::Ok, S::Error>
        where
            S: Serializer,
        {
            let mut state = serializer.serialize_struct("Request", 3)?;
            state.serialize_field("type", &self.req_type)?;
            state.serialize_field("stream", &self.stream)?;
            state.serialize_field("gifts", &self.gifts)?;
            state.end()
        }
    }
    /// The function `deserialized_to_request` deserializes the file json
    /// into the object of the `Request`
    /// Prints a `Request` object in the TOML format.
    ///
    /// ## Examples
    ///
    /// Basic usage:
    ///
    /// ```rust
    ///  use json::*;
    ///
    ///  use request::*;
    ///
    ///  let request:Request  = deserialized_to_request("request.json");
    /// ```
    pub fn deserialized_to_request<P: AsRef<Path>>(path: P) -> Result<Request> {
        let file = File::open(path)?;
        let deserialized: Request = serde_json::from_reader(file)?;
        Ok(deserialized)
    }

    /// Prints a `Request` object in the YAML format.
    ///
    /// ## Examples
    ///
    /// Basic usage:
    ///
    /// ```rust
    ///  use json::*;
    ///
    ///  use request::*;
    ///
    ///  if let Ok(request) = deserialized_to_request("request.json") {
    ///
    ///    println!("Format YAML:");
    ///    print_yaml(&request);
    ///
    ///  }
    /// ```
    pub fn print_yaml(request: &Request) -> Result<()> {
        let s: String = serde_yaml::to_string(&request)?;
        println!("{}", s);
        Ok(())
    }

    /// Prints a `Request` object in the TOML format.
    ///
    /// ## Examples
    ///
    /// Basic usage:
    ///
    /// ```rust
    ///  use json::*;
    ///
    ///  use request::*;
    ///
    ///  if let Ok(request) = deserialized_to_request("request.json") {
    ///
    ///    println!("Format TOML:");
    ///    print_toml(&request);
    ///
    ///  }
    /// ```
    pub fn print_toml(request: &Request) -> Result<()> {
        let s: String = toml::to_string(&request)?;
        println!("{}", s);
        Ok(())
    }

    #[cfg(test)]
    mod test {
        #[test]
        fn test_yaml() {
            use request::*;
            if let Ok(request) = deserialized_to_request("request.json") {
                assert!(print_yaml(&request).is_ok());
            } else {
                assert!(false);
            }
        }

        #[test]
        fn test_toml() {
            use request::*;
            if let Ok(request) = deserialized_to_request("request.json") {
                assert!(print_toml(&request).is_ok());
            } else {
                assert!(false);
            }
        }
    }
}

fn main() {
    use request::*;

    if let Ok(request) = deserialized_to_request("request.json") {
        println!("Format YAML:");
        print_yaml(&request);

        println!("Format TOML:");
        print_toml(&request);
    }
}
