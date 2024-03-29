# Cosmic Critter Date Format Handler - Rust

This is a Rust reimplementation of my orginal Python version, found here [CC Date](https://github.com/blob1807/cc_date). Made to be an executable with the addition of the Interactive (REPL) Tool.  

## Terminology
* String   - !1234 ABC 123
* Digits   - (1234, 0, 1, 2, 123)
* Decimal  - 21688812123

## Format
Cosmic Critters (CC) format is: `!YYYY MMM DDD`, `!0-inf A-Z*3 0-999`  
*It's unknown what CC Dates represent. Year, Month & Day are placeholders.*
* String is in CC format
* Digits is the letters converted to their place in the alphabet
* Decimal is the date converted down to Days/base10

## Help
```
CLI:
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

Interactive (REPL) Tool:
    help        - Shows this help
    usage       - Shows Tool usage examples
    quit/exit   - Closes Tool
    clear/cls   - Clears Screen
    var/vars    - Shows all created Variables
    valid       
        string  - Shows all Vaild String formats
        digits  - Shows all Vaild Digits formats
        decimal - Shows all Vaild Decimal formats
        all     - Shows all the above type

    string      - Returns following value/s in string format
    digits      - Returns following value/s in digits format
    decimal     - Returns following value/s in decimal format
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

Usage:
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

    "3 i 999"-568
    !3 AAI 431

    ans
    !3 AAI 431

    d1 = ans
    !3 AAI 4311

    vars
    The saved arguments are:
    {"ans": 52736431, "d1": 52736431}
```

## Compiling
If you don't want to use the provided binaries or don't use Windows.  
You can Compile the program by doing the folllowing:  
* Download & Install Rust from the instrution here https://www.rust-lang.org/tools/install  
* Open cmd/terminal to the root dirctory `./CC_Date_Rust/`  
* Run `cargo build --release` for standard Release Build (recommned)  
* Run `cargo build` for standard Debug Build
* Your exe can now be found in `./CC_Date_Rust/target/[build type]`