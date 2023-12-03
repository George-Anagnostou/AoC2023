use std::fs::File;
use std::io::prelude::*;
use std::io::Error;

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
    const RED_MAX: usize = 12;
    const GREEN_MAX: usize = 13;
    const BLUE_MAX: usize = 14;
    let input: Vec<&str> = input.split(":").collect();
    let game_id = input.first().unwrap().split_whitespace().last().unwrap();

    let sets: Vec<&str> = input.last().unwrap().split(";").collect();
    let mut games = Vec::new();
    for set in &sets {
        let mut game: Vec<&str> = set.split(",").collect();
        games.append(&mut game);
    }

    for color_pair in games {
        let color_pair = color_pair.trim();
        let pair: Vec<&str> = color_pair.split_whitespace().collect();
        let qty = pair.first().unwrap();
        let color = pair.last().unwrap();

        if color == &"blue" && qty.parse::<usize>().unwrap() > BLUE_MAX { return 0 }
        if color == &"red" && qty.parse::<usize>().unwrap() > RED_MAX { return 0 }
        if color == &"green" && qty.parse::<usize>().unwrap() > GREEN_MAX { return 0 }
    }
    game_id.parse::<usize>().unwrap()
}
