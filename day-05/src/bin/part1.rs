use std::fs::File;
use std::io::prelude::*;
use std::io::Error;

fn main() {
    let path = "src/day-05.txt";
    let input = open_file(path).unwrap();
    let lowest_location = find_lowest_location(&input);
    println!("{}", lowest_location);
}

fn open_file(path: &str) -> Result<String, Error> {
    let mut input_file = File::open(path)?;
    let mut input_string = String::new();
    input_file.read_to_string(&mut input_string)?;
    Ok(input_string)
}

#[derive(Debug)]
struct RangeMap {
    destination_range: usize,
    source_range: usize,
    range_length: usize,
}

impl RangeMap {
    fn from(line: &str) -> Self {
        let nums: Vec<usize> = line
            .split_whitespace()
            .map(|num| num.parse::<usize>().unwrap())
            .collect();

        Self {
            destination_range: nums[0],
            source_range: nums[1],
            range_length: nums[2],
        }
    }
}

fn get_location(seed: usize, mappings: &Vec<Vec<RangeMap>>) -> usize {
    let mut num: usize = seed;
    for section in mappings {
        for row in section {
            if (row.source_range..row.source_range + row.range_length).contains(&num) {
                let diff = num - row.source_range;
                num = row.destination_range + diff;
                break;
            }
        }
    }
    num
}

fn get_almanac(parsed_input: Vec<Vec<&str>>) -> Vec<Vec<RangeMap>> {
    parsed_input
        .into_iter()
        .map(|section| {
            section
                .into_iter()
                .filter(|line| !line.is_empty())
                .map(RangeMap::from)
                .collect()
        })
        .collect()
}

fn get_seeds(parsed_input: &mut Vec<Vec<&str>>) -> Vec<usize> {
    parsed_input
        .remove(0)
        .first()
        .unwrap()
        .split_whitespace()
        .map(|num| num.parse::<usize>().unwrap())
        .collect()
}

fn parse_input(input: &str) -> Vec<Vec<&str>> {
    input
        .split("\n\n")
        .map(|mapping| mapping.split(":").last().unwrap().lines().collect())
        .collect()
}

fn find_lowest_location(input: &str) -> usize {
    let mut parsed_input = parse_input(input);
    let seeds = get_seeds(&mut parsed_input);
    let mappings = get_almanac(parsed_input);

    seeds
        .into_iter()
        .map(|seed| get_location(seed, &mappings))
        .min()
        .unwrap()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_find_lowest_location() {
        let input = include_str!("../../small_input.txt");
        assert_eq!(find_lowest_location(input), 35);
    }
}
