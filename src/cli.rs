
use crate::util::{date_eval, VALID_DECIMAL_FORMAT, VALID_DIGITS_FORMAT, VALID_STRING_FORMAT};


pub const HELP: &str = "
Help:
    --help (-h) [Default]
        Shows this.
    --repl (-r)
        Lanches an Interactive Tool.
    
    --string  (-s)  <date:(str or array[i32;5] or i64 int)>
        Converts a given date into String format.
    --decimal (-de) <date:(str or array[i32;5] or i64 int)>
        Converts a given date into Decimal format.
    --digits  (-di) <date:(str or array[i32;5] or i64 int)>
        Converts a given date into Digits format.

    --math (-m) <operation: add (+) | sub (-)> <date:(str or array[i32;5] or i64 int)> ...
        Lets you Add & Sub any number of dates.
    
    --valid (-v) <type: all | string | decimal | digits>
        Shows all the vaild formts of the given Type.
";

pub fn cli(args: &Vec<String>) -> String {
    let mode: String = args[1].to_owned();
    let out: String;

    if mode == "--help" || mode == "-h" || args.len() < 3 {
        out = format!("{HELP}");
    }
    else if mode == "--math" || mode == "-m" {
        if args.len() < 5 {
            return format!("{}", "You need more then 1 date to do math.");
        };

        let mut work: i64 = {
            match date_eval(&args[3]) {
                Ok(a) => a.to_decimal(),
                Err(e) => return e
            }
        };

        if args[2] == "add" || args[2] == "+" {
            for date in args[4..].into_iter() {
                match date_eval(date) {
                    Ok(a) => work += a.to_decimal(),
                    Err(e) => return e
                }
            };
        }
        else if args[2] == "sub" || args[2] == "-" {
            for date in args[4..].into_iter() {
                match date_eval(date) {
                    Ok(a) => work -= a.to_decimal(),
                    Err(e) => return e
                }
            };
        }
        else {
            return format!("{} is an invalid argument.\n{}", args[2], HELP)
        };
        out = format!("{}", work);

    }
    else if mode == "--string" || mode == "-s" {
        match date_eval(&args[2]) {
            Ok(a) => out = format!("{}", a.to_string()),
            Err(e) => return e
        };
    }
    else if mode == "--decimal" || mode == "-de"{
        match date_eval(&args[2]) {
            Ok(a) => out = format!("{}", a.to_decimal()),
            Err(e) => return e
        };
    }
    else if mode == "--digits" || mode == "-di" {
        match date_eval(&args[2]) {
            Ok(a) => out = format!("{:?}", a.to_digits()),
            Err(e) => return e
        };
    }
    else if mode == "valid" {
        out = {
            if      args[2] == "string"  {format!("{}", VALID_STRING_FORMAT)}
            else if args[2] == "digits"  {format!("{}", VALID_DIGITS_FORMAT)}
            else if args[2] == "decimal" {format!("{}", VALID_DECIMAL_FORMAT)}
            else if args[2] == "all"     {format!("{}\n{}\n{}", VALID_STRING_FORMAT, VALID_DIGITS_FORMAT, VALID_DECIMAL_FORMAT)}
            else                         {format!("Invalid type: {}", args[1])}
        }
    }
    else {
        out = format!("error: {} is an invalid argument.\n{}", mode, HELP)
    };

    return out
}