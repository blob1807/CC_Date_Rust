
use crate::util::{norm_digits, norm_string};

#[derive(Debug, Clone, Copy)]
pub struct CCDate { date: i64 }

impl CCDate {
    pub fn from_string(date: &String) -> Self {
        match norm_string(date) {
            Some(a) => {
                let date: Vec<&str> = a.strip_prefix("!").unwrap().split_whitespace().collect();
                let mut work: Vec<String> = Vec::new();
            
                for i in date {work.push(i.to_string())}
            
                let mut out: Vec<i32> = vec!(work[0].parse().unwrap_or(0));
                
                for letter in work[1].to_uppercase().chars() {
                    out.push( match "ABCDEFGFIJKLMNOPQRSTUVWXYZ".find(letter) {
                            Some(a) => a as i32,
                            None => 0
                        }
                    )
                };
                out.push(work[2].parse().unwrap_or(0));

                CCDate { date: digits_to_decimal(&norm_digits(&out).unwrap()) }
            },
            None => panic!("Unable to normalize Date.")
        }
    }
    pub fn from_decimal(date: &i64) -> Self {
        CCDate { date: date.to_owned() }
    }
    pub fn from_digits(date: &[i32;5]) -> Self {
        match norm_digits(&date.to_vec()) {
            Ok(a) => CCDate { date: digits_to_decimal(&a) },
            Err(e) => panic!("{}", e)
        }
    }

    pub fn to_string(&self) -> String {
        let date: [i32; 5] = decimal_to_digits(&self.date);
        let letters: [char; 26] = [
            'A','B','C','D','E','F','G','H','I','J','K','L','M',
            'N','O','P','Q','R','S','T','U','V','W','X','Y','Z'];
        format!(
            "!{} {}{}{} {}",
                date[0],
                letters[date[1] as usize],
                letters[date[2] as usize],
                letters[date[3] as usize],
                date[4]
            )
    }
    pub fn to_decimal(&self) -> i64 {
        self.date.to_owned()
    }
    pub fn to_digits(&self) -> [i32; 5] {
        decimal_to_digits(&self.date)
    }

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

fn digits_to_decimal(date: &[i32;5]) -> i64 {
    let mut out: i64 = 0;
    for (w, n) in [17576000, 676000, 26000, 1000, 1].iter().zip(date.to_owned()) {
        out += w * i64::from(n)
    };
    out
}