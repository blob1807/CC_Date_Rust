
use std::collections::HashMap;
use std::io::{self, Write, stdin, stdout, Result};

use crate::date::CCDate;
use crate::util::{date_eval, VALID_DECIMAL_FORMAT, VALID_DIGITS_FORMAT, VALID_STRING_FORMAT};


const HELP: &str = "
 help        - Shows this help
 usage       - Shows Tool usage examples
 quit/exit   - Closes Tool
 clear/cls   - Clears Screen
 var/vars    - Shows all created Variables
 valid       
     string/str  - Shows all Vaild String formats
     digits/dig  - Shows all Vaild Digits formats
     decimal/dec - Shows all Vaild Decimal formats
     all         - Shows all the above type

 string/str      - Returns following value/s in string format
 digits/dig      - Returns following value/s in digits format
 decimal/dec     - Returns following value/s in decimal format

 add         - Adds all following values
 sub         - Subs all following values

 ans         - Shows the last resault. Used if no values are given before Operators. 
               Can also be used as variable
 [date]      - If a valid Date is given by its self, then that Date in string format will be returned
 [var]       - If a valid Variable is given by its self, then that Variable in string format will be returned

 Operators:
    =        - Creates/Changes Variables. Variables can be used as a vaild date.
    +        - Adds left & right values
    -        - Subs left & right values
";

const USAGE: &str = "
 string 123 123AbC456 '123 aBc 456'
    123 -> !0 AAA 123        
    123AbC456 -> !123 ABC 456
    '123 aBc 456' -> !123 ABC 456

 digits 123 123AbC456 '123 aBc 456'
    123 -> [0, 0, 0, 0, 123]
    123AbC456 -> [123, 0, 1, 2, 456]
    123 aBc 456 -> [123, 0, 1, 2, 456]

 decimal 123 123AbC456 '123 aBc 456'
    123 -> 123
    123AbC456 -> 2161876456
    123 aBc 456 -> 2161876456

 add 123 123AbC456 '123 aBc 456'
    !246 ACF 35

 sub 123 123AbC456 '123 aBc 456' 
    Negive numbers aren't supported as a valid Date rn.
    -4323753035

 '123 abc 456' + 987efd654
    !1110 EGG 110

 \"3 i 999\"-568
    !3 AAI 431

 ans
    !3 AAI 431

 d1 = ans
    !3 AAI 4311

 vars
    The saved arguments are:
    {\"ans\": 52736431, \"d1\": 52736431}
";

fn get_date (vars: &HashMap<String, i64>, arg: &String) -> Option<CCDate> {
    match vars.get(arg) {
        Some(a) => Some(CCDate::from_decimal(a)),
        None => match date_eval(arg) {
            Ok(a) => Some(a),
            Err(_) => None
        }
    }
}

fn get_date_as_num (vars: &HashMap<String, i64>, arg: &String, lock: &mut io::StdoutLock) -> i64 {
    match get_date(vars, arg) {
        Some(a) => a.to_decimal(),
        None => {writeln!(lock, "Couldn't parse {} as a valid date.", arg).unwrap(); 0}
    }
}


pub fn repl() -> Result<()> {
    let mut lock: io::StdoutLock = stdout().lock();
    let mut vars: HashMap<String, i64> = HashMap::new();
    vars.insert("ans".to_string(), 0);

    let ops: &str = "+-=";
    let str_conts: &str = "'\"";
    writeln!(lock, "Welcome to the Cosmic Critters Date REPL Tool.\nType 'help' or 'usage' for how to use it.\n")?;

    'repl: loop {
        let mut input: String = String::new();

        write!(lock, "cc_date> ")?;
        // write!(lock, "cc-date> ")?;
        lock.flush()?;
        stdin().read_line(&mut input)?;

        if input.trim().is_empty() {
            continue;
        };

        let mut args: Vec<String> = Vec::new();
        let mut constructor: String = String::new();
        let mut is_string: bool = false;

        for c in input.trim().chars() {
            if str_conts.contains(c) {
                is_string = true;
                constructor.push(c);
            }
            else if ops.contains(c) {
                if is_string {
                    writeln!(lock, "Operator '{}' was found in a String Date starting with '{}'", c, constructor)?;
                    continue 'repl;
                }
                else if c == '=' && args.len() > 2 {
                    writeln!(lock, "Operator '=' should be the 1st or 2nd argument")?;
                    continue 'repl;
                }
                else if !constructor.is_empty() {
                    args.push(constructor.clone());
                    constructor.clear()
                }
                args.push(c.to_string())
            }
            else if c.is_whitespace() && !is_string {
                if !constructor.is_empty() {
                    args.push(constructor.clone());
                    constructor.clear()
                }
            }
            else {
                constructor.push(c);
            };
        };

        if !constructor.is_empty() { args.push(constructor.clone()); };
        //println!("{:?}", input);
        //println!("{:?}", args);

        if args[0] == "help" {
            writeln!(lock, "{}", HELP)?;
        }
        else if args[0] == "usage" {
            writeln!(lock, "{}", USAGE)?;
        }
        else if args[0] == "quit" || args[0] == "exit" {
            break 'repl
        }
        else if args[0] == "var" || args[0] == "vars"{
            writeln!(lock, "The saved arguments are:\n{:?}", vars)?
        }
        else if args[0] == "valid" {
            match args.get(1) {
                None => writeln!(lock, "No type was provided.")?,
                Some(a) => {
                     if a=="string"  || a=="str" {writeln!(lock, "{}", VALID_STRING_FORMAT)?}
                else if a=="digits"  || a=="dig" {writeln!(lock, "{}", VALID_DIGITS_FORMAT)?}
                else if a=="decimal" || a=="dec" {writeln!(lock, "{}", VALID_DECIMAL_FORMAT)?}
                else if a=="all" {writeln!(lock, "{}\n{}\n{}", VALID_STRING_FORMAT, VALID_DIGITS_FORMAT, VALID_DECIMAL_FORMAT)?}
                else {writeln!(lock, "Invalid type: {}", a)?}
                }
            }
        }

        else if args[0] == "string" || args[0] == "str"{
            for d in args[1..].iter() {
                let out: String = {
                    match get_date(&vars, &d) {
                        Some(a) => {
                            vars.insert("ans".to_string(), a.to_decimal());
                            format!("{} -> {}", d, a.to_string())
                        },
                        None => format!("{} -> Unable to convert", d)
                    }
                };
                writeln!(lock, "{}", out)?
            }
        }
        else if args[0] == "digits" || args[0] == "dig"{
            for d in args[1..].iter() {
                let out: String = {
                    match get_date(&vars, &d) {
                        Some(a) => {
                            vars.insert("ans".to_string(), a.to_decimal());
                            format!("{} -> {:?}", d, a.to_digits())
                        },
                        None => format!("{} -> Unable to convert", d)
                    }
                };
                writeln!(lock, "{}", out)?
            }
            
        }
        else if args[0] == "decimal" || args[0] == "dec"{
            for d in args[1..].iter() {
                let out: String = {
                    match get_date(&vars, &d) {
                        Some(a) => {
                            vars.insert("ans".to_string(), a.to_decimal());
                            format!("{} -> {}", d, a.to_decimal())
                        },
                        None => format!("{} -> Unable to convert", d)
                    }
                };
                writeln!(lock, "{}", out)?
            }
            
        }
        else if args[0] == "add" || args[0] == "sum" {
            let mut out: i64 = 0;
            for d in args[1..].iter() {
                let work = {
                    match get_date(&vars, &d) {
                        Some(a) => a.to_decimal(),
                        None => {writeln!(lock, "Unable to add. Invalid date '{}'", d)?; 0}
                    }
                };
                out += work;
            }
            vars.insert("ans".to_string(), out);
            writeln!(lock, "{}", CCDate::from_decimal(&out).to_string())?;
            
        }
        else if args[0] == "sub" {
            let mut out: i64 = 0;
            for d in args[1..].iter() {
                let work = {
                    match get_date(&vars, &d) {
                        Some(a) => a.to_decimal(),
                        None => {writeln!(lock, "Unable to add. Invalid date '{}'", d)?; 0}
                    }
                };
                out -= work;
            }
            vars.insert("ans".to_string(), out);
            if out < 0 {
                writeln!(lock, "Negive numbers aren't supported as a valid Date rn.\n{}", out)?;
            }
            else {
                writeln!(lock, "{}", CCDate::from_decimal(&out).to_string())?;
            }
        }
        else if args.len() == 1 {
            match get_date(&vars, &args[0]) {
                Some(a) => {
                    vars.insert("ans".to_string(), a.to_decimal());
                    writeln!(lock, "{}", a.to_string())?
                },
                None => writeln!(lock, "Couldn't parse {} as a valid date.", args[0])?
            }
        }
        else {
            let mut pos: usize = 0;
            let store: String = {
                if args[0] == "=" {
                    pos += 1;
                    String::from("ans")
                }
                else if args[1] == "=" {
                    match &args[0].parse::<usize>() {
                        Ok(_) => {
                            write!(lock, "WARNING: Are you sure you want to set '{}' as a Variable N|y? ", args[0])?;
                            lock.flush()?;
                            let mut or_check = String::new();
                            stdin().read_line(&mut or_check)?;
                            if "yes".contains(&or_check.to_lowercase().trim()) { true }
                            else { continue; }
                        },
                        Err(_) => true
                    };
                    pos += 2;
                    args[0].to_owned()
                } 
                else {
                    String::from("ans")
                }
            };
            if !ops[..2].contains(&args[0]) {
                vars.insert(store.to_owned(), get_date_as_num(&vars, &args[pos], &mut lock));
                pos += 1;
            }

            while pos < args.len() {

                if args[pos] == "+" {
                    let d: i64 = vars[&store] + get_date_as_num(&vars, &args[pos+1], &mut lock);
                    vars.insert(store.to_owned(), d);
                    // writeln!(lock, "{}", vars[&store])?;
                    pos += 2
                    
                }
                else if args[pos] == "-" {
                    let d: i64 = vars[&store] - get_date_as_num(&vars, &args[pos+1], &mut lock);
                    vars.insert(store.to_owned(), d);
                    // writeln!(lock, "{}", vars[&store])?;
                    pos += 2

                }
                else {
                    writeln!(lock, "Invaild argument: {:?}", args[pos])?;
                    pos += 1;
                };
            };
            vars.insert("ans".to_string(), vars[&store]);
            if vars["ans"] < 0 {
                writeln!(lock, "Negive numbers aren't supported as a valid Date rn.\n{}", vars["ans"])?;
            }
            else {
                writeln!(lock, "{}", CCDate::from_decimal(&vars["ans"]).to_string())?;
            }
        };
        writeln!(lock, "")?;
    }
    Ok(())
}