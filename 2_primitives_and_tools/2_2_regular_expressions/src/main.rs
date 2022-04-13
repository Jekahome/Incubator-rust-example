extern crate regex;

#[macro_use]
extern crate lazy_static;

use regex::Regex;

/// #User module with validation email
///
/// Regular expression is taken from [source]: https://habr.com/post/55820/
///
/// ## Examples
///
/// Basic usage:
///
/// ```rust
///  use user::User;
///
///  if let Some(user) = User::validate_and_set_email("mail@mail.ru") {
///    if let Some(domain) = user.email_domain(){
///      assett_eq!(domain,"mail.ru");
///    }
///  }
/// ```
mod user {
    use super::*;

    /// Structure containing the user's email.
    pub struct User<'a> {
        pub email: &'a str,
    }
    /// Methods for verifying the user's email.
    impl<'a> User<'a> {
        /// Creates the `User` object after successfully checking its email.
        /// Validation email «username@hostname»,
        /// username : latin characters, numbers, signs !#$%&'*+—/=?^_`{|}~
        /// hostname : contains components and suffixes (first-level domains) and domains of countries.
        ///
        /// ## Examples
        ///
        /// Basic usage:
        ///
        /// ```rust
        ///  use user::User;
        ///
        ///  if let Some(user) = User::validate_and_set_email("mail@mail.ru") {
        ///    assert!(true);
        ///  }
        /// ```
        pub fn validate_and_set_email(email: &'a str) -> Option<Self> {
            lazy_static! {
               static ref EMAIL: Regex =  Regex::new(r"(?x)
                                        ^[-a-z0-9!\#$%&'*+/=?^_`{|}~]+(\.[-a-z0-9!\#$%&'*+/=?^_`{|}~]+)*  # the username
                                        @([a-z0-9]([-a-z0-9]{0,61}[a-z0-9])?\.)*  # components separated by a period and not exceeding 63 characters
                                        ([a-z]{2,5}) # suffixes (limited list of first level domains)
                                        \.[a-z][a-z]$                             # country domains
                                        ").unwrap();
            }

            if EMAIL.is_match(email) {
                return Some(User { email: email });
            }
            return None;
        }

        /// Analyzes a portion of the domain with the user's email address with a regular expression and returns it.
        /// ## Examples
        ///
        /// Basic usage:
        ///
        /// ```rust
        ///  use user::User;
        ///
        ///  if let Some(user) = User::validate_and_set_email("mail@mail.ru") {
        ///    if let Some(domain) = user.email_domain(){
        ///      assett_eq!(domain,"mail.ru");
        ///    }
        ///  }
        /// ```
        pub fn email_domain(&self) -> Option<&'a str> {
            lazy_static! {
                static ref EMAIL_DOMAIN: Regex = Regex::new(r"@").unwrap();
            }

            EMAIL_DOMAIN.split(self.email).last()
        }

    }

    #[cfg(test)]
    mod test {
        use super::*;

        #[test]
        fn none_validation_email() {
            match User::validate_and_set_email("mailmail.ru") {
                Some(_) => assert!(false),
                None => assert!(true),
            }
        }

        #[test]
        fn none_validation_domain() {
            match User::validate_and_set_email("mail@mailru") {
                Some(_) => assert!(false),
                None => assert!(true),
            }
        }

        #[test]
        fn none_validation_username() {
            match User::validate_and_set_email("().@mail.ru") {
                Some(_) => assert!(false),
                None => assert!(true),
            }
        }

        #[test]
        fn some_validation_username() {
            match User::validate_and_set_email("user.user@mail.ru") {
                Some(_) => assert!(true),
                None => assert!(false),
            }
        }

        #[test]
        fn some_validation_domain() {
            match User::validate_and_set_email("user.user@mail.ru") {
                Some(user) => {
                    if let Some(domain) = user.email_domain() {
                        assert_eq!(domain, "mail.ru");
                    }
                }
                None => assert!(false),
            }
        }
    }
}

fn main() {
    use user::User;

    if let Some(user) = User::validate_and_set_email("mail@mail.ru") {
        if let Some(domain) = user.email_domain() {
            assert_eq!(domain, "mail.ru");
        }
    }
}
