use std::collections::HashMap;
use std::fs::File;
use std::io::Error;
use std::io::prelude::*;

fn main() {
    let path = "src/day-07.txt";
    let input = open_file(path).unwrap();
    let mut hands = parse_input(&input);
    hands.sort();
    let winnings = calculate_winnings(hands);
    println!("Winnings = {}", winnings);
}

fn calculate_winnings(hands: Vec<Hand>) -> u32 {
    let mut winnings = 0;
    for (rank, hand) in hands.iter().enumerate().map(|(i, item)| (i + 1, item)) {
        let winning = rank as u32 * hand.bid;
        winnings += winning;
    }
    winnings
}

#[derive(Debug, PartialEq, Eq, PartialOrd, Ord, Clone, Copy)]
enum CardRank {
    RankJ,
    Rank2,
    Rank3,
    Rank4,
    Rank5,
    Rank6,
    Rank7,
    Rank8,
    Rank9,
    RankT,
    RankQ,
    RankK,
    RankA,
}

#[derive(Debug, PartialEq, Eq, PartialOrd, Ord)]
enum HandType {
    HighCard,
    OnePair,
    TwoPair,
    ThreeOfAKind,
    FullHouse,
    FourOfAKind,
    FiveOfAKind,
}

#[derive(Debug, PartialEq, Eq, PartialOrd, Ord)]
struct Hand {
    hand_type: HandType,
    cards: Vec<CardRank>,
    bid: u32,
}

impl Hand {
    fn from(cards: String, bid: u32) -> Self {
        let mut card_ranks: Vec<CardRank> = Vec::with_capacity(cards.len());
        for ch in cards.chars() {
            let card = Self::get_rank(ch);
            card_ranks.push(card);
        }
        Hand {
            cards: card_ranks,
            hand_type: Self::get_type(cards),
            bid,
        }
    }

    fn get_rank(card: char) -> CardRank {
        match card {
            'J' => CardRank::RankJ,
            '2' => CardRank::Rank2,
            '3' => CardRank::Rank3,
            '4' => CardRank::Rank4,
            '5' => CardRank::Rank5,
            '6' => CardRank::Rank6,
            '7' => CardRank::Rank7,
            '8' => CardRank::Rank8,
            '9' => CardRank::Rank9,
            'T' => CardRank::RankT,
            'Q' => CardRank::RankQ,
            'K' => CardRank::RankK,
            'A' => CardRank::RankA,
            _ => unreachable!("Invalid Card"),
        }
    }

    fn get_type(card: String) -> HandType {
        // Count occurrences of each card
        let mut type_map: HashMap<char, u32> = HashMap::new();
        for ch in card.chars() {
            type_map
                .entry(ch)
                .and_modify(|count| *count += 1)
                .or_insert(1);
        }

        // Count jokers and remove them from the map
        let num_jokers = type_map.remove(&'J').unwrap_or(0);

        // Special case: all cards are jokers
        if num_jokers == 5 {
            return HandType::FiveOfAKind;
        }

        // Get counts of non-joker cards and sort by count (descending)
        let mut counts: Vec<u32> = type_map.values().copied().collect();
        counts.sort_by(|a, b| b.cmp(a));

        // Add all jokers to the highest count to maximize hand value
        if let Some(highest) = counts.first_mut() {
            *highest += num_jokers;
        }

        // Determine hand type based on the counts
        match counts.as_slice() {
            [5] => HandType::FiveOfAKind,
            [4, 1] => HandType::FourOfAKind,
            [3, 2] => HandType::FullHouse,
            [3, 1, 1] => HandType::ThreeOfAKind,
            [2, 2, 1] => HandType::TwoPair,
            [2, 1, 1, 1] => HandType::OnePair,
            _ => HandType::HighCard,
        }
    }
}

fn parse_input(input: &str) -> Vec<Hand> {
    let mut hands: Vec<Hand> = Vec::new();
    for line in input.lines() {
        let mut parts = line.split_whitespace();
        let cards = parts.next().expect("expected cards").to_string();
        let bid = parts.next().expect("expected bid").parse::<u32>().unwrap();
        let hand = Hand::from(cards, bid);
        hands.push(hand);
    }
    hands
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
        let file = include_str!("../../../src/day-07.small.txt");
        let parsed_input = parse_input(file);
        let hands: Vec<Hand> = vec![
            Hand {
                cards: vec![
                    CardRank::Rank3,
                    CardRank::Rank2,
                    CardRank::RankT,
                    CardRank::Rank3,
                    CardRank::RankK,
                ],
                hand_type: HandType::OnePair,
                bid: 765,
            },
            Hand {
                cards: vec![
                    CardRank::RankT,
                    CardRank::Rank5,
                    CardRank::Rank5,
                    CardRank::RankJ,
                    CardRank::Rank5,
                ],
                hand_type: HandType::FourOfAKind,
                bid: 684,
            },
            Hand {
                cards: vec![
                    CardRank::RankK,
                    CardRank::RankK,
                    CardRank::Rank6,
                    CardRank::Rank7,
                    CardRank::Rank7,
                ],
                hand_type: HandType::TwoPair,
                bid: 28,
            },
            Hand {
                cards: vec![
                    CardRank::RankK,
                    CardRank::RankT,
                    CardRank::RankJ,
                    CardRank::RankJ,
                    CardRank::RankT,
                ],
                hand_type: HandType::FourOfAKind,
                bid: 220,
            },
            Hand {
                cards: vec![
                    CardRank::RankQ,
                    CardRank::RankQ,
                    CardRank::RankQ,
                    CardRank::RankJ,
                    CardRank::RankA,
                ],
                hand_type: HandType::FourOfAKind,
                bid: 483,
            },
        ];
        assert_eq!(parsed_input.len(), hands.len());
        let zipped_hands = parsed_input.iter().zip(hands);
        for (parsed_hand, hand) in zipped_hands {
            assert_eq!(*parsed_hand, hand);
        }
    }

    #[test]
    fn test_get_type() {
        assert_eq!(Hand::get_type(String::from("K2345")), HandType::HighCard);
        assert_eq!(Hand::get_type(String::from("32T3K")), HandType::OnePair);
        assert_eq!(Hand::get_type(String::from("KK677")), HandType::TwoPair);
        assert_eq!(Hand::get_type(String::from("KTJJT")), HandType::FourOfAKind);
        assert_eq!(Hand::get_type(String::from("T55J5")), HandType::FourOfAKind,);
        assert_eq!(Hand::get_type(String::from("QQQJA")), HandType::FourOfAKind,);
        assert_eq!(Hand::get_type(String::from("KKKAA")), HandType::FullHouse);
        assert_eq!(Hand::get_type(String::from("K2K22")), HandType::FullHouse);
        assert_eq!(Hand::get_type(String::from("22224")), HandType::FourOfAKind);
        assert_eq!(Hand::get_type(String::from("88888")), HandType::FiveOfAKind);
        assert_eq!(Hand::get_type(String::from("JJJJJ")), HandType::FiveOfAKind);
    }

    #[test]
    fn test_hand_cmp() {
        let hand1 = Hand {
            cards: vec![
                CardRank::RankK,
                CardRank::RankK,
                CardRank::Rank6,
                CardRank::Rank7,
                CardRank::Rank7,
            ],
            hand_type: HandType::TwoPair,
            bid: 28,
        };
        let hand2 = Hand {
            cards: vec![
                CardRank::RankK,
                CardRank::RankT,
                CardRank::RankJ,
                CardRank::RankJ,
                CardRank::RankT,
            ],
            hand_type: HandType::FourOfAKind,
            bid: 220,
        };
        let hand3 = Hand {
            cards: vec![
                CardRank::Rank3,
                CardRank::Rank2,
                CardRank::RankT,
                CardRank::Rank3,
                CardRank::RankK,
            ],
            hand_type: HandType::OnePair,
            bid: 765,
        };
        let hand4 = Hand {
            cards: vec![
                CardRank::RankT,
                CardRank::Rank5,
                CardRank::Rank5,
                CardRank::RankJ,
                CardRank::Rank5,
            ],
            hand_type: HandType::FourOfAKind,
            bid: 684,
        };
        let hand5 = Hand {
            cards: vec![
                CardRank::RankQ,
                CardRank::RankQ,
                CardRank::RankQ,
                CardRank::RankJ,
                CardRank::RankA,
            ],
            hand_type: HandType::FourOfAKind,
            bid: 483,
        };

        assert!(hand4 > hand1);
        assert!(hand1 > hand3);
        assert!(hand2 > hand5);
        assert!(hand5 > hand4);
    }

    #[test]
    fn test_calculate_winnings() {
        let hand1 = Hand {
            cards: vec![
                CardRank::RankK,
                CardRank::RankK,
                CardRank::Rank6,
                CardRank::Rank7,
                CardRank::Rank7,
            ],
            hand_type: HandType::TwoPair,
            bid: 28,
        };
        let hand2 = Hand {
            cards: vec![
                CardRank::RankK,
                CardRank::RankT,
                CardRank::RankJ,
                CardRank::RankJ,
                CardRank::RankT,
            ],
            hand_type: HandType::FourOfAKind,
            bid: 220,
        };
        let hand3 = Hand {
            cards: vec![
                CardRank::Rank3,
                CardRank::Rank2,
                CardRank::RankT,
                CardRank::Rank3,
                CardRank::RankK,
            ],
            hand_type: HandType::OnePair,
            bid: 765,
        };
        let hand4 = Hand {
            cards: vec![
                CardRank::RankT,
                CardRank::Rank5,
                CardRank::Rank5,
                CardRank::RankJ,
                CardRank::Rank5,
            ],
            hand_type: HandType::FourOfAKind,
            bid: 684,
        };
        let hand5 = Hand {
            cards: vec![
                CardRank::RankQ,
                CardRank::RankQ,
                CardRank::RankQ,
                CardRank::RankJ,
                CardRank::RankA,
            ],
            hand_type: HandType::FourOfAKind,
            bid: 483,
        };

        let mut hands: Vec<Hand> = vec![hand1, hand2, hand3, hand4, hand5];
        hands.sort();
        let winnings = calculate_winnings(hands);
        let expected_winnings = 5905;
        assert_eq!(winnings, expected_winnings);
    }

    #[test]
    fn test_solution() {
        let input = include_str!("../../../src/day-07.small.txt");
        let mut hands = parse_input(&input);
        hands.sort();
        let winnings = calculate_winnings(hands);
        let expected_winnings = 5905;
        assert_eq!(winnings, expected_winnings);
    }
}
