use std::collections::HashSet;
// const SYMBOLS = ['+', '*', '$', '-', '%', '=', '@', '&', '#', '/'];
fn main() {
    let input = include_str!("../../input.txt");
    let part_sum = calculate_part_numbers(input);
    println!("{}", part_sum);
}

#[derive(Debug, Clone)]
struct Coordinate {
    item: String,
    col: usize,
    row: usize,
}

impl Coordinate {
    fn from(item: String, col: usize, row: usize) -> Self {
        Self {
            item,
            col,
            row,
        }
    }
}

fn get_symbols(input: &str) -> HashSet<char> {
    let mut symbols: Vec<char> = Vec::new();
    for line in input.lines() {
        for c in line.chars() {
            if c != '.' && !c.is_alphanumeric() {
                symbols.push(c);
            }
        }
    }
    let symbols: HashSet<char> = symbols.into_iter()
        .collect::<HashSet<char>>();
    symbols
}

fn get_symbol_coordinates(input: &str) -> Vec<Coordinate> {
    let symbols = get_symbols(&input);
    let mut symbol_coordinate_list: Vec<Coordinate> = Vec::new();
    for (row, line) in input.lines().enumerate() {
        for symbol in &symbols {
            let cols: Vec<usize> = line.match_indices(*symbol).map(|tup| tup.0).collect();
            for col in cols {
                symbol_coordinate_list.push(Coordinate::from(symbol.to_string(), col, row));
            }
        }
    }
    symbol_coordinate_list
}

fn calculate_part_numbers(input: &str) -> usize {
    let total_nums = get_all_nums(input);
    let non_adjacent_nums = get_all_non_adjacent_nums(input);

    let part_number_sum = total_nums - non_adjacent_nums;
    part_number_sum
}

fn get_all_nums(input: &str) -> usize {
    let mut sum = 0;
    for line in input.lines() {
        sum += line
            .split(&['.', '+', '*', '$', '-', '%', '=', '@', '&', '#', '/'])
            .filter(|x| !x.is_empty())
            .map(|num: &str| num.parse::<usize>().unwrap())
            .sum::<usize>();
    }
    sum
}

fn get_all_non_adjacent_nums(input: &str) -> usize {
    let mut num_coords: Vec<Coordinate> = Vec::new();
    for (row, line) in input.lines().enumerate() {
        for (col, c) in line.chars().enumerate() {
            if c.is_digit(10) {
                num_coords.push(Coordinate::from(c.to_string(), col, row));
            }
        }
    }

    let mut num_areas: Vec<NumArea> = Vec::new();
    let mut number_unit: Vec<Coordinate> = Vec::new();
    let mut last_num_coord = Coordinate::from("".to_string(), 0, 0);
    for num_coord in num_coords {
        if num_coord.row != last_num_coord.row || num_coord.col != last_num_coord.col + 1 {
            if !number_unit.is_empty() {
                num_areas.push(NumArea::from(number_unit.clone()));
            }
            number_unit.clear();
        }
        number_unit.push(num_coord.clone());
        last_num_coord = num_coord;
    }
    num_areas.push(NumArea::from(number_unit.clone()));

    let symbol_coordinates = get_symbol_coordinates(input);
    let non_adjacent_sum = num_areas
        .into_iter()
        .filter(|num_area| !check_num_area_for_symbol(num_area, &symbol_coordinates))
        .map(|num_area| num_area.number)
        .sum();
    non_adjacent_sum
}

fn check_num_area_for_symbol(num_area: &NumArea, symbol_coordinates: &Vec<Coordinate>) -> bool {
    for symbol_coordinate in symbol_coordinates {
        for surrounding in &num_area.area {
            if surrounding.col == symbol_coordinate.col &&
                surrounding.row == symbol_coordinate.row {
                    return true
                }
        }
    }
    false
}

#[derive(Debug)]
struct NumArea {
    area: Vec<Coordinate>,
    number: usize,
}

impl NumArea {
    fn from(digit_coordinates: Vec<Coordinate>) -> Self {
        let number: String = digit_coordinates.clone()
            .into_iter()
            .map(|digit| digit.item.parse::<char>().unwrap())
            .collect();
        let number = number.parse::<usize>().unwrap();

        let mut area: Vec<Coordinate> = Vec::new();
        for digit_coordinate in &digit_coordinates {
            let mut x: isize = -1;
            for _ in 0..3 {
                let mut y: isize = -1;
                for _ in 0..3 {
                    let new_coordinate = Coordinate::from(
                        digit_coordinate.item.clone(),
                        digit_coordinate.col.checked_add_signed(x).unwrap_or(0),
                        digit_coordinate.row.checked_add_signed(y).unwrap_or(0),
                    );
                    area.push(new_coordinate);
                    y += 1;
                }
                x += 1;
            }
        }
        Self {
            area,
            number,
        }
    }
}
