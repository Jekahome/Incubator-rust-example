extern crate chrono;
use chrono::prelude::*;

/// # Module for working with the date of birth.

/// A simple structure with the chrono::Date<Utc> field and the methods
/// of working with this date.
/// The module `user` uses the `chrono` module
/// [Chrono]: https://crates.io/crates/chrono
///
/// ## Examples
///
/// Basic usage:
///
/// ```rust
///  use user::User;
///
///  let (year, month, day) = (1985, 2, 13);
///  if let Some(user) = User::new(year, month, day) {
///     println!("Your age:{} years old", user.age());
///     match user.is_adult() {
///       true => println!("successful"),
///       false => println!("You are not yet 18 years old"),
///     }
///  }
/// ```
mod user {

    use super::*;

    /// The structure contains the user's date of birth.
    pub struct User {
        birthdate: Date<Utc>,
    }

    /// Implementation of methods for working with the date of birth of the user.
    impl User {
        /// Returns the current age of the user in years.
        /// It takes into account the high years and the time zone.
        ///
        /// ## Examples
        ///
        /// Basic usage:
        ///
        /// ```rust
        ///  use user::User;
        ///
        ///  let (year, month, day) = (1985, 2, 13);
        ///  if let Some(user) = User::new(year, month, day) {
        ///    println!("Your age:{} years old", user.age());
        ///  }
        /// ```
        pub fn age(&self) -> i32 {
            let today: Date<Utc> = Utc::today();

            let mut year = 1;

            if today.month() > self.birthdate.month()
                || self.birthdate.month() == today.month() && self.birthdate.day() <= today.day()
                || today.year() == self.birthdate.year()
            {
                year = 0;
            }

            (today.year() - self.birthdate.year()) - year
        }

        /// Checks if user is 18 years old at the moment.
        ///
        /// ## Examples
        ///
        /// Basic usage:
        ///
        /// ```rust
        ///  use user::User;
        ///
        ///  let (year, month, day) = (1985, 2, 13);
        ///    if let Some(user) = User::new(year, month, day) {
        ///      match user.is_adult() {
        ///        true => println!("successful"),
        ///        false => println!("You are not yet 18 years old"),
        ///      }
        ///   }
        /// ```
        pub fn is_adult(&self) -> bool {
            self.age() >= 18
        }

        /// Creates a new User object.
        ///
        /// ## Examples
        ///
        /// Basic usage:
        ///
        /// ```rust
        ///  use user::User;
        ///
        ///  let (year, month, day) = (1985, 2, 13);
        ///  if let Some(user) = User::new(year, month, day) {
        ///
        ///  }
        /// ```
        pub fn new(year: i32, month: u32, day: u32) -> Option<Self> {
            if Utc::today().year() < year {
                return None;
            }
            NaiveDate::from_ymd_opt(year, month, day).and_then(|naive_date: NaiveDate| {
                Some(User {
                    birthdate: Date::<Utc>::from_utc(naive_date, Utc),
                })
            })
        }
    }

    #[cfg(test)]
    mod test {
        use super::*;

        #[test]
        fn before_birthday() {
            let birthdate: Date<Utc> = (Utc::now() - chrono::Duration::days(1)).date();

            match User::new(birthdate.year(), birthdate.month(), birthdate.day()) {
                Some(_user) => {
                    let today: Date<Utc> = Utc::today();
                    if birthdate.month() > today.month()
                        || today.year() == birthdate.year()
                        || birthdate.month() == today.month() && birthdate.day() <= today.day()
                    {
                        assert_eq!(_user.age(), Utc::now().year() - birthdate.year());
                    } else {
                        assert_eq!(_user.age(), (Utc::now().year() - birthdate.year()) - 1);
                    }
                }
                None => assert!(false),
            }
        }

        #[test]
        fn after_birthday() {
            let birthdate: Date<Utc> = (Utc::now() + chrono::Duration::days(1)).date();

            match User::new(birthdate.year(), birthdate.month(), birthdate.day()) {
                Some(_user) => {
                    let today: Date<Utc> = Utc::today();
                    if birthdate.month() > today.month()
                        || today.year() == birthdate.year()
                        || birthdate.month() == today.month() && birthdate.day() <= today.day()
                    {
                        assert_eq!(_user.age(), Utc::now().year() - birthdate.year());
                    } else {
                        assert_eq!(_user.age(), (Utc::now().year() - birthdate.year()) - 1);
                    }
                }
                None => assert!(false),
            }
        }

        #[test]
        fn none_on_unknown_date() {
            assert!(User::new(2017, 2, 29).is_none());
        }

        #[test]
        fn some_on_success_date() {
            assert!(User::new(2016, 2, 29).is_some());
        }

        #[test]
        fn year_before_our_era() {
            assert!(User::new(-1000, 1, 1).is_some());
        }
    }
}

fn main() {
    use user::User;

    let (year, month, day) = (1985, 2, 13);
    if let Some(user) = User::new(year, month, day) {
        println!("Your age:{} years old", user.age());
        match user.is_adult() {
            true => println!("You are 18 years old"),
            false => println!("You are not yet 18 years old"),
        }
    }
}
