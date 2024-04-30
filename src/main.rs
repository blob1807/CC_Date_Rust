use std::env;

mod date;
mod repl;
mod util;
mod cli;

use repl::repl;
use cli::{cli, HELP};
use std::io::Result;


fn main() -> Result<()> {
    //let args: Vec<String> = env::args().collect();
    let args: Vec<String> = vec!["cc_date.exe".to_string(), "--repl".to_string()];
    // let args: Vec<String> = vec!["cc_date.exe".to_string(), "--string".to_string(), "[123]".to_string()];
    // println!("{:?}", args);

    match args.len() {
        0 => println!("error: Something went wrong. Input args where empty."),
        1 => println!("{HELP}"),
        _ => {
            if args[1] == "--repl" || args[1] == "-r" { 
                repl()? 
            } else { 
                println!("{}", cli(&args))
            }
        }
    };
    Ok(())
}
