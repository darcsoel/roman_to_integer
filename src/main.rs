use std::io;

fn convert(roman_number: &String) -> i64 {
    let roman_to_decimal: Vec<(&str, i64)> = vec![
        ("M", 1000),
        ("CM", 900),
        ("D", 500),
        ("CD", 400),
        ("C", 100),
        ("XC", 90),
        ("L", 50),
        ("XL", 40),
        ("X", 10),
        ("IX", 9),
        ("V", 5),
        ("IV", 4),
        ("I", 1),
    ];
    
    let mut result: i64 = 0;
    let mut index: usize = 0;

    for (roman, decimal) in &roman_to_decimal {
        loop {
            println!("Proceeding roman {roman}");

            if roman_number[index..roman_number.len()].starts_with(roman) {
                result += decimal;
                println!("Adding {decimal} to result");
                index += roman.len();
            } else {
                println!("break loop for current roman number");
                break;
            }
        }
    }

    return result;
}

fn main() {
    println!("Enter roman number to convert");

    let mut roman_number = String::new();

    io::stdin()
        .read_line(&mut roman_number)
        .expect("Failed to read line");

    let decimal = convert(&roman_number);

    println!("Entered {roman_number}");
    println!("Result: {decimal}");
}


#[cfg(test)]
mod tests {
    use crate::convert;

    #[test]
    fn check_roman_x() {
        let roman_number = String::from("X");
        let result = convert(&roman_number);
        assert_eq!(result, 10);
    }

    #[test]
    fn check_roman_ix() {
        let roman_number = String::from("IX");
        let result = convert(&roman_number);
        assert_eq!(result, 9);
    }

    #[test]
    fn check_roman_lviii() {
        let roman_number = String::from("LVIII");
        let result = convert(&roman_number);
        assert_eq!(result, 58);
    }

    #[test]
    fn check_roman_xciv() {
        let mut roman_number = String::new();
        roman_number.push_str("XCIV");

        let result = convert(&roman_number);
        assert_eq!(result, 94);
    }

    #[test]
    fn check_roman_xxvii() {
        let roman_number = String::from("XXVII");
        let result = convert(&roman_number);
        assert_eq!(result, 27);
    }

    #[test]
    fn check_roman_mcmxciv() {
        let roman_number = String::from("MCMXCIV");
        let result = convert(&roman_number);
        assert_eq!(result, 1994);
    }
}