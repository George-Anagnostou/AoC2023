use std::fs::File;
use std::io::prelude::*;
use std::io::Error;

fn main() {
    let path = "src/day-04.txt";
    let input = open_file(path).unwrap();
    let total_points = calculate_total_points(&input);
    println!("{}", total_points);
}

fn open_file(path: &str) -> Result<String, Error> {
    let mut input_file = File::open(path)?;
    let mut input_string = String::new();
    input_file.read_to_string(&mut input_string)?;
    Ok(input_string)
}

fn calculate_total_points(input: &str) -> usize {
    input.lines().map(parse_line).map(get_line_total).sum()
}

fn parse_line(line: &str) -> (Vec<usize>, Vec<usize>) {
    let (winners, ours) = line.split(":").last().unwrap().split_once("|").unwrap();

    let winners_vec: Vec<usize> = winners
        .split_whitespace()
        .map(|x| x.parse::<usize>().unwrap())
        .collect();
    let ours_vec: Vec<usize> = ours
        .split_whitespace()
        .map(|x| x.parse::<usize>().unwrap())
        .collect();
    (winners_vec, ours_vec)
}

fn get_line_total(line_pair: (Vec<usize>, Vec<usize>)) -> usize {
    let (winners_vec, ours_vec) = line_pair;
    let mut count = 0;

    for winner in &winners_vec {
        if ours_vec.contains(winner) {
            count += 1;
        }
    }

    score_numbers(count)
}

fn score_numbers(n: usize) -> usize {
    if n == 0 || n == 1 {
        return n;
    };
    let mut k = 1;
    for _ in 1..n {
        k *= 2;
    }
    k
}

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn test_parse_line() {
        assert_eq!(
            parse_line("Card 1: 41 48 83 86 17 | 83 86  6 31 17  9 48 53"),
            (vec![41, 48, 83, 86, 17], vec![83, 86, 6, 31, 17, 9, 48, 53])
        );
        assert_eq!(
            parse_line("Card 2: 13 32 20 16 61 | 61 30 68 82 17 32 24 19"),
            (
                vec![13, 32, 20, 16, 61],
                vec![61, 30, 68, 82, 17, 32, 24, 19]
            )
        );
    }

    #[test]
    fn test_get_line_total() {
        assert_eq!(
            get_line_total((vec![41, 48, 83, 86, 17], vec![83, 86, 6, 31, 17, 9, 48, 53])),
            8
        );
        assert_eq!(
            get_line_total((
                vec![13, 32, 20, 16, 61],
                vec![61, 30, 68, 82, 17, 32, 24, 19]
            )),
            2
        );
    }

    #[test]
    fn test_score_numbers() {
        assert_eq!(score_numbers(4), 8);
        assert_eq!(score_numbers(2), 2);
        assert_eq!(score_numbers(1), 1);
        assert_eq!(score_numbers(0), 0);
    }
}
