use std::collections::HashMap;
use std::fs::File;
use std::io::Error;
use std::io::prelude::*;
use std::iter::Cycle;
use std::str::Chars;

fn main() {
    let path = "src/day-08.txt";
    let input = open_file(path).unwrap();
    let instructions = get_instructions(&input);
    let instructions = instructions.chars().cycle();
    let map = make_map(&input);

    let starting_nodes: Vec<String> = map
        .map
        .keys()
        .filter(|&key| key.ends_with("A"))
        .cloned()
        .collect();

    let count = count_nodes(map, instructions, &starting_nodes);

    println!("count: {}", count);
}

fn count_nodes(map: Map, instructions: Cycle<Chars<'_>>, starting_nodes: &Vec<String>) -> u64 {
    let mut counts = Vec::new();
    for starting_node in starting_nodes {
        let count = count_nodes_single(&map, instructions.clone(), starting_node.to_owned());
        counts.push(count);
    }
    lcm_of_list(counts)
}

fn count_nodes_single(map: &Map, mut instructions: Cycle<Chars<'_>>, starting_node: String) -> u64 {
    let mut current_node = starting_node;
    let mut count = 0;
    while !current_node.ends_with("Z") {
        count += 1;
        let (left, right) = map.find_node(&current_node);
        current_node = match instructions.next().unwrap() {
            'L' => left,
            'R' => right,
            _ => unreachable!(),
        }
    }
    count
}

fn gcd(mut a: u64, mut b: u64) -> u64 {
    while b != 0 {
        let temp = b;
        b = a % b;
        a = temp;
    }
    a
}

fn lcm(a: u64, b: u64) -> u64 {
    if a == 0 || b == 0 {
        return 0;
    }
    (a * b) / gcd(a, b)
}

fn lcm_of_list(nums: Vec<u64>) -> u64 {
    let mut nums = nums.iter();
    if nums.len() == 0 {
        return 0;
    }
    let mut lcm_val = nums.next().unwrap().clone();
    for num in nums {
        lcm_val = lcm(lcm_val, *num);
    }
    lcm_val
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
        let input = include_str!("../../../src/day-08.small.2.txt");
        let instructions = get_instructions(input);
        assert_eq!(instructions, String::from("LR"));
    }

    #[test]
    fn test_make_map() {
        let input = include_str!("../../../src/day-08.small.2.txt");
        let map = make_map(input);
        let mut expected_map = Map::new();
        expected_map
            .map
            .insert("11A".to_string(), ("11B".to_string(), "XXX".to_string()));
        expected_map
            .map
            .insert("11B".to_string(), ("XXX".to_string(), "11Z".to_string()));
        expected_map
            .map
            .insert("11Z".to_string(), ("11B".to_string(), "XXX".to_string()));
        expected_map
            .map
            .insert("22A".to_string(), ("22B".to_string(), "XXX".to_string()));
        expected_map
            .map
            .insert("22B".to_string(), ("22C".to_string(), "22C".to_string()));
        expected_map
            .map
            .insert("22C".to_string(), ("22Z".to_string(), "22Z".to_string()));
        expected_map
            .map
            .insert("22Z".to_string(), ("22B".to_string(), "22B".to_string()));
        expected_map
            .map
            .insert("22Z".to_string(), ("22B".to_string(), "22B".to_string()));
        expected_map
            .map
            .insert("XXX".to_string(), ("XXX".to_string(), "XXX".to_string()));
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

    #[test]
    fn test_all_final_nodes() {
        let nodes = vec![String::from("11Z"), String::from("22Z")];
        assert!(all_final_nodes(&nodes));

        let bad_nodes = vec![String::from("11A"), String::from("22Z")];
        assert!(!all_final_nodes(&bad_nodes));
    }

    #[test]
    fn test_count_nodes() {
        let input = include_str!("../../../src/day-08.small.2.txt");
        let instructions = get_instructions(&input);
        let instructions = instructions.chars().cycle();
        let map = make_map(&input);

        let starting_nodes: Vec<String> = map
            .map
            .keys()
            .filter(|&key| key.ends_with("A"))
            .cloned()
            .collect();

        let count = count_nodes(map, instructions, &starting_nodes);
        assert_eq!(count, 6);
    }

    #[test]
    fn test_gcd() {
        assert_eq!(gcd(48, 18), 6);
    }
}
