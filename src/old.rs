fn calc_radixes(date: &u64) -> [usize; 5] {
    [10usize.pow((date.to_string().len()).try_into().unwrap()), 26, 26, 26, 1000]
}
fn calc_weights(rads: &[usize; 5]) -> [usize; 5] {
    [rads[1] * 676000, 676000, 26000, 1000, 1]
}

fn string_to_digits(date: &String) -> [i32;5] {
    let mut out: [i32;5] = [0, 0, 0, 0, 0];
    out[1+pos] = match LETTERS_UPPER.iter().position(|&x| x == letter) {
        Some(a) => a as i32,
        None => 0
    }
}

fn eval(date: &String) -> Option<DateTypes> {
    if date.contains('[') || date.contains(']') {
        let work = date.
            replace("[", "").
            replace("]", "").
            split(",").collect::<Vec<&str>>().
            iter().map(|x| x.parse().unwrap_or(0)).
            collect::<Vec<i32>>();

        match <Vec<i32> as TryInto<[i32;5]>>::try_into(work) {
                Ok(a) => return Some(DateTypes::Digits(a)),
                Err(_) => return None
            };
        
    if date.contains('[') || date.contains(']') ||
    date.contains('(') || date.contains(')') {
        let work = 
    match date.
        replace("[", "").
        replace("]", "").
        replace("(", "").
        replace(")", "").
        split(",").
        collect::<Vec<&str>>().
        iter().
        map(|x| x.parse().unwrap_or(0)).
        collect::<Vec<i32>>().
        try_into() {
            Ok(a) => return Some(DateTypes::Digits(a)),
            Err(_) => return None
        };
        };
    }}

fn main() {
    let date = "[]".to_string();

    let work: Option<DateTypes> = match date.
            replace("[", "").
            replace("]", "").
            split(",").collect::<Vec<&str>>().
            iter().map(|x| x.parse().unwrap_or(0)).
            collect::<Vec<i32>>().try_into() {
                Ok(a) => Some(DateTypes::Digits(a)),
                Err(_) =>  None
            };
}

/*let date_dig: [i32;5] = [0,1,2,3,4];
    let date_dec: i64 = 731004;
    let date_str: String = "!0 BCD 4".to_string();

    let date: CCDate = CCDate::from_string(&"!123 DEF 789".to_string());

    println!("{:?}, {:?}", date_dig, digits_to_string(&date_dig));
    println!("{:?}, {:?}", date_dig, digits_to_decimal(&date_dig));
    println!("{:?}, {:?}", date_dec, decimal_to_digits(&date_dec));
    println!("{:?}, {:?}", date_dec, decimal_to_string(&date_dec));
    println!("{:?}, {:?}", date_str, string_to_digits(&date_str));
    println!("{:?}, {:?}", date_str, string_to_decimal(&date_str));

    println!("{:?}, {:?}", date, date.to_decimal());
    println!("{:?}, {:?}", date, date.to_digits());
    println!("{:?}, {:?}", date, date.to_string());

let t: Vec<String> = vec![
        "731004".to_string(),
        "'!0 BCD 4'".to_string(),
        "[0,1,2,3,4]".to_string()];

    let work: (i64, String, [i32;5]) =  (
        t[0].trim().parse().unwrap_or(0),
        t[1].trim().replace("'", ""),
        t[2].trim().
            replace("[", "").
            replace("]", "").
            split(",").collect::<Vec<&str>>().
            iter().map(|x| x.parse().unwrap_or(0)).
            collect::<Vec<i32>>().try_into().unwrap()
    );

    let w = "'!0 BCD 4'".to_string();

    println!("{:?}", w.replace("'", ""));

    let mut g: CCDate = CCDate::from_string(&"!0 BCD 4".to_string());

    println!("{:?}", g.to_decimal() + 65);*/
