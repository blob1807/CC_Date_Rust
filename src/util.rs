
use std::num::ParseIntError;

use lazy_static::lazy_static;
use regex::Regex;

use crate::date::CCDate;


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
                Ok(a) => Ok(CCDate::from_digits(&a.try_into().unwrap())),
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

pub fn norm_digits(date_: &Vec<i32>) ->  Result<[i32;5], ParseIntError> {
    let mut date_: Vec<i32> = date_.to_owned();
    if date_.len() < 5 {
        date_.reverse();
        date_.resize(5, 0);
        date_.reverse();
    }
    if date_.len() > 5 {
        /*
        [1, 32, 2, 3, 4, 5, 6] -> [2, 6, 2, 3, 4, 5, 6]
        [2, 6, 2, 3, 4, 5, 6] -> [2, 623, 4, 5, 6] -> [2, 3, 4, 5, 6], 620
        base26 -> base10: 620 -> 4108
        [2+4108, 3, 4, 5, 6] -> [4110, 3, 4, 5, 6]
        */
        let base26: Vec<char> = "0123456789abcdefghijklmnop".chars().collect();
        let h: i32 = {
            let mut out: String = String::new();
            for n in (1..date_.len()-4).rev() {
                date_[n-1] += date_[n]/26;
                date_[n] = date_[n]%26;        
            }
            for n in date_[1..date_.len()-4].iter() {
                out.push(base26[*n as usize]);
            }
            out.push('0');
            i32::from_str_radix(out.as_str(), 26)?
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