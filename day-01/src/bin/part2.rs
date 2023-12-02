use std::fs::File;
use std::io::Error;
use std::io::prelude::*;

use aho_corasick::AhoCorasick;

fn main() -> Result<(), Error> {
    let path = "input.txt";
    let input_string = open_path(path)?;

    let calibration_value = sum_callibrations(input_string);
    println!("{}", calibration_value);

    Ok(())
}

fn open_path(path: &str) -> Result<String, Error> {
    let mut input_file = File::open(path)?;
    let mut input_string = String::new();
    input_file.read_to_string(&mut input_string)?;
    Ok(input_string)
}

fn sum_callibrations(input: String) -> i32 {
    let mut sum = 0;
    for line in input.lines() {
        let num = parse_calibration_value(line);
        // println!("{}: {}", line, &num);
        sum += num;
    }
    sum
}

fn parse_calibration_value(line: &str) -> i32 {
    let patterns = &[
        "1", "2", "3", "4", "5", "6", "7", "8", "9",
        "one", "two", "three", "four", "five", "six", "seven", "eight", "nine",
    ];

    let ac = AhoCorasick::new(patterns).unwrap();
    let matches: Vec<usize> = ac
        .find_overlapping_iter(line)
        .map(|mat| mat.pattern().as_usize())
        .collect();

    let numbers: Vec<&str> = matches.into_iter()
        .map(|idx| patterns[idx])
        .collect();

    let first_digit = convert_string_number(numbers.first().unwrap());
    let last_digit = convert_string_number(numbers.last().unwrap());

    let num = format!("{}{}", first_digit, last_digit);
    num.parse::<i32>().unwrap()
}

fn convert_string_number(number: &str) -> &str {
    match number {
        "one" => "1",
        "two" => "2",
        "three" => "3",
        "four" => "4",
        "five" => "5",
        "six" => "6",
        "seven" => "7",
        "eight" => "8",
        "nine" => "9",
        _ => number,
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_parse_calibration_value() {
        assert_eq!(92, parse_calibration_value("nine92jnhgqzctpgbcbpz"));
        assert_eq!(72, parse_calibration_value("lkajdsf7klsdftwo"));
        assert_eq!(32, parse_calibration_value("lkjfew3seventeentwentytwo"));
        assert_eq!(43, parse_calibration_value("ljflewkfour342353seven3"));
        assert_eq!(23, parse_calibration_value("twoneeighthree"));
    }
}
