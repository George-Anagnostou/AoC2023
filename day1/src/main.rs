use std::fs::File;
use std::io::{BufReader, BufRead, Error};

fn main() -> Result<(), Error> {
    let path = "input.txt";
    let input = File::open(path)?;
    let buffered = BufReader::new(input);

    let mut sum = 0;
    for line in buffered.lines() {
        let num = get_line_number(line?);
        sum += num;
    }
    println!("{}", sum);
    Ok(())
}

fn get_line_number(line: String) -> i32 {
    let first_idx = line.find(|x: char| x.is_digit(10)).unwrap();
    let first_digit = line.chars().nth(first_idx).unwrap();

    let last_idx = line.rfind(|x: char| x.is_digit(10)).unwrap();
    let last_digit = line.chars().nth(last_idx).unwrap();

    let mut sum = String::new();
    sum.push(first_digit);
    sum.push(last_digit);

    sum.parse::<i32>().unwrap()
}
