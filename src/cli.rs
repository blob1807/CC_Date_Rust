
use crate::date::CCDate;
use crate::until::{date_eval, HELP, VALID_DECIMAL_FORMAT, VALID_DIGITS_FORMAT, VALID_STRING_FORMAT};

pub fn cli(args: &Vec<String>) -> String {
    let mode: String = args[1].to_owned();

    if mode == "--help" || mode == "-h" || args.len() < 3 {
        return format!("{HELP}");
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

        if args[2] == "add" {
            for date in args[4..].into_iter() {
                match date_eval(date) {
                    Ok(a) => work += a.to_decimal(),
                    Err(e) => return e
                }
            };
        }
        else if args[2] == "sub" {
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
        return format!("{}", work);

    }
    else if mode == "--string" || mode == "-s" {
        match date_eval(&args[2]) {
            Ok(a) => return format!("{}", a.to_string()),
            Err(e) => return e
        };
    }
    else if mode == "--decimal" || mode == "-de"{
        match date_eval(&args[2]) {
            Ok(a) => return format!("{}", a.to_decimal()),
            Err(e) => return e
        };
    }
    else if mode == "--digits" || mode == "-di" {
        match date_eval(&args[2]) {
            Ok(a) => return format!("{:?}", a.to_digits()),
            Err(e) => return e
        };
    }
    else if mode == "valid" {
        if      args[2] == "string" {return format!("{}", VALID_STRING_FORMAT)}
        else if args[2] == "digits" {return format!("{}", VALID_DIGITS_FORMAT)}
        else if args[2] == "decimal" {return format!("{}", VALID_DECIMAL_FORMAT)}
        else if args[2] == "all" {return format!("{}\n{}\n{}", VALID_STRING_FORMAT, VALID_DIGITS_FORMAT, VALID_DECIMAL_FORMAT)}
        else {return format!("Invalid type: {}", args[1])}
    }
    else {
       return format!("error: {} is an invalid argument.\n{}", mode, HELP)
    };


}