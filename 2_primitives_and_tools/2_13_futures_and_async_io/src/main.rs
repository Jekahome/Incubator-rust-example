extern crate futures;
extern crate hyper;
extern crate hyper_tls;
extern crate tokio;

#[macro_use(values_t, value_t, crate_version, crate_authors)]
extern crate clap;

use clap::{App, Arg, ArgMatches};
use futures::stream::Stream;
use hyper::Body;
use hyper::{Client, Request};
use std::fs::read_to_string;
use std::fs::File;
use std::io;
use std::io::Write;
use std::path::Path;
use tokio::runtime::Runtime;

/// # Downloading links through asynchronous libraries.
///
/// ## Module implements command line arguments.
///
/// ### Examples
///
/// Basic usage:
///
///```bash
///   $ MyApp download.file.link
///```
///
/// ```rust
///
///   use settings_args::*;
///
///   let settings: Settings = settings_args::new();
///   assert_eq!(4,settings.max_threads);
///   assert_eq!("download.file.link",settings.file);
///
/// ```
///   To call help:
///
///```bash
///   $ MyApp --help
///```
///
mod settings_args {
    use super::*;

    /// The typed data structure of command line arguments.
    #[derive(Debug)]
    pub struct Settings {
        pub max_threads: u8,
        pub file: String,
    }

    /// Function checking the existence of a file.
    fn has_file(file: String) -> Result<(), String> {
        if Path::new(&file).exists() {
            return Ok(());
        }
        Err(String::from("The file notfound"))
    }

    /// Return ArgMatches Object.
    fn get_matches<'a>() -> ArgMatches<'a> {
        App::new("Load files CLI")
            .usage("MyApp [--max-threads = <number>] <file>")
            .bin_name("MyApp")
            .version(crate_version!())
            .author(crate_authors!())
            .about("Load link")
            .args(&[
                Arg::with_name("file")
                    .validator(has_file)
                    .required(true)
                    .help("Link file, line break delimiter"),
                Arg::with_name("max-threads")
                    .long("max-threads")
                    .value_name("number")
                    .required(false)
                    .help("Number of threads"),
            ]).get_matches()
    }

    /// Create Settings Object.
    pub fn new() -> Settings {
        let matches = get_matches();

        let file = matches.value_of("file").unwrap();

        let max_threads: u8 = value_t!(matches, "max-threads", u8).unwrap_or(4);

        Settings {
            file: file.to_string(),
            max_threads: max_threads,
        }
    }
}


/// ## Load link
/// Read the list of links from `<file>` and concurrently load the contents of each link into a separate .html file (by reference)
/// ### Examples
///
/// Basic usage:
///
/// ```rust
///
///   use settings_args::*;
///   use load_html::load_html;
///
///   let settings: Settings = settings_args::new();
///
///   load_html(settings.max_threads as usize, &settings.file);
///
mod load_html {
    use super::*;
    /// Function a list of links and loads them in concurrently.
    pub fn load_html(
        max_threads: usize,
        file_list: &str,
    ) -> Result<(), Box<std::error::Error + 'static>> {
        let mut runtime = Runtime::new().unwrap();

        let mut https = hyper_tls::HttpsConnector::new(max_threads)?;

        let client = Client::builder().build::<_, hyper::Body>(https);

        let source: String = read_to_string(file_list)?;

        for (i, url) in source.lines().enumerate() {
            let req = Request::builder().uri(url).body(Body::empty())?;

            let response = runtime.block_on(client.request(req))?;

            let body = runtime.block_on(response.into_body().concat2())?;

            if let Ok(mut file) = File::create(format!("file_{}.html", i)) {
                file.write_all(&body);
            }
        }

        Ok(())
    }

    #[cfg(test)]
    mod test {
        use super::*;
        #[test]
        fn test_load_html() {
            std::fs::write("test_load_html", "https://www.google.com");
            match load_html(4, "test_load_html") {
                Ok(_) => {
                    std::fs::remove_file("test_load_html");
                    std::fs::remove_file("file_0.html");
                    assert!(true);
                }
                Err(_) => assert!(false),
            }
        }
    }
}

fn main() {
    use load_html::load_html;
    use settings_args::*;

    let settings: Settings = settings_args::new();

    load_html(settings.max_threads as usize, &settings.file);
}
