use std::fs::File;
use std::io::Error;
use std::io::prelude::*;

fn main() {
    let path = "src/day-06.txt";
    let input = open_file(path).unwrap();
    let race = parse_input(&input);

    let wins = num_ways_to_win(race.time, race.distance);
    println!("wins = {}", wins);
}

#[derive(Debug, PartialEq)]
struct Race {
    time: u64,
    distance: u64,
}

impl Race {
    fn from(time: u64, distance: u64) -> Self {
        Self { time, distance }
    }
}

fn concat_numbers(nums: &Vec<u64>) -> u64 {
    let mut string = String::new();
    for num in nums {
        string.push_str(&num.to_string());
    }
    if let Ok(num) = string.parse::<u64>() {
        num
    } else {
        0
    }
}

fn parse_input(input: &str) -> Race {
    let mut lines: Vec<Vec<u64>> = Vec::new();
    for line in input.lines() {
        let values: Vec<u64> = line
            .split(":")
            .last()
            .unwrap()
            .split_whitespace()
            .map(|elm| elm.parse::<u64>().unwrap())
            .collect();
        lines.push(values);
    }
    let times: Vec<u64> = Vec::from_iter(lines.first().unwrap().into_iter().map(|&x| x));
    let time = concat_numbers(&times);
    let distances: Vec<u64> = Vec::from_iter(lines.last().unwrap().into_iter().map(|&x| x));
    let distance = concat_numbers(&distances);
    let race = Race::from(time, distance);
    race
}

fn time_and_charge_to_distance(total_time: u64, charge_time: u64) -> u64 {
    let remaining_time = total_time - charge_time;
    remaining_time * charge_time
}

fn time_to_distances(total_time: u64, given_distance: u64) -> Vec<u64> {
    let mut possible_distances: Vec<u64> = Vec::new();
    for charge_time in 0..=total_time {
        let distance = time_and_charge_to_distance(total_time, charge_time);
        if distance > given_distance {
            possible_distances.push(distance);
        }
    }

    possible_distances
}

fn num_ways_to_win(total_time: u64, given_distance: u64) -> usize {
    let distances = time_to_distances(total_time, given_distance);
    distances.len()
}

fn open_file(path: &str) -> Result<String, Error> {
    let mut input_file = File::open(path)?;
    let mut input_string = String::new();
    input_file.read_to_string(&mut input_string)?;
    Ok(input_string)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_parse_input() {
        let file = include_str!("../../../src/day-06.small.txt");
        let race = Race {
            time: 71530,
            distance: 940200,
        };
        assert_eq!(parse_input(file), race);
    }

    #[test]
    fn test_time_and_charge_to_distance() {
        assert_eq!(time_and_charge_to_distance(7, 0), 0);
        assert_eq!(time_and_charge_to_distance(7, 1), 6);
        assert_eq!(time_and_charge_to_distance(7, 2), 10);
        assert_eq!(time_and_charge_to_distance(7, 3), 12);
        assert_eq!(time_and_charge_to_distance(7, 4), 12);
        assert_eq!(time_and_charge_to_distance(7, 5), 10);
        assert_eq!(time_and_charge_to_distance(7, 6), 6);
        assert_eq!(time_and_charge_to_distance(7, 7), 0);
    }

    #[test]
    fn test_time_to_distances() {
        assert_eq!(time_to_distances(7, 9), vec![10, 12, 12, 10]);
    }

    #[test]
    fn test_num_ways_to_win() {
        assert_eq!(num_ways_to_win(7, 9), 4);
        assert_eq!(num_ways_to_win(15, 40), 8);
        assert_eq!(num_ways_to_win(30, 200), 9);
        assert_eq!(num_ways_to_win(71530, 940200), 71503);
    }

    #[test]
    fn test_concat_numbers() {
        assert_eq!(concat_numbers(&vec![1, 2, 3]), 123);
        assert_eq!(concat_numbers(&vec![7, 15, 30]), 71530);
        assert_eq!(concat_numbers(&vec![9, 40, 200]), 940200);
    }
}
