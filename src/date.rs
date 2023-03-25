#![allow(dead_code)]
#![allow(unused_variables, unused_mut, unused_imports)]

use crate::until::{norm_digits, norm_string};

const LETTERS_UPPER: [char;26] = [
    'A','B','C','D','E','F','G','H','I','J','K','L','M',
    'N','O','P','Q','R','S','T','U','V','W','X','Y','Z'];

fn letter_match(letter: char) -> Option<i32> {
    match letter {
        'A'=>Some(0),
        'B'=>Some(1),
        'C'=>Some(2),
        'D'=>Some(3),
        'E'=>Some(4),
        'F'=>Some(5),
        'G'=>Some(6),
        'H'=>Some(7),
        'I'=>Some(8),
        'J'=>Some(9),
        'K'=>Some(10),
        'L'=>Some(11),
        'M'=>Some(12),
        'N'=>Some(13),
        'O'=>Some(14),
        'P'=>Some(15),
        'Q'=>Some(16),
        'R'=>Some(17),
        'S'=>Some(18),
        'T'=>Some(19),
        'U'=>Some(20),
        'V'=>Some(21),
        'W'=>Some(22),
        'X'=>Some(23),
        'Y'=>Some(24),
        'Z'=>Some(25),
        _ => None
    }
}


#[derive(Debug, Clone, Copy)]
pub struct CCDate { date: i64,}

impl Default for CCDate {
    fn default() -> Self {
        CCDate { date: 0 }
    }
}

impl CCDate {
    pub fn new() -> Self {
        CCDate::default()
    }
    pub fn from_string<'a >(date: &'a String) -> Self {
        CCDate { date: string_to_decimal(&date) }
    }
    pub fn from_decimal<'a >(date: &'a i64) -> Self {
        CCDate { date: date.to_owned() }
    }
    pub fn from_digits<'a >(date: &'a [i32;5]) -> Self {
        CCDate { date: digits_to_decimal(&date) }
    }

    pub fn to_string(&self) -> String {
        decimal_to_string(&self.date)
    }
    pub fn to_decimal(&self) -> i64 {
        self.date.to_owned()
    }
    pub fn to_digits(&self) -> [i32; 5] {
        decimal_to_digits(&self.date)
    }

}

fn string_to_decimal(date: &String) -> i64 {
    digits_to_decimal(&string_to_digits(&date))
}
fn string_to_digits(date: &String) -> [i32;5] {
    let date = match norm_string(date) {
        Some(a) => a,
        None => panic!("Unable to normalize Date.")
    };
    let date: Vec<&str> = date.strip_prefix("!").unwrap().split_whitespace().collect();
    let mut work: Vec<String> = Vec::new();

    for i in date {work.push(i.to_string());}

    let mut out: [i32;5] = [
        work[0].parse().unwrap_or(0),
        0, 0, 0,
        work[2].parse().unwrap_or(0)];

    for (pos, letter) in work[1].to_uppercase().chars().enumerate() {
        out[1+pos] = match letter_match(letter) {
            Some(a) => a,
            None => 0
        };
    };
    out
}

fn decimal_to_string(date: &i64) -> String {
    digits_to_string(&decimal_to_digits(&date))
}
fn decimal_to_digits(date: &i64) -> [i32;5] {
    let mut date: i64 = date.to_owned();
    let mut out: [i32;5] = [0;5];

    for (p, r) in [1000, 26, 26, 26].iter().enumerate() {
        out[p] = {date % r} as i32;
        date /= r
    };
    out[4] = date as i32;
    out.reverse();
    out
}

fn digits_to_string(date: &[i32;5]) -> String {
    format!(
        "!{} {}{}{} {}",
            date[0],
            LETTERS_UPPER[date[1] as usize],
            LETTERS_UPPER[date[2] as usize],
            LETTERS_UPPER[date[3] as usize],
            date[4]
        )
}
fn digits_to_decimal(date: &[i32;5]) -> i64 {
    let mut out: i64 = 0;
    for (w, n) in [17576000, 676000, 26000, 1000, 1].iter().zip(date.to_owned()) {
        out += w * i64::from(n)
    };
    out
}