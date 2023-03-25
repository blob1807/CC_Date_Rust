#![allow(dead_code)]
#![allow(unused_variables, unused_mut, unused_imports)]

use std::env;
use regex::Regex;

mod date;
mod console;
mod until;
mod cli;

use console::console;
use date::CCDate;
use until::HELP;
use cli::cli;


fn main() {
    // let args: Vec<String> = env::args().collect();
    let args: Vec<String> = vec!["cc_date.exe".to_string(), "--console".to_string()];
    // let args: Vec<String> = vec!["cc_date.exe".to_string(), "--string".to_string(), "[123]".to_string()];
    // println!("{:?}", args);

    match args.is_empty() {
        true => {
            println!("error: Something went wrong. Input args where empty. \n{}", HELP);
            return;
        },
        false => true
    };

    match args.len() {
        1 => {
            println!("{HELP}");
        },
        _ => {
            if args[1] == "--console" || args[1] == "-c" { console() }
            else { println!("{}", cli(&args)) }
        }
    };

    // let t = CCDate::from_string(&"b".to_string());
}