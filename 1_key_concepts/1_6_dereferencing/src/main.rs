use std::fs;
use std::io::{Read, Write};
use std::path::Path;

/// The module implements a smart pointer
///
/// The module implements inefficient and funny user-defined type of smart pointer containing the data File.
/// Removes data after the end of the lifetime.
/// For a smart pointer File<'a, T> implemented traits: Deref, DerefMut, Drop
///
/// # Examples
///
/// Basic usage:
///
/// ```rust
///  use SPFile::File;
///
///  let path = Path::new("file.txt");
///
///   if let Some(file) = File::create(path){
///      let b: &[u8] = "some bytes".as_bytes();
///      let mut file = &*file;
///      file.write(b);
///   }
/// ```
mod SPFile {

    use super::*;
    use std::ops::{Deref, DerefMut};

    /// This structure is a smart point.
    /// The target data type is contained in the field `pub file` and has a data type std::fs::File.
    /// The `path` field  has a data type std::path::Path and contains the path to the file.
    #[derive(Debug)]
    pub struct File<'a, T> {
        pub file: T,
        path: &'a Path,
    }

    /// Implements Deref trait for smart pointer struct File<'a, T>.
    /// Ability to read target data only.
    impl<'a, T> Deref for File<'a, T> {
        type Target = T;
        /// Realization of the deref function for struct File<'a, T>.
        fn deref(&self) -> &T {
            &self.file
        }
    }

    /// Implements DerefMut trait for smart pointer struct File<'a, T>.
    /// Allows you to change the target data.
    impl<'a, T> DerefMut for File<'a, T> {
        /// Realization of the deref_mut function for struct File<'a, T>
        fn deref_mut(&mut self) -> &mut T {
            &mut self.file
        }
    }

    /// Implements Drop trait for smart pointer struct File<'a, T>.
    /// If you delete File<'a, T>, the target data will be deleted if the data exists.
    impl<'a, T> Drop for File<'a, T> {
        /// Realization of the drop function for struct File<'a, T>.
        fn drop(&mut self) {
            if self.path.exists() == true {
                if let Some(file_name) = &self.path.file_name() {
                    std::fs::remove_file(file_name);
                    println!("File is being dropped");
                }
            }
        }
    }

    /// Implements the target type  for the structure File<'a,T>.
    ///
    /// In dereferencing operations, explicit or not explicit,
    /// the value of the target type will be converted to std::fs::File.
    ///
    ///  ```text
    ///   if let Some(file) = File::create(path){
    ///      // the file is of type std::fs::File
    ///      let mut file:&std::fs::File = &*_file;
    ///    }
    ///  ```
    impl<'a> File<'a, fs::File> {
        /// Creates a smart-pointer for reading only.
        /// Condition, the file must exist.
        ///
        /// ## Examples
        ///
        /// Basic usage:
        ///
        /// ```rust
        ///  use SPFile::File;
        ///
        ///  let path = Path::new("file.txt");
        ///
        ///   if let Some(file) = File::create(path){
        ///      let b: &[u8] = "some bytes".as_bytes();
        ///      let mut file = &*file;
        ///      file.write(b);
        ///   }
        /// ```
        pub fn open(path: &'a Path) -> Option<File<fs::File>> {
            let file = fs::File::open(path).ok()?;
            Some(File::new(file, path))
        }

        /// Creates a smart-pointer for writing.
        ///
        /// ## Examples
        ///
        /// Basic usage:
        ///
        /// ```rust
        ///  use SPFile::File;
        ///
        ///  let path = Path::new("file.txt");
        ///
        ///   if let Some(file) = File::open(path){
        ///      let mut file = &*file;
        ///      let mut buffer = String::new();
        ///      file.read_to_string(&mut buffer);
        ///   }
        /// ```
        pub fn create(path: &'a Path) -> Option<File<fs::File>> {
            let file = fs::File::create(path).ok()?;
            Some(File::new(file, path))
        }
    }

    /// Implementation of the File for general type.
    /// General type T must implement std::fs::File.
    impl<'a, T> File<'a, T> {
        /// Creates new `File<T>` smart-pointer.
        fn new(file: T, path: &'a Path) -> Self {
            File {
                file: file,
                path: path,
            }
        }
    }

    #[test]
    fn file_test() {
        use SPFile::File;

        let path = Path::new("file.txt");

        match File::create(path) {
            Some(_file) => {
                let b: &[u8] = "some bytes".as_bytes();
                let mut file = &*_file;
                file.write(b);

                match File::open(path) {
                    Some(_file) => {
                        let mut file = &*_file;

                        let mut buffer = String::new();
                        file.read_to_string(&mut buffer);
                        assert_eq!("some bytes", buffer);
                    }
                    None => assert!(false),
                }

                assert!(File::open(path).is_none());
            }
            None => assert!(false),
        }
    }

}

fn main() {
    use SPFile::File;

    let path = Path::new("file.txt");

    match File::create(path) {
        Some(_file) => {
            let b: &[u8] = "some bytes".as_bytes();
            let mut file = &*_file;
            file.write(b);

            match File::open(path) {
                Some(_file) => {
                    let mut file = &*_file;

                    let mut buffer = String::new();
                    file.read_to_string(&mut buffer);
                    assert_eq!("some bytes", buffer);
                }
                None => assert!(false),
            }
        }
        None => assert!(false),
    }
}
