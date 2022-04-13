#[macro_use]
extern crate bitflags;

/// # Bit mask application
///
/// The module json implements a fictitious analog of the PHP function, json_encode ()
/// which takes the bitmask as the second parameter and returns its
/// readable string representation instead of the actual JSON.
///
/// To work with bit masks we use [bitflags]: https://crates.io/crates/bitflags
///
/// ## Examples
///
/// Basic usage:
///
/// ```rust
///  use json::*;
///
///   if let Some(mask) = json_encode_fict(
///        &vec![1, 2, 3],
///        JSON::HEX_TAG |
///        JSON::HEX_APOS |
///        JSON::PRETTY_PRINT,
///   ) {
///        assert_eq!(mask,String::from("00010000101"));
///   }
/// ```
mod json {
    use super::*;

    /// The bitflags contains JSON masks.
    bitflags! {
        pub struct JSON : u32 {

            /// All < and > are converted to \u003C and \u003E.
            const HEX_TAG =          0b00000000001;

            /// All &s are converted to \u0026.
            const HEX_AMP =          0b00000000010;

            /// All ' are converted to \u0027.
            const HEX_APOS =         0b00000000100;

            /// All " are converted to \u0022.
            const HEX_QUOT =         0b00000001000;

            /// Outputs an object rather than an array when a non-associative array is used.
            /// Especially useful when the recipient of the output
            /// is expecting an object and the array is empty.
            const FORCE_OBJECT =     0b00000010000;

            /// Encodes numeric strings as numbers.
            const NUMERIC_CHECK =    0b00000100000;

            /// Don't escape /.
            const UNESCAPED_SLASHES =0b00001000000;

            /// Use whitespace in returned data to format it.
            const PRETTY_PRINT =     0b00010000000;

            /// Encode multibyte Unicode characters literally (default is to escape as \uXXXX).
            const UNESCAPED_UNICODE =0b00100000000;

            /// Substitute some unencodable values instead of failing.
            const PARTIAL_OUTPUT_ON_ERROR =  0b01000000000;

            /// Ensures that float values are always encoded as a float value.
            const PRESERVE_ZERO_FRACTION =   0b10000000000;

        }
    }

    /// Implementations Display trait.
    impl std::fmt::Display for JSON {
        /// Formatted output according to the length of the mask.
        fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
            write!(f, "{:011b}", self.bits)
        }
    }

    /// Implementations Default trait.
    impl Default for JSON {
        /// The default value is JSON_HEX_TAG and JSON_HEX_AMP masks.
        fn default() -> JSON {
            JSON::HEX_TAG | JSON::HEX_AMP
        }
    }

    /// Implements the fictitious function of the PHP, json_encode().
    /// Returns a simple string representation of the mask.
    ///
    /// ## Examples
    ///
    /// Basic usage:
    ///
    /// ```rust
    ///  use json::*;
    ///
    ///  if let Some(mask) = json_encode_fict(&vec![1, 2, 3], Default::default() ) {
    ///     assert_eq!(mask,String::from("00000000011"));
    ///  }
    /// ```
    pub fn json_encode_fict(value: &Vec<i32>, mask: JSON) -> Option<String> {
        let display = format!("{}", format_args!("{:011b}", mask.bits()));

        Some(display)
    }

    #[cfg(test)]
    mod test {
        use super::*;

        #[test]
        fn bit_or_test() {
            if let Some(mask) = json_encode_fict(
                &vec![1],
                JSON::HEX_TAG | JSON::HEX_APOS | JSON::PRETTY_PRINT,
            ) {
                assert_eq!(mask, String::from("00010000101"));
            } else {
                assert!(false);
            }
        }

        #[test]
        fn bit_and_test() {
            if let Some(mask) = json_encode_fict(
                &vec![1, 2, 3],
                JSON::HEX_TAG & JSON::HEX_APOS & JSON::PRETTY_PRINT,
            ) {
                assert_eq!(mask, String::from("00000000000"));
            } else {
                assert!(false);
            }
        }

        #[test]
        fn bit_xor_test() {
            if let Some(mask) = json_encode_fict(
                &vec![1, 2, 3],
                JSON::HEX_TAG ^ JSON::HEX_APOS ^ JSON::HEX_AMP,
            ) {
                assert_eq!(mask, String::from("00000000111"));
            } else {
                assert!(false);
            }
        }

        #[test]
        fn sub_test() {
            if let Some(mask) = json_encode_fict(
                &vec![1, 2, 3],
                JSON::HEX_TAG | JSON::HEX_AMP - JSON::HEX_TAG,
            ) {
                assert_eq!(mask, String::from("00000000011"));
            } else {
                assert!(false);
            }
        }

        #[test]
        fn not_test() {
            if let Some(mask) = json_encode_fict(&vec![1, 2, 3], !(JSON::HEX_TAG | JSON::HEX_AMP)) {
                assert_eq!(mask, String::from("11111111100"));
            } else {
                assert!(false);
            }
        }

    }

}

fn main() {
    use json::{json_encode_fict, JSON};

    if let Some(mask) = json_encode_fict(
        &vec![1, 2, 3],
        JSON::HEX_TAG | JSON::HEX_APOS | JSON::PRETTY_PRINT,
    ) {
        assert_eq!(mask, String::from("00010000101"));
    }
}
