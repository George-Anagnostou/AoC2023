use std::collections::HashMap;

fn main() {
    let input = include_str!("../../input.txt");
    let total_scratchcards = calculate_total_scratchcards(&input);
    println!("{}", total_scratchcards);
}

fn calculate_total_scratchcards(input: &str) -> usize {
    let card_map = create_card_map(&input);

    card_map
        .values()
        .sum()
}

fn parse_line(line: &str) -> (usize, Vec<usize>, Vec<usize>) {
    let (card_number, values) = line.split_once(":").unwrap();
    let card_number = card_number.split_whitespace().last().unwrap().parse::<usize>().unwrap();
    let (winners, ours) = values
        .split_once("|")
        .unwrap();

    let winners_vec: Vec<usize> = winners
        .split_whitespace()
        .map(|x| x.parse::<usize>().unwrap())
        .collect();
    let ours_vec: Vec<usize> = ours
        .split_whitespace()
        .map(|x| x.parse::<usize>().unwrap())
        .collect();
    (card_number, winners_vec, ours_vec)
}

fn get_line_total(line_pair: (usize, Vec<usize>, Vec<usize>)) -> usize {
    let (_card_number, winners_vec, ours_vec) = line_pair;
    let mut count = 0;

    for winner in &winners_vec {
        if ours_vec.contains(winner) {
            count += 1;
        }
    }

    count
}

fn create_card_map(input: &str) -> HashMap<usize, usize> {
    let mut card_counts: HashMap<usize, usize> = HashMap::new();

    let num_matches: Vec<usize> = input
        .lines()
        .map(parse_line)
        .map(get_line_total)
        .collect();

    for (row, look_ahead) in num_matches.iter().enumerate() {
        card_counts
            .entry(row)
            .and_modify(|counter| *counter += 1)
            .or_insert(1);

        let count = match card_counts.get(&row) {
            Some(num) => *num,
            None => 1,
        };
        for _ in 0..count {
            for i in 1..=*look_ahead {
                card_counts
                    .entry(row + i)
                    .and_modify(|counter| *counter += 1)
                    .or_insert(1);
            }
        }
    }

    card_counts
}

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn test_parse_line() {
        assert_eq!(parse_line("Card 1: 41 48 83 86 17 | 83 86  6 31 17  9 48 53"),
                   (1, vec![41, 48, 83, 86, 17], vec![83, 86, 6, 31, 17, 9, 48, 53]));
        assert_eq!(parse_line("Card 2: 13 32 20 16 61 | 61 30 68 82 17 32 24 19"),
                   (2, vec![13, 32, 20, 16, 61], vec![61, 30, 68, 82, 17, 32, 24, 19]));
    }

    #[test]
    fn test_get_line_total() {
        assert_eq!(get_line_total((1, vec![41, 48, 83, 86, 17], vec![83, 86, 6, 31, 17, 9, 48, 53])), 4);
        assert_eq!(get_line_total((2, vec![13, 32, 20, 16, 61], vec![61, 30, 68, 82, 17, 32, 24, 19])), 2);
        assert_eq!(get_line_total((5, vec![87, 83, 26, 28, 32], vec![88, 30, 70, 12, 93, 22, 82, 36])), 0);
    }

    #[test]
    fn test_calculate_total_scratchcards() {
        let input = "Card 1: 41 48 83 86 17 | 83 86  6 31 17  9 48 53
    Card 2: 13 32 20 16 61 | 61 30 68 82 17 32 24 19
    Card 3:  1 21 53 59 44 | 69 82 63 72 16 21 14  1
    Card 4: 41 92 73 84 69 | 59 84 76 51 58  5 54 83
    Card 5: 87 83 26 28 32 | 88 30 70 12 93 22 82 36
    Card 6: 31 18 13 56 72 | 74 77 10 23 35 67 36 11";

        assert_eq!(calculate_total_scratchcards(&input), 30);
    }
}
