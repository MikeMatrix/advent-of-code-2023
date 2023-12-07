use std::cmp::Ordering;

use crate::aoc_util::read_lines::read_lines_vec;

pub fn run() {
    let file_path = "./inputs/day7/input";

    match read_lines_vec(file_path) {
        Ok(lines) => {
            part_1(&lines);
            part_2(&lines);
        }
        Err(e) => {
            println!("{}", e);
        }
    }
}

const VALUES_NORMAL: [char; 13] = [
    '2', '3', '4', '5', '6', '7', '8', '9', 'T', 'J', 'Q', 'K', 'A',
];
const VALUES_JOKER: [char; 13] = [
    'J', '2', '3', '4', '5', '6', '7', '8', '9', 'T', 'Q', 'K', 'A',
];
const VALUES_JOKER_POS: usize = 0;

#[derive(PartialEq, PartialOrd)]
enum HandType {
    FiveKind = 6,
    FourKind = 5,
    FullHouse = 4,
    ThreeKind = 3,
    TwoPair = 2,
    OnePair = 1,
    HighCard = 0,
}

#[derive(Debug)]
struct Hand {
    cards: [usize; 5],
    bet: u32,
    joker_rules: bool,
}

impl Hand {
    fn get_card_counts(&self) -> Vec<(usize, u32)> {
        let mut card_counts: Vec<(usize, u32)> = vec![];

        for card in self.cards {
            let count = card_counts.iter_mut().find(|v| v.0 == card);

            match count {
                Some(v) => {
                    v.1 += 1;
                }
                None => {
                    card_counts.push((card, 1));
                }
            }
        }

        return card_counts;
    }

    fn get_hand_type(&self) -> HandType {
        let card_counts = self.get_card_counts();

        let joker = match self.joker_rules {
            false => None,
            true => card_counts.iter().find(|v| v.0 == VALUES_JOKER_POS),
        };

        return match card_counts.len() {
            1 => HandType::FiveKind,
            2 => {
                if joker.is_some() {
                    return HandType::FiveKind;
                }

                if card_counts.iter().find(|v| v.1 == 4).is_some() {
                    return HandType::FourKind;
                }

                return HandType::FullHouse;
            }
            3 => {
                if card_counts.iter().find(|v| v.1 == 3).is_some() {
                    if joker.is_some() {
                        return HandType::FourKind;
                    }

                    return HandType::ThreeKind;
                }

                if let Some(j) = joker {
                    if j.1 == 1 {
                        return HandType::FullHouse;
                    }

                    if j.1 == 2 {
                        return HandType::FourKind;
                    }
                }

                return HandType::TwoPair;
            }
            4 => {
                if joker.is_some() {
                    return HandType::ThreeKind;
                }

                return HandType::OnePair;
            }
            5 => {
                if joker.is_some() {
                    return HandType::OnePair;
                }

                return HandType::HighCard;
            }
            _ => HandType::HighCard,
        };
    }

    fn compare(&self, other: &Hand) -> Ordering {
        return match self.get_hand_type().partial_cmp(&other.get_hand_type()) {
            Some(Ordering::Equal) => {
                let mismatch = self
                    .cards
                    .iter()
                    .zip(other.cards.iter())
                    .find(|(card_a, card_b)| card_a.cmp(card_b) != Ordering::Equal);

                return match mismatch {
                    Some((card_a, card_b)) => card_a.cmp(card_b),
                    None => Ordering::Equal,
                };
            }
            Some(ord) => ord,
            None => Ordering::Equal,
        };
    }
}

fn parse_inputs(lines: &Vec<String>, joker_rules: bool) -> Vec<Hand> {
    let value_list = match joker_rules {
        true => VALUES_JOKER,
        false => VALUES_NORMAL,
    };

    return lines
        .iter()
        .map(|line| {
            let hand_chars: Vec<_> = line[..5].chars().collect();
            let cards: [usize; 5] = [
                value_list.iter().position(|v| *v == hand_chars[0]).unwrap(),
                value_list.iter().position(|v| *v == hand_chars[1]).unwrap(),
                value_list.iter().position(|v| *v == hand_chars[2]).unwrap(),
                value_list.iter().position(|v| *v == hand_chars[3]).unwrap(),
                value_list.iter().position(|v| *v == hand_chars[4]).unwrap(),
            ];

            let bet: u32 = line[6..].parse().unwrap();

            return Hand {
                cards,
                bet,
                joker_rules,
            };
        })
        .collect();
}

fn part_1(lines: &Vec<String>) -> () {
    let mut data = parse_inputs(lines, false);

    data.sort_by(|a, b| a.compare(b));

    let sum: u32 = data
        .iter()
        .enumerate()
        .map(|(idx, hand)| {
            let rank: u32 = idx.try_into().unwrap_or(1) + 1;
            return hand.bet * rank;
        })
        .sum();

    println!("Part 1: {}", sum);
}

fn part_2(lines: &Vec<String>) -> () {
    let mut data = parse_inputs(lines, true);

    data.sort_by(|a, b| a.compare(b));

    let sum: u32 = data
        .iter()
        .enumerate()
        .map(|(idx, hand)| {
            let rank: u32 = idx.try_into().unwrap_or(1) + 1;
            return hand.bet * rank;
        })
        .sum();

    println!("Part 2: {}", sum);
}
