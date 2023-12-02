use std::fs::File;
use std::io::{BufReader, BufRead, Error};
use std::io::prelude::*;

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
        sum += num;
    }
    sum
}

fn parse_calibration_value(line: &str) -> i32 {
    let first_digit = line.chars().find(|x| x.is_digit(10)).unwrap();
    let last_digit = line.chars().rfind(|x| x.is_digit(10)).unwrap();

    let mut num = String::new();
    num.push(first_digit);
    num.push(last_digit);
    num.parse::<i32>().unwrap()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_parse_calibration_value() {
        assert_eq!(92, parse_calibration_value("nine92jnhgqzctpgbcbpz"));
        assert_eq!(73, parse_calibration_value("sevensddvc73three"));
        assert_eq!(98, parse_calibration_value("9986fmfqhdmq8"));

        assert_ne!(12, parse_calibration_value("one4seven5two"));
    }
}
