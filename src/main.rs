const FILE_PATH: &str = "test.txt";

#[derive(Debug)]
pub struct Fyi {
    pub ts: String,
    pub value: String,
}
impl Fyi {
    pub fn format(&self) -> String {
        format!("{}: {}\n", self.ts, self.value)
    }
}

#[derive(Debug)]
pub struct How {
    pub value: String,
}

mod arguments {
    extern crate chrono;
    use crate::{ Fyi, How };
    use chrono::Local;
    use std::io::Error;

    #[derive(Debug)]
    pub enum Input {
        Help(i32),
        Remember(Fyi),
        Query(How),
    }

    const SEP: &str = " ";
    pub fn gather_input(args: &Vec<String>) -> Input {
        match args.get(1) {
            Some(first_arg) => {
                if !is_command(first_arg) {
                    Input::Remember(Fyi { ts: Local::now().to_string(), value: args[1..].join(SEP) } )
                } else if is_query(args) {
                    Input::Query(How { value: args[2..].join(SEP) })
                } else if is_help(args) {
                    Input::Help(0)
                } else {
                    Input::Help(1)
                }
            },
            None => Input::Help(1),
        }
    }
    fn is_command(s: &String) -> bool { s.starts_with('-') }
    fn query_input_exists(args: &Vec<String>) -> bool { args.get(2).is_some() }
    fn is_query(args: &Vec<String>) -> bool { match args.get(1) {
        Some(first_arg) => query_input_exists(args) && (first_arg == "-q" || first_arg == "--query"),
        None => false
    } }
    fn is_help(args: &Vec<String>) -> bool { match args.get(1) {
        Some(first_arg) => first_arg == "-h" || first_arg == "--help",
        None => false
    } }
    pub fn print_help(code: i32) -> Result<(), Error> {
        println!("exit!");
        Err(Error::from_raw_os_error(code))
    }
}

mod fyi_file {
    use crate::{ How, Fyi };
    use std::fs::OpenOptions;
    use std::io::prelude::*;
    use std::io::Error;
    use std::io::BufReader;
    use crate::FILE_PATH;
    use std::fs::File;

    fn parse_entry(entry: &String) -> Option<String> {
        if entry.len() > 35 {
            let mut s: String = String::new();
            s.push_str(&entry[35..]);
            Some(s)
        }
        else { None }
    }

    pub fn save(input: &Fyi) -> Result<(), Error> {
        println!("fyi: {}", input.format());
        let mut file = OpenOptions::new()
            .write(true)
            .append(true)
            .create(true)
            .open(FILE_PATH)?;
        file.write_all(input.format().as_bytes())
    }

    fn find_match(file: File) -> Result<String, Error> {
        let nothing_found: String = "Nothing Found :(\n".to_string();
        let file = BufReader::new(&file);
        file.lines()
            .map(|entry| entry)
            .collect::<Result<Vec<String>, Error>>()
            .map(|lines| {
                lines.iter().fold(nothing_found, |out, line| {
                    match parse_entry(line) {
                        Some(l) => l,
                        None => out
                    }
                })
            })
    }

    fn print_search(r: Result<String, Error>) {
        match r {
            Ok(s) => println!("{}", s),
            Err(e) => println!("{}", e.to_string()),
        }
    }

    pub fn search(input: &How) -> Result<(), Error> {
        println!("how: {}", input.value);
        OpenOptions::new()
            .read(true)
            .open(FILE_PATH)
            .map(find_match)
            .map(print_search)
    }
}

use arguments::*;
use std::env;
use std::io::Error;
use std::process;

fn main() {
    // collect args into a Vec
    let args: Vec<String> = env::args().collect();

    // gather the input, could be 1 of 3 input types
    let input: Input = arguments::gather_input(&args);

    let outcome: Result<(), Error> = match input {
        Input::Help(code) => print_help(code),
        Input::Query(how) => fyi_file::search(&how),
        Input::Remember(fyi) => fyi_file::save(&fyi),
    };

    match outcome {
        Err(e) => {
            println!("{}", e.to_string());
            process::exit(1)
        },
        _ => process::exit(0),
    }
}
