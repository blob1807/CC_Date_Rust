
#![allow(dead_code)]
#![allow(unused_variables, unused_mut, unused_imports)]

use lazy_static::lazy_static;
use regex::Regex;


use crate::date::CCDate;

pub const HELP: &str = "
Help:
    --help (-h) [Default]
        Shows this.
    --console (-c)
        Lanches the Console Tool.
    
    --string  (-s)  <date:(str or array[i32;5] or i64 int)>
        Converts a given date into String format.
    --decimal (-de) <date:(str or array[i32;5] or i64 int)>
        Converts a given date into Decimal format.
    --digits  (-di) <date:(str or array[i32;5] or i64 int)>
        Converts a given date into Digits format.

    --math (-m) <operation: add | sub> <date:(str or array[i32;5] or i64 int)> ...
        Lets you Add & Sub any number of dates.
    
    --valid (-v) <type: all | string | decimal | digits>
        Shows all the vaild formts of the given Type.
";

pub const VALID_STRING_FORMAT: &str = "
!1234 aAa 123 | !1234aAa123 
1234 aAa 123 | 1234aAa123
1234 A 123 | A 123 | 1234 A 
1234A123 | A123 | 1234A 
234123 | aAa";

pub const VALID_DIGITS_FORMAT: &str = "[0-2147483647, 0-25, 0-25, 0-25, 0-999] \n(0-2147483647, 0-25, 0-25, 0-25, 0-999)";
pub const VALID_DECIMAL_FORMAT: &str = "0-9223372036854775807";

lazy_static! {
    pub static ref RE_DATE: Regex = Regex::new(r"!?(?P<year>\d+) ?(?P<month>[a-zA-Z]{3}) ?(?P<day>\d{3})").unwrap();
}
lazy_static! {
    pub static ref RE_GREEDY: Regex = Regex::new(r"(?P<year>\d*) *(?P<month>[a-zA-Z]+) *(?P<day>\d*)").unwrap();
}


pub fn date_eval(date_: &String) -> Result<CCDate, String> {
    if date_.contains('[') || date_.contains(']') ||
        date_.contains('(') || date_.contains(')') {
            let work: String = date_.
                replace("[", "").
                replace("]", "").
                replace("(", "").
                replace(")", "");
            let work: Vec<&str> = work.split(",").collect();

            let mut temp: Vec<i32> = Vec::new();
            for i in work {
                let t: i32 = match i.parse() {
                    Ok(a) => a,
                    Err(_) => return Err(format!("Unable to parse date {}.", date_))
                };
                temp.push(t);
            };
            match norm_digits(&temp) {
                Ok(a) => Ok(CCDate::from_digits(&a)),
                Err(_) => Err(format!("Unable to parse date {}.", date_))
            }
    }
    else {
        match date_.trim().parse::<i64>() {
            Ok(a) => Ok(CCDate::from_decimal(&a)),
            Err(_) => {
                match norm_string(&date_) {
                    Some(a) => Ok(CCDate::from_string(&a)),
                    None => Err(format!("Unable to parse date {}.", date_))
                }
            }
        }
    }
}


pub fn norm_digits(date_: &Vec<i32>) ->  Result<[i32;5], String> {
    let mut date_: Vec<i32> = date_.to_owned();
    if date_.len() < 5 {
        date_.reverse();
        date_.resize(5, 0);
        date_.reverse();
    }
    if date_.len() > 5 {
        let base26: Vec<char> = "0123456789abcdefghijklmnop".chars().collect();
        let h: i32 = {
            let mut out: String = String::new();
            for (pos, c) in date_[1..date_.len()-4].iter().enumerate(){
                if *c > 25 {
                    return Err(format!("Num {} at pos {} is bigger than 25", c, pos));
                }
                else { out.push(base26[*c as usize]); }
            }
            out.push('0');
            match i32::from_str_radix(out.as_str(), 26) {
                Ok(a) => a,
                Err(e) => return Err(e.to_string())
            }
        };
        date_[0] = date_[0] + h;
        date_.drain(1..date_.len()-4);
    }
    Ok(date_.try_into().unwrap())
}

pub fn norm_string(date_: &String) -> Option<String> {
    match RE_GREEDY.captures(&date_) {
        // Some(a) => Some(format!("!0{} {} 0{}", &a["year"], &a["month"], &a["day"])),
        Some(a) => Some(format!("!{} {} {}", 
            if a["year"].is_empty() {"0"} else {&a["year"]},
            {
                let mut temp = a["month"].to_owned();
                while temp.len() < 3 {
                    temp = "a".to_string() + &temp
                };
                temp.to_uppercase()
            },
            if a["day"].is_empty() {"0"} else {&a["day"]}
        )),
        None => None
    }
}