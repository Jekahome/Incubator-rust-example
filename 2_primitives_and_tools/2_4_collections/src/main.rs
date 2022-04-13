extern crate im;
use im::hashmap::HashMap;
use std::borrow::Cow;
use std::cmp::Ordering;

/// # Applying an immutable collection with pattern Repository
///
/// The module implements simple methods of applying [immutable collections]:
/// https://docs.rs/im/11.0.1/im.
/// Used as a [HashMap]:https://docs.rs/im/11.0.1/im/hashmap/struct.HashMap.html
/// of users, a search is performed for id and nickname.
/// An example of a `HashMap` change in place was implemented.
///
///
/// ## Examples
///
/// Basic usage:
///
/// ```rust
///  use super::*;
///
///  let mut map_users: HashMap<UserId, User> = <HashMap<UserId, User>>::new();
///
///  let user = User::new(UserId(4usize), Cow::Borrowed("Sara Delafon"));
///  map_users.insert(user.get_id().clone(), user);
///  let user = User::new(UserId(2usize), Cow::Borrowed("Jacob Delafon"));
///  map_users.insert(user.get_id().clone(), user);
///
///  let users_source: DBMemory = DBMemory::new(map_users);
///
///  assert!(get_user_by_id(&users_source, 2).is_some());
/// ```
mod users {

    use super::*;

    /// The simple `Repository` trait (interface) which supports 3 operations:
    ///  - returns single `User` by its ID;
    ///  - returns multiple `User`s by their IDs;
    ///  - return IDs of `User`s which `nickname` contains given string (search function);
    pub trait UsersRepository {
        /// User search by ID.
        fn get_user_by_id(&self, id: UserId) -> Option<User>;

        /// Search for all users that match the identifiers.
        fn get_users_by_ids(&self, vec: Vec<UserId>) -> HashMap<UserId, User>;

        /// Search for users by nickname.
        fn get_ids_user_by_nickname(&self, nickname: &str) -> Vec<UserId>;
    }

    /// Mock implementation of `UsersRepository` trait which allows in-place setup of returned values.
    pub trait UsersRepositoryMock {
        /// Search for a user by ID or create a user with this ID.
        fn get_user_by_id_mock(&mut self, id: UserId) -> Option<User>;

        /// Search for all users who match IDs or create users.
        /// In the absence of users, they are created with these identifiers.
        fn get_users_by_ids_mock(&mut self, vec: Vec<UserId>) -> HashMap<UserId, User>;

        /// Search for users by nickname.
        /// If there is no result, a user with nickname.
        fn get_ids_user_by_nickname_mock(&mut self, nickname: &'static str) -> Vec<UserId>;
    }

    /// Simple user type.
    #[derive(Debug, Clone)]
    pub struct User {
        id: UserId,
        nickname: Cow<'static, str>,
    }

    /// Simple identifier type for `User` type.
    /// will be used as a key in hashmap for this we implement a crunchy tarit:
    /// Eq,Ord,PartialOrd,PartialEq.
    #[derive(Eq, Debug, Clone, Hash)]
    pub struct UserId(pub usize);

    /// ## Implementation of tarit for comparing identifiers.

    /// Implementation of Ord for UserId.
    impl Ord for UserId {
        fn cmp(&self, other: &UserId) -> Ordering {
            self.0.cmp(&other.0)
        }
    }
    /// Implementation of PartialOrd for UserId.
    impl PartialOrd for UserId {
        fn partial_cmp(&self, other: &UserId) -> Option<Ordering> {
            Some(self.cmp(other))
        }
    }
    /// Implementation of PartialEq for UserId.
    impl PartialEq for UserId {
        fn eq(&self, other: &UserId) -> bool {
            self.0 == other.0
        }
    }

    /// The type implements the Repository pattern.
    /// The users field is private to hide direct access to methods such as DBMemory.
    /// To work with methods like `DBMemory` implements the trait `UsersRepository`:
    /// `get_user_by_id()`,`get_users_by_ids()`,`get_ids_user_by_nickname()`.
    /// To work with methods like `DBMemory` implements the trait `UsersRepositoryMock`:
    /// `get_user_by_id_mock()`,`get_users_by_ids_mock()`,`get_ids_user_by_nickname_mock()`.
    #[derive(Debug)]
    pub struct DBMemory {
        users: HashMap<UserId, User>,
    }

    /// Methods of type DBMemory.
    impl DBMemory {
        /// Creates a new DBMemory object.
        pub fn new(users: HashMap<UserId, User>) -> Self {
            DBMemory { users: users }
        }
    }

    /// Methods of type User.
    /// Implemented set and get methods for private fields.
    impl User {
        /// Creates a new User object.
        pub fn new(id: UserId, nickname: Cow<'static, str>) -> Self {
            User {
                id: id,
                nickname: nickname,
            }
        }

        /// Returns the private field `id`.
        pub fn get_id(&self) -> &UserId {
            &self.id
        }

        /// Returns the private field `nickname`.
        pub fn get_nickname(&self) -> &str {
            &self.nickname
        }

        /// Sets the value for the `nickname` field.
        pub fn set_nickname(&mut self, nickname: Cow<'static, str>) {
            self.nickname = nickname;
        }
    }

    /// Implementing the template Repocators for type `DBMemory`
    impl UsersRepository for DBMemory {
        /// User search by ID.
        /// Access to the method via the function `get_user_by_id()`.
        ///
        /// ## Examples
        ///
        /// Basic usage:
        ///
        /// ```rust
        ///  use super::*;
        ///
        ///  let mut map_users: HashMap<UserId, User> = <HashMap<UserId, User>>::new();
        ///
        ///  let user = User::new(UserId(4usize), Cow::Borrowed("Sara Delafon"));
        ///  map_users.insert(user.get_id().clone(), user);
        ///
        ///  let mut users_source: DBMemory = DBMemory::new(users);
        ///
        ///  assert!(get_user_by_id(&users_source, UserId(2)).is_some());
        /// ```
        fn get_user_by_id(&self, id: UserId) -> Option<User> {
            if let Some(user) = self.users.get(&id) {
                return Some(user.clone());
            };
            None
        }

        /// Search for all users that match the identifiers.
        /// Access to the method via the function `get_users_by_ids()`.
        ///
        /// ## Examples
        ///
        /// Basic usage:
        ///
        /// ```rust
        ///  use super::*;
        ///
        ///  let mut map_users: HashMap<UserId, User> = <HashMap<UserId, User>>::new();
        ///
        ///  let user = User::new(UserId(4usize), Cow::Borrowed("Sara Delafon"));
        ///  map_users.insert(user.get_id().clone(), user);
        ///
        ///  let user = User::new(UserId(2usize), Cow::Borrowed("Jacob Delafon"));
        ///  map_users.insert(user.get_id().clone(), user);
        ///
        ///  let user = User::new(UserId(5usize), Cow::Borrowed("Sara Daniel"));
        ///  map_users.insert(user.get_id().clone(), user);
        ///
        ///  let mut users_source: DBMemory = DBMemory::new(map_users);
        ///
        ///  let v = get_users_by_ids(&users_source, vec![UserId(2), UserId(4)]);
        ///
        ///  assert_eq!(2, v.len());
        ///
        /// ```
        fn get_users_by_ids(&self, vec: Vec<UserId>) -> HashMap<UserId, User> {
            self.users
                .iter()
                .filter(|(_, ref value)| vec.contains(&value.get_id()))
                .cloned()
                .collect::<HashMap<UserId, User>>()
        }

        /// Search for users by nickname.
        /// Access to the method via the function `get_ids_user_by_nickname()`.
        ///
        /// ## Examples
        ///
        /// Basic usage:
        ///
        /// ```rust
        ///  use super::*;
        ///  let mut map_users: HashMap<UserId, User> = <HashMap<UserId, User>>::new();
        ///
        ///  let user = User::new(UserId(4usize), Cow::Borrowed("Sara Delafon"));
        ///  map_users.insert(user.get_id().clone(), user);
        ///  let user = User::new(UserId(2usize), Cow::Borrowed("Jacob Delafon"));
        ///  map_users.insert(user.get_id().clone(), user);
        ///
        ///  let mut users_source: DBMemory = DBMemory::new(map_users);
        ///  let ids:Vec<UserId> = get_ids_user_by_nickname(&users_source, "Delafon");
        ///
        ///  assert_eq!(2, ids.len());
        ///
        /// ```
        fn get_ids_user_by_nickname(&self, nickname: &str) -> Vec<UserId> {
            let nickname = nickname.to_lowercase();
            let nickname: &str = nickname.as_str();
            let map: HashMap<UserId, User> = self.users
                .iter()
                .filter(|(_, ref value)| value.get_nickname().to_lowercase().contains(nickname))
                .cloned()
                .collect::<HashMap<UserId, User>>();
            map.keys().cloned().collect::<Vec<UserId>>()
        }
    }
    /// Mock implementing the template Repocators for type `DBMemory`.
    impl UsersRepositoryMock for DBMemory {
        /// Search for a user by ID or create a user with this ID.
        /// Access to the method via the function `get_user_by_id_mock()`.
        ///
        /// ## Examples
        ///
        /// Basic usage:
        ///
        /// ```rust
        ///  use super::*;
        ///
        ///  let map_users: HashMap<UserId, User> = <HashMap<UserId, User>>::new();
        ///  let mut users_source: DBMemory = DBMemory::new(map_users);
        ///
        ///  assert!(get_user_by_id(&users_source, 2).is_some());
        /// ```
        fn get_user_by_id_mock(&mut self, id: UserId) -> Option<User> {
            let user: User = self.users
                .entry(id.clone())
                .or_insert(User::new(id, Default::default()))
                .clone();
            Some(user)
        }

        /// Search for all users that match the identifiers.
        /// In the absence of users, they are created with these identifiers.
        /// Access to the method via the function `get_users_by_ids_mock()`.
        ///
        /// ## Examples
        ///
        /// Basic usage:
        ///
        /// ```rust
        ///  use super::*;
        ///
        ///  let map_users: HashMap<UserId, User> = <HashMap<UserId, User>>::new();
        ///  let mut users_source: DBMemory = DBMemory::new(map_users);
        ///
        ///  let v = get_users_by_ids(&users_source, vec![UserId(2), UserId(4)]);
        ///
        ///  assert_eq!(2, v.len());
        ///
        /// ```
        fn get_users_by_ids_mock(&mut self, vec: Vec<UserId>) -> HashMap<UserId, User> {
            for key in &vec {
                &*self.users
                    .entry(key.clone())
                    .or_insert(User::new(key.clone(), Default::default()));
            }
            self.users
                .iter()
                .filter(|(_, ref value)| vec.contains(&value.get_id()))
                .cloned()
                .collect::<HashMap<UserId, User>>()
        }

        /// Search for users by nickname.
        /// If there is no result, a user with nickname.
        /// Access to the method via the function `get_ids_user_by_nickname_mock()`.
        ///
        /// ### Examples
        ///
        /// Basic usage:
        ///
        /// ```rust
        ///  use super::*;
        ///  let map_users: HashMap<UserId, User> = <HashMap<UserId, User>>::new();
        ///
        ///  let mut users_source: DBMemory = DBMemory::new(map_users);
        ///  let ids:Vec<UserId> = get_ids_user_by_nickname_mock(&users_source, "Delafon");
        ///
        ///  assert_eq!(1, ids.len());
        ///
        /// ```
        fn get_ids_user_by_nickname_mock(&mut self, nickname: &'static str) -> Vec<UserId> {
            let nickname_lower = nickname.to_lowercase();
            let nickname_lower: &str = nickname_lower.as_str();

            let map: HashMap<UserId, User> = self.users
                .iter()
                .filter(|(_, ref value)| {
                    value.get_nickname().to_lowercase().contains(nickname_lower)
                })
                .cloned()
                .collect::<HashMap<UserId, User>>();
            let id: UserId = UserId(0);
            let mut ids: Vec<UserId> = map.keys().cloned().collect::<Vec<UserId>>();
            if ids.is_empty() {
                &*self.users
                    .entry(id.clone())
                    .and_modify(|ref mut value| value.set_nickname(Cow::Borrowed(nickname)))
                    .or_insert(User::new(id.clone(), Cow::Borrowed(nickname)));

                ids.push(id);
            }
            ids
        }
    }

    /// ## These functions provide an interface for any type of Implementing `UsersRepository` tarit.

    /// Provides access to the `get_user_by_id` method.
    ///
    /// ### Examples
    ///
    /// Basic usage:
    ///
    /// ```rust
    ///  use super::*;
    ///
    ///  let mut map_users: HashMap<UserId, User> = <HashMap<UserId, User>>::new();
    ///
    ///  let user = User::new(UserId(4usize), Cow::Borrowed("Sara Delafon"));
    ///  map_users.insert(user.get_id().clone(), user);
    ///
    ///  let mut users_source: DBMemory = DBMemory::new(users);
    ///
    ///  assert!(get_user_by_id(&users_source, UserId(2)).is_some());
    /// ```
    pub fn get_user_by_id(repository: &users::UsersRepository, id: UserId) -> Option<User> {
        repository.get_user_by_id(id)
    }

    /// Provides access to the `get_users_by_ids` method.
    ///
    /// ### Examples
    ///
    /// Basic usage:
    ///
    /// ```rust
    ///  use super::*;
    ///
    ///  let mut map_users: HashMap<UserId, User> = <HashMap<UserId, User>>::new();
    ///
    ///  let user = User::new(UserId(4usize), Cow::Borrowed("Sara Delafon"));
    ///  map_users.insert(user.get_id().clone(), user);
    ///
    ///  let user = User::new(UserId(2usize), Cow::Borrowed("Jacob Delafon"));
    ///  map_users.insert(user.get_id().clone(), user);
    ///
    ///  let user = User::new(UserId(5usize), Cow::Borrowed("Sara Daniel"));
    ///  map_users.insert(user.get_id().clone(), user);
    ///
    ///  let mut users_source: DBMemory = DBMemory::new(map_users);
    ///
    ///  let users:HashMap<UserId, User> = get_users_by_ids(&users_source, vec![UserId(2), UserId(4)]);
    ///
    ///  assert_eq!(2, users.len());
    ///
    /// ```
    pub fn get_users_by_ids(
        repository: &users::UsersRepository,
        vec: Vec<UserId>,
    ) -> HashMap<UserId, User> {
        repository.get_users_by_ids(vec)
    }

    /// Provides access to the `get_ids_user_by_nickname` method.
    ///
    /// ### Examples
    ///
    /// Basic usage:
    ///
    /// ```rust
    ///  use super::*;
    ///  let mut map_users: HashMap<UserId, User> = <HashMap<UserId, User>>::new();
    ///
    ///  let user = User::new(UserId(5usize), Cow::Borrowed("Sara Delafon"));
    ///  map_users.insert(user.get_id().clone(), user);
    ///
    ///  let mut users_source: DBMemory = DBMemory::new(map_users);
    ///  let ids:Vec<UserId> = get_ids_user_by_nickname(&users_source, "Delafon");
    ///
    ///  assert_eq!(1, ids.len());
    ///
    /// ```
    pub fn get_ids_user_by_nickname(
        repository: &users::UsersRepository,
        nickname: &str,
    ) -> Vec<UserId> {
        repository.get_ids_user_by_nickname(nickname)
    }

    /// ## These functions provide an interface for any type of Implementing `UsersRepositoryMock` tarit.

    /// Provides access to the `get_user_by_id_mock` method.
    ///
    /// ### Examples
    ///
    /// Basic usage:
    ///
    /// ```rust
    ///  use super::*;
    ///
    ///  let mut users: HashMap<usize, User> = HashMap::new();
    ///  let mut users_source: DBMemory = DBMemory::new(users);
    ///
    ///  assert!(get_user_by_id(&users_source, 2).is_some());
    /// ```
    pub fn get_user_by_id_mock(
        repository: &mut users::UsersRepositoryMock,
        id: UserId,
    ) -> Option<User> {
        repository.get_user_by_id_mock(id)
    }

    /// Provides access to the `get_users_by_ids_mock` method.
    ///
    /// ### Examples
    ///
    /// Basic usage:
    ///
    /// ```rust
    ///  use super::*;
    ///
    ///  let map_users: HashMap<UserId, User> = <HashMap<UserId, User>>::new();
    ///  let mut users_source: DBMemory = DBMemory::new(map_users);
    ///
    ///  let users: HashMap<UserId, User> = get_users_by_ids(&users_source, vec![UserId(2), UserId(4)]);
    ///
    ///  assert_eq!(2, users.len());
    ///
    /// ```
    pub fn get_users_by_ids_mock(
        repository: &mut users::UsersRepositoryMock,
        vec: Vec<UserId>,
    ) -> HashMap<UserId, User> {
        repository.get_users_by_ids_mock(vec)
    }

    /// Provides access to the `get_ids_user_by_nickname_mock` method.
    ///
    /// ### Examples
    ///
    /// Basic usage:
    ///
    /// ```rust
    ///  use super::*;
    ///
    ///  let map_users: HashMap<UserId, User> = <HashMap<UserId, User>>::new();
    ///
    ///  let mut users_source: DBMemory = DBMemory::new(map_users);
    ///  let ids:Vec<UserId> = get_ids_user_by_nickname_mock(&users_source, "Delafon");
    ///
    ///  assert_eq!(1, ids.len());
    ///
    /// ```
    pub fn get_ids_user_by_nickname_mock(
        repository: &mut users::UsersRepositoryMock,
        nickname: &'static str,
    ) -> Vec<UserId> {
        repository.get_ids_user_by_nickname_mock(nickname)
    }

    #[cfg(test)]
    mod test {
        use super::*;

        #[test]
        fn test_get_users_by_ids() {
            let mut map_users: HashMap<UserId, User> = <HashMap<UserId, User>>::new();

            let user = User::new(UserId(4usize), Cow::Borrowed("Sara Delafon"));
            map_users.insert(user.get_id().clone(), user);

            let user = User::new(UserId(2usize), Cow::Borrowed("Jacob Delafon"));
            map_users.insert(user.get_id().clone(), user);

            let user = User::new(UserId(5usize), Cow::Borrowed("Sara Daniel"));
            map_users.insert(user.get_id().clone(), user);

            let users_source: DBMemory = DBMemory::new(map_users);

            let v = get_users_by_ids(&users_source, vec![UserId(2), UserId(4)]);

            assert_eq!(2, v.len());
        }

        #[test]
        fn test_get_user_by_id() {
            let mut map_users: HashMap<UserId, User> = <HashMap<UserId, User>>::new();

            let user = User::new(UserId(4usize), Cow::Borrowed("Sara Delafon"));
            map_users.insert(user.get_id().clone(), user);

            let user = User::new(UserId(2usize), Cow::Borrowed("Jacob Delafon"));
            map_users.insert(user.get_id().clone(), user);

            let users_source: DBMemory = DBMemory::new(map_users);

            assert!(get_user_by_id(&users_source, UserId(2)).is_some());
            assert!(get_user_by_id(&users_source, UserId(8)).is_none());
        }

        #[test]
        fn test_get_ids_user_by_nickname() {
            let mut map_users: HashMap<UserId, User> = <HashMap<UserId, User>>::new();

            let user = User::new(UserId(4usize), Cow::Borrowed("Sara Delafon"));
            map_users.insert(user.get_id().clone(), user);

            let user = User::new(UserId(2usize), Cow::Borrowed("Jacob Delafon"));
            map_users.insert(user.get_id().clone(), user);

            let user = User::new(UserId(5usize), Cow::Borrowed("Sara Daniel"));
            map_users.insert(user.get_id().clone(), user);

            let users_source: DBMemory = DBMemory::new(map_users);

            let ids: Vec<UserId> = get_ids_user_by_nickname(&users_source, "Delafon");

            if !ids.is_empty() {
                assert_eq!(2, ids.len());
            } else {
                assert!(false);
            }
        }

        #[test]
        fn test_get_user_by_id_mock() {
            let users: HashMap<UserId, User> = HashMap::new();

            let mut users_source: DBMemory = DBMemory::new(users);

            if let Some(user) = get_user_by_id_mock(&mut users_source, UserId(4)) {
                assert_eq!(&UserId(4), user.get_id());
            }
        }

        #[test]
        fn test_get_users_by_ids_mock() {
            let users: HashMap<UserId, User> = HashMap::new();

            let mut users_source: DBMemory = DBMemory::new(users);
            let value = get_users_by_ids_mock(&mut users_source, vec![UserId(2), UserId(4)]);

            assert_eq!(2, value.len());
        }

        #[test]
        fn test_get_ids_user_by_nickname_mock() {
            let users: HashMap<UserId, User> = HashMap::new();

            let mut users_source: DBMemory = DBMemory::new(users);

            let ids: Vec<UserId> = get_ids_user_by_nickname_mock(&mut users_source, "Delafon");

            if !ids.is_empty() {
                assert_eq!(1, ids.len());
            } else {
                assert!(false);
            }
        }
    }
}

fn main() {
    use users::*;

    let mut map_users: HashMap<UserId, User> = <HashMap<UserId, User>>::new();

    let user = User::new(UserId(4usize), Cow::Borrowed("Sara Delafon"));
    map_users.insert(user.get_id().clone(), user);

    let user = User::new(UserId(2usize), Cow::Borrowed("Jacob Delafon"));
    map_users.insert(user.get_id().clone(), user);

    let user = User::new(UserId(5usize), Cow::Borrowed("Sara Daniel"));
    map_users.insert(user.get_id().clone(), user);

    let users_source: DBMemory = DBMemory::new(map_users);

    let id: UserId = UserId(2);
    if let Some(user) = get_user_by_id(&users_source, id) {
        assert_eq!("Jacob Delafon", user.get_nickname());
    }

    let ids: Vec<UserId> = get_ids_user_by_nickname(&users_source, "Delafon");

    if !ids.is_empty() {
        let users: HashMap<UserId, User> = get_users_by_ids(&users_source, ids);
        assert_eq!(2, users.len());
    }

    // Mock
    let map_users: HashMap<UserId, User> = HashMap::new();
    let mut users_source: DBMemory = DBMemory::new(map_users);

    assert_eq!(
        &UserId(4),
        get_user_by_id_mock(&mut users_source, UserId(4))
            .unwrap()
            .get_id()
    );
    {
        let users: HashMap<UserId, User> =
            get_users_by_ids_mock(&mut users_source, vec![UserId(2), UserId(4)]);
        assert_eq!(2, users.len());
    }

    assert_eq!(
        1,
        get_ids_user_by_nickname_mock(&mut users_source, "Sara Delafon").len()
    );
}
