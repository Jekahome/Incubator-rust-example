

#[macro_use(values_t,value_t,crate_version,crate_authors)]
extern crate clap;
extern crate handlebars;
#[macro_use]
extern crate serde_json;
extern crate env_logger;
extern crate serde;
extern crate crossbeam;

use clap::{App, Arg,ArgMatches};
use handlebars::Handlebars;
use std::collections::btree_map::BTreeMap;
use std::error::Error;
use std::path::Path;

use std::{env, io};
//use std::borrow::Cow;

use std::fs::File;
use std::io::Read;
use std::thread;

use load_files::*;



mod load_files{
    use super::*;

    #[derive(Debug)]
    pub struct Settings {
        pub max_threads: u8,
        pub file: String,
    }

    fn has_file(file: String) -> Result<(), String> {
        if Path::new(&file).exists() {
            return Ok(());
        }
        Err(String::from("The file notfound"))
    }
    fn get_matches<'a>() -> ArgMatches<'a> {

        App::new("Load files CLI")
            .usage("MyApp [--max-threads = <number>] <file>")
            .bin_name("MyApp")
            .version(crate_version!())
            .author(crate_authors!())
            .about("Load files")
            .args(&[
                Arg::with_name("file")
                    .validator(has_file)
                    .required(true)
                    .help("Load files"),
                Arg::with_name("max-threads")
                    .long("max-threads")
                    .value_name("number")
                    .required(false)
                    .help("thread number"),
            ])
            .get_matches()
    }


    pub fn new() ->   Settings {

        let matches = get_matches();

        let file = matches.value_of("file").unwrap_or("download");

        let max_threads:u8 = value_t!(matches, "max-threads", u8).unwrap_or(4);

        Settings{file:file.to_string() ,max_threads:max_threads}

    }
}

#[derive(Debug)]
struct Task{
    url:String
}
impl Task{
    fn new(url:String)->Self{
        Task{url}
    }
}


fn main() -> Result<(), Box<std::error::Error + 'static>>{

    let settings:Settings = load_files::new();

    println!("{:?} {:?}",
             settings.file,
             settings.max_threads);



    let s:String = std::fs::read_to_string(settings.file)?;
    let mut v:Vec<Task> = vec![];
    for url in s.lines(){
        v.push(Task::new(url.to_string()));
        let url_ = url.clone();

        crossbeam::scope(|scope_| {
                  scope_.spawn(move ||{
                    // load url and create file number thread
                    println!("{}",url_);



              });
        });

    }

    for url in v{



    }



    println!("{:?}",v);



    Ok(())



}
