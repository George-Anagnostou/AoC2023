use std::fs::File;
use std::io::Error;
use std::io::prelude::*;

fn main() {
    let path = "src/day-06.txt";
    let input = open_file(path).unwrap();
    let races = parse_input(&input);

    let mut wins = 1;
    for race in races {
        let num_ways_to_win = num_ways_to_win(race.time, race.distance);
        wins *= num_ways_to_win;
    }
    println!("wins = {}", wins);
}

#[derive(Debug)]
struct Race {
    time: u32,
    distance: u32,
}

impl Race {
    fn from(time: u32, distance: u32) -> Self {
        Self { time, distance }
    }
}

fn parse_input(input: &str) -> Vec<Race> {
    let mut lines: Vec<Vec<u32>> = Vec::new();
    for line in input.lines() {
        let values: Vec<u32> = line
            .split(":")
            .last()
            .unwrap()
            .split_whitespace()
            .map(|elm| elm.parse::<u32>().unwrap())
            .collect();
        lines.push(values);
    }
    let times = lines.first().unwrap().into_iter();
    let distances = lines.last().unwrap().into_iter();
    let time_distance_zip = times.zip(distances);
    let mut races: Vec<Race> = Vec::with_capacity(time_distance_zip.len());
    for (time, distance) in time_distance_zip {
        let race = Race::from(*time, *distance);
        races.push(race);
    }

    races
}

fn time_and_charge_to_distance(total_time: u32, charge_time: u32) -> u32 {
    let remaining_time = total_time - charge_time;
    remaining_time * charge_time
}

fn time_to_distances(total_time: u32, given_distance: u32) -> Vec<u32> {
    let mut possible_distances: Vec<u32> = Vec::new();
    for charge_time in 0..=total_time {
        let distance = time_and_charge_to_distance(total_time, charge_time);
        if distance > given_distance {
            possible_distances.push(distance);
        }
    }

    possible_distances
}

fn num_ways_to_win(total_time: u32, given_distance: u32) -> usize {
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

    // #[test]
    // fn test_find_num_winning_strategies() {
    //     let path = "../src/day-06.small.txt";
    //     let input = open_file(path).unwrap();
    //     assert_eq!(find_num_winning_strategies(&input), 288);
    // }

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
    }
}
