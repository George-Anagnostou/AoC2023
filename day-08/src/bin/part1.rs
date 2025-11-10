use std::collections::HashMap;
use std::fs::File;
use std::io::Error;
use std::io::prelude::*;

fn main() {
    let path = "src/day-08.txt";
    let input = open_file(path).unwrap();
    let instructions = get_instructions(&input);
    let mut instructions = instructions.chars().cycle();
    let map = make_map(&input);
    let mut current_node = String::from("AAA");
    let last_node = String::from("ZZZ");
    let mut count = 0;
    while current_node != last_node {
        count += 1;
        let (left, right) = map.find_node(&current_node);
        current_node = match instructions.next().unwrap() {
            'L' => left,
            'R' => right,
            _ => unreachable!(),
        }
    }
    println!("count: {}", count);
}

#[derive(Debug, PartialEq)]
struct Map {
    map: HashMap<String, (String, String)>,
}

impl Map {
    fn new() -> Self {
        let map: HashMap<String, (String, String)> = HashMap::new();
        Map { map }
    }

    fn find_node(&self, node: &str) -> (String, String) {
        self.map.get(node).unwrap().clone()
    }
}

fn get_instructions(input: &str) -> String {
    input.split("\n\n").take(1).collect()
}

fn make_map(input: &str) -> Map {
    let mut map = Map::new();
    for (i, line) in input.lines().enumerate() {
        if i < 2 {
            continue;
        }
        let parts: Vec<String> = line
            .replace(['=', '(', ')', ','], "")
            .split_whitespace()
            .map(|elm| elm.to_string())
            .collect();
        map.map
            .insert(parts[0].clone(), (parts[1].clone(), parts[2].clone()));
    }
    map
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
    fn test_get_instructions() {
        let input = include_str!("../../../src/day-08.small.txt");
        let instructions = get_instructions(input);
        assert_eq!(instructions, String::from("LLR"));
    }

    #[test]
    fn test_make_map() {
        let input = include_str!("../../../src/day-08.small.txt");
        let map = make_map(input);
        let mut expected_map = Map::new();
        expected_map
            .map
            .insert("AAA".to_string(), ("BBB".to_string(), "BBB".to_string()));
        expected_map
            .map
            .insert("BBB".to_string(), ("AAA".to_string(), "ZZZ".to_string()));
        expected_map
            .map
            .insert("ZZZ".to_string(), ("ZZZ".to_string(), "ZZZ".to_string()));
        assert_eq!(map, expected_map);
    }

    #[test]
    fn test_find_node() {
        let mut map = Map::new();
        map.map
            .insert("AAA".to_string(), ("BBB".to_string(), "BBB".to_string()));

        let value = map.find_node("AAA");
        assert_eq!(value, ("BBB".to_string(), "BBB".to_string()));

        map.map
            .insert("BBB".to_string(), ("AAA".to_string(), "ZZZ".to_string()));
        let value2 = map.find_node("BBB");
        assert_eq!(value2, ("AAA".to_string(), "ZZZ".to_string()));
    }
}
