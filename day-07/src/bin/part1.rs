use std::collections::HashMap;
use std::fs::File;
use std::io::Error;
use std::io::prelude::*;

fn main() {
    let path = "src/day-07.txt";
    let input = open_file(path).unwrap();
    let mut hands = parse_input(&input);
    hands.sort();
    let mut winnings = 0;
    for (rank, hand) in hands.iter().enumerate().map(|(i, item)| (i + 1, item)) {
        let winning = rank as u32 * hand.bid;
        winnings += winning;
    }
    println!("Winnings = {}", winnings);
}

#[derive(Debug, PartialEq, Eq, PartialOrd, Ord)]
enum CardRank {
    Rank2,
    Rank3,
    Rank4,
    Rank5,
    Rank6,
    Rank7,
    Rank8,
    Rank9,
    RankT,
    RankJ,
    RankQ,
    RankK,
    RankA,
    RankError, // should never occur. used to safisfy rank_match
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
            '2' => CardRank::Rank2,
            '3' => CardRank::Rank3,
            '4' => CardRank::Rank4,
            '5' => CardRank::Rank5,
            '6' => CardRank::Rank6,
            '7' => CardRank::Rank7,
            '8' => CardRank::Rank8,
            '9' => CardRank::Rank9,
            'T' => CardRank::RankT,
            'J' => CardRank::RankJ,
            'Q' => CardRank::RankQ,
            'K' => CardRank::RankK,
            'A' => CardRank::RankA,
            _ => CardRank::RankError,
        }
    }

    fn get_type(card: String) -> HandType {
        let mut type_map: HashMap<char, u32> = HashMap::new();
        for ch in card.chars() {
            type_map
                .entry(ch)
                .and_modify(|count| *count += 1)
                .or_insert(1);
        }

        for (key, value) in type_map.iter() {
            if *value == 5 {
                return HandType::FiveOfAKind;
            } else if *value == 4 {
                return HandType::FourOfAKind;
            } else if *value == 3 {
                for (key2, value2) in type_map.iter() {
                    if key2 != key && *value2 == 2 {
                        return HandType::FullHouse;
                    }
                }
                return HandType::ThreeOfAKind;
            } else if *value == 2 {
                for (key2, value2) in type_map.iter() {
                    if key2 != key && *value2 == 3 {
                        return HandType::FullHouse;
                    }
                    if key2 != key && *value2 == 2 {
                        return HandType::TwoPair;
                    }
                }
                return HandType::OnePair;
            } else {
                continue;
            }
        }
        HandType::HighCard
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
                hand_type: HandType::ThreeOfAKind,
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
                hand_type: HandType::TwoPair,
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
                hand_type: HandType::ThreeOfAKind,
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
        assert_eq!(Hand::get_type(String::from("12345")), HandType::HighCard);
        assert_eq!(Hand::get_type(String::from("32T3K")), HandType::OnePair);
        assert_eq!(Hand::get_type(String::from("KK677")), HandType::TwoPair);
        assert_eq!(Hand::get_type(String::from("KTJJT")), HandType::TwoPair);
        assert_eq!(
            Hand::get_type(String::from("T55J5")),
            HandType::ThreeOfAKind
        );
        assert_eq!(
            Hand::get_type(String::from("QQQJA")),
            HandType::ThreeOfAKind
        );
        assert_eq!(Hand::get_type(String::from("KKKAA")), HandType::FullHouse);
        assert_eq!(Hand::get_type(String::from("12122")), HandType::FullHouse);
        assert_eq!(Hand::get_type(String::from("22224")), HandType::FourOfAKind);
        assert_eq!(Hand::get_type(String::from("88888")), HandType::FiveOfAKind);
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
            hand_type: HandType::TwoPair,
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
        assert!(hand1 > hand2);
        assert!(hand1 > hand3);
        assert!(hand2 > hand3);
    }
}
