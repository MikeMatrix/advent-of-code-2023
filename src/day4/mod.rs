use crate::aoc_util::read_lines::read_lines_vec;

pub fn run() {
    let file_path = "./inputs/day4/input";

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

const BASE: u32 = 2;

fn part_1(lines: &Vec<String>) -> () {
    let sum: u32 = lines
        .iter()
        .map(|v| parse_line(v))
        .map(|card| -> u32 {
            let matching = card
                .picks
                .iter()
                .filter(|pick| card.winners.iter().any(|winner| *winner == **pick))
                .collect::<Vec<_>>()
                .len();

            return match matching {
                0 => 0,
                len => BASE.pow(TryInto::<u32>::try_into(len).unwrap() - 1),
            };
        })
        .sum::<u32>();

    println!("Sum (Part 1): {}", sum);
}

fn part_2(lines: &Vec<String>) -> () {
    let cards: Vec<_> = lines.iter().map(|line| parse_line(line)).collect();
    let mut card_counts: Vec<u32> = vec![1; cards.len()];

    for (idx, card) in cards.iter().enumerate() {
        let matching = card
            .picks
            .iter()
            .filter(|pick| card.winners.iter().any(|winner| *winner == **pick))
            .collect::<Vec<_>>()
            .len();

        let counts = card_counts[idx];

        for i in idx + 1..idx + 1 + matching {
            if i >= card_counts.len() {
                break;
            }
            card_counts[i] += counts;
        }
    }

    println!("Sum (Part 2): {}", card_counts.iter().sum::<u32>());
}

#[derive(Debug, Clone)]
struct Line {
    card: u32,
    winners: Vec<u32>,
    picks: Vec<u32>,
}

const CARD_NUMBER: [usize; 2] = [5, 8];
const WINNERS: [usize; 2] = [10, 39];
const PICKS: [usize; 2] = [42, 116];

fn parse_line(line: &String) -> Line {
    let card = line[CARD_NUMBER[0]..CARD_NUMBER[1]]
        .trim()
        .parse::<u32>()
        .unwrap_or(0);

    let winners: Vec<_> = line[WINNERS[0]..WINNERS[1]]
        .split_whitespace()
        .map(|v| v.parse::<u32>().unwrap_or(0))
        .collect();
    let picks: Vec<_> = line[PICKS[0]..PICKS[1]]
        .split_whitespace()
        .map(|v| v.parse::<u32>().unwrap_or(0))
        .collect();

    return Line {
        card,
        winners,
        picks,
    };
}
