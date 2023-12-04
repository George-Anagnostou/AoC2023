use std::fs::File;
use std::io::prelude::*;
use std::io::Error;

// const RED_MAX: usize = 12;
// const GREEN_MAX: usize = 13;
// const BLUE_MAX: usize = 14;

fn main() -> Result<(), Error> {
    let path = "input.txt";
    let input = open_file(path)?;

    let mut sum = 0;
    for line in input.lines() {
        let num = parse_input(line);
        sum += num;
    }
    println!("{}", sum);
    Ok(())
}

fn open_file(path: &str) -> Result<String, Error> {
    let mut file = File::open(path)?;
    let mut input = String::new();
    file.read_to_string(&mut input)?;
    Ok(input)
}

fn parse_input(input: &str) -> usize {
    let input: Vec<&str> = input.split(":").collect();

    let sets: Vec<&str> = input.last().unwrap().split(";").collect();
    let mut games = Vec::new();
    for set in &sets {
        let mut game: Vec<&str> = set.split(",").collect();
        games.append(&mut game);
    }

    let mut max_red: usize = 0;
    let mut max_blue: usize = 0;
    let mut max_green: usize = 0;
    for color_pair in games {
        let color_pair = color_pair.trim();
        let pair: Vec<&str> = color_pair.split_whitespace().collect();
        let qty = pair.first().unwrap().parse::<usize>().unwrap();
        let color = pair.last().unwrap();
        match (color, qty) {
            (&"blue", qty) => {
                if qty > max_blue { max_blue = qty }
            },
            (&"red", qty) => {
                if qty > max_red { max_red = qty }
            },
            (&"green", qty) => {
                if qty > max_green { max_green = qty }
            }
            _ => ()
        }

    }

    let power = max_red * max_blue * max_green;
    power
}
