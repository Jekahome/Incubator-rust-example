#[macro_use]
extern crate clap;
extern crate handlebars;
#[macro_use]
extern crate serde_json;
extern crate env_logger;
extern crate serde;

use clap::{App, Arg,ArgMatches};
use handlebars::Handlebars;
use std::collections::btree_map::BTreeMap;
use std::error::Error;
use std::path::Path;

/// #Application clap
///
/// The module uses the Handlebars template and
/// crate clap for parsing command-line arguments.
///
/// ## Examples
///
/// Basic usage:
///
///```bash
///  $ MyApp --val world=Jeka --output out.txt  hello.handlebars
///```
/// To call help:
///
///```bash
/// $ MyApp --help
///```
mod cli_handlebars {
    use super::*;

    /// Function for checking the existence of a file.
    /// Used as a filter for the argument.
    fn has_file(file: String) -> Result<(), String> {
        if Path::new(&file).exists() {
            return Ok(());
        }
        Err(String::from("The file notfound"))
    }

    /// Return ArgMatches object.
    fn get_matches<'a>() -> ArgMatches<'a> {

        App::new("Handlebars CLI")
            .usage("MyApp [FLAGS] [OPTIONS] <FILE>")
            .bin_name("MyApp")
            .version(crate_version!())
            .author(crate_authors!())
            .about("Renders handlebars templates to STDOUT")
            .args(&[
                Arg::with_name("FILE")
                    .validator(has_file)
                    .required(true)
                    .help("The template file to be rendered"),
                Arg::with_name("data")
                    .short("v")
                    .long("val")
                    .takes_value(true)
                    .value_names(&["name", "value"])
                    .required(false)
                    .multiple(true)
                    .require_delimiter(true)
                    .value_delimiter("=")
                    .help("Set a value for template variable"),
                Arg::with_name("output")
                    .takes_value(true)
                    .short("o")
                    .required(false)
                    .long("output")
                    .value_name("FILE")
                    .help("Write rendering result into a file instead of STDOUT"),
            ])
            .get_matches()
    }

    /// The main function of the module.
    /// Executes command-line arguments parsing.
    /// The result of the work is written to the file of the `FILE` argument.
    pub fn init() -> Result<(), Box<Error>> {
        let mut handlebars: Handlebars = Handlebars::new();

        let matches = get_matches();
       
        let output_file = matches.value_of("output").unwrap_or("default.txt");

        let file = matches.value_of("FILE").unwrap();
        let source: &Path = Path::new(file);
        handlebars.register_template_file("tpl", source)?;

        if matches.is_present("data") {
            let iter = matches.values_of_lossy("data").unwrap().into_iter();

            let mut bool_ = false;
            let (even, odd): (Vec<String>, Vec<String>) = iter.partition(|ref mut _n| {
                bool_ = !bool_;
                bool_
            });
            let values: BTreeMap<_, _> = even.iter().zip(odd.iter()).collect::<BTreeMap<_, _>>();

            let data = handlebars.render("tpl", &values)?;
            std::fs::write(output_file, data);
        } else {
            let data = handlebars.render("tpl", &json!({"world": "Unknown"}))?;
            std::fs::write(output_file, data);
        }

        Ok(())
    }
}

fn main() -> Result<(), Box<Error>> {
    env_logger::init();

    cli_handlebars::init();

    Ok(())
}

