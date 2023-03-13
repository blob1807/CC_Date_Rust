#![allow(dead_code)]
#![allow(unused_variables, unused_mut, unused_imports)]

use std::env;
use std::io;

mod date;
use date::CCDate;

const HELP: &str = "
Help:
    --help (-h) [Default]
        Prints this.
    
    --string (-s) <date:(str or array[i32;5] or i64 int)>
        Converts a given date into String format.
    --decimal (-de) <date:(str or array[i32;5] or i64 int)>
        Converts a given date into Decimal format.
    --digits (-di) <date:(str or array[i32;5] or i64 int)>
        Converts a given date into Digits format.

    --math (-m) <operation> <date:(str or array[i32;5] or i64 int)> ...
        Lets you Add & Sub any number of dates.
    
    --console (-c)
";

fn eval(date: &String) -> Result<CCDate, String> {
    if date.contains('[') || date.contains(']') ||
        date.contains('(') || date.contains(')') {
            let work: String = date.
                replace("[", "").
                replace("]", "").
                replace("(", "").
                replace(")", "");
            let work: Vec<&str> = work.split(",").collect();

            let mut temp: Vec<i32> = Vec::new();
            for i in work {
                let t: i32 = match i.parse() {
                    Ok(a) => a,
                    Err(_) => return Err(format!("Unable to parse given digits {}.", date))
                };
                temp.push(t);
            };
            match temp.try_into() {
                Ok(a) => Ok(CCDate::from_digits(&a)),
                Err(_) => Err(format!("Unable to parse given digits {}.", date))
            }
    } else {
        match date.trim().parse::<i64>() {
            Ok(a) => Ok(CCDate::from_decimal(&a)),
            Err(_) => Ok(CCDate::from_string(&date.replace("'", "")))
        }
    }
}

fn cli(args: &Vec<String>) -> String {
    let mode: String = args[1].to_owned();

    if mode == "--help" || mode == "-h" {
        return format!("{HELP}");
    }
    else if mode == "--math" || mode == "-m" {
        if args.len() < 5 {
            return format!("{}", "You need more then 1 date to do math.".to_string());
        };

        let sub_mode: String = args[2].to_owned();
        let mut work: i64 = {
            match eval(&args[3]) {
                Ok(a) => a.to_decimal(),
                Err(e) => return e
            }
        };

        if sub_mode == "add" {
            for date in args[4..].into_iter() {
                match eval(date) {
                    Ok(a) => work += a.to_decimal(),
                    Err(e) => return e
                }
            };
        } else if sub_mode == "sub" {
            for date in args[4..].into_iter() {
                match eval(date) {
                    Ok(a) => work -= a.to_decimal(),
                    Err(e) => return e
                }
            };
        } else {
            return format!("{} is an invalid argument.\n{}", sub_mode, HELP)
        };
        return format!("{}", work);

    } else if mode == "--string" || mode == "-s" {
        match eval(&args[2]) {
            Ok(a) => return format!("{}", a.to_string()),
            Err(e) => return e
        };
    } else if mode == "--decimal" || mode == "-de"{
        match eval(&args[2]) {
            Ok(a) => return format!("{}", a.to_decimal()),
            Err(e) => return e
        };
    } else if mode == "--digits" || mode == "-di" {
        match eval(&args[2]) {
            Ok(a) => return format!("{:?}", a.to_digits()),
            Err(e) => return e
        };
    } else {
       return format!("error: {} is an invalid argument.\n{}", mode, HELP)
    };


}

fn console() {
    loop {
        break;
    }
}

fn main() {
    // let args: Vec<String> = env::args().collect();
    // let args: Vec<String> = "cc_date.exe -s '!123abc456'".split_whitespace().map(|a| a.to_string()).collect();
    let args: Vec<String> = vec!["cc_date.exe".to_string(), "-m".to_string(), "add".to_string()];
    println!("{:?}", args);

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
}