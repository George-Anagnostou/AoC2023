use regex::Regex;
use std::collections::HashMap;
use std::fs::File;
use std::io::prelude::*;
use std::io::Error;
fn main() {
    let path = "src/day-03.txt";
    let input = open_path(path).unwrap();
    let gear_ratio_sum = sum_gear_ratios(&input);
    println!("{}", gear_ratio_sum);
}

fn open_path(path: &str) -> Result<String, Error> {
    let mut input_file = File::open(path)?;
    let mut input_string = String::new();
    input_file.read_to_string(&mut input_string)?;
    Ok(input_string)
}

#[derive(Hash, Eq, PartialEq, Debug)]
struct Coordinate {
    point: (usize, usize),
    character: char,
}

impl Coordinate {
    pub fn from(point: (usize, usize), s: char) -> Self {
        Self {
            point,
            character: s,
        }
    }
}

fn sum_gear_ratios(input: &str) -> usize {
    let board: Vec<String> = input.lines().map(|line| line.trim().to_string()).collect();

    let regex = Regex::new(r"\d+").unwrap();
    let mut gear_nums: HashMap<Coordinate, Vec<usize>> = HashMap::new();

    for (row, line) in board.iter().enumerate() {
        for found in regex.find_iter(&line) {
            let num = found.as_str().parse::<usize>().unwrap();

            let start_y = row.checked_sub(1).unwrap_or(0);
            let end_y = row.checked_add(1).unwrap_or(row);
            let start_x = found.start().checked_sub(1).unwrap_or(0);
            let end_x = found.end();

            for i in start_y..=end_y {
                for j in start_x..=end_x {
                    if let Some(c) = &board.get(i).and_then(|line| line.chars().nth(j)) {
                        if *c == '*' {
                            gear_nums
                                .entry(Coordinate::from((i, j), c.to_owned()))
                                .or_insert(Vec::new())
                                .push(num);
                        }
                    }
                }
            }
        }
    }

    let gear_ratio_sum: usize = gear_nums
        .into_iter()
        .filter(|(_, v)| v.len() == 2)
        .map(|(_, v)| v.first().unwrap() * v.last().unwrap())
        .sum();

    gear_ratio_sum
}
