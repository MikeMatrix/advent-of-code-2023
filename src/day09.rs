use crate::aoc_util::read_lines::read_lines_vec;

pub fn run() {
    let file_path = "./inputs/day9/input";

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

fn parse(lines: &Vec<String>) -> Vec<Vec<i64>> {
    return lines
        .iter()
        .map(|line| {
            line.split_whitespace()
                .map(|entry| entry.parse::<i64>().unwrap())
                .collect()
        })
        .collect();
}

fn process_next(data: &Vec<i64>) -> i64 {
    let last = *data.last().unwrap();
    let differences: Vec<i64> = data.windows(2).map(|w| w[1] - w[0]).collect();

    if data.windows(2).all(|w| w[0] == w[1]) {
        let last_diff = *differences.last().unwrap();
        return last + last_diff;
    }

    let next = process_next(&differences);
    return last + next;
}

fn part_1(lines: &Vec<String>) -> () {
    let data_sets = parse(lines);

    let sum: i64 = data_sets.iter().map(|v| process_next(v)).sum();
    println!("Part 1: {}", sum);
}

fn process_prev(data: &Vec<i64>) -> i64 {
    let first = *data.first().unwrap();
    let differences: Vec<i64> = data.windows(2).map(|w| w[1] - w[0]).collect();

    if data.windows(2).all(|w| w[0] == w[1]) {
        let first_diff = *differences.first().unwrap();
        return first - first_diff;
    }

    let prev = process_prev(&differences);
    return first - prev;
}

fn part_2(lines: &Vec<String>) -> () {
    let data_sets = parse(lines);

    let sum: i64 = data_sets.iter().map(|v| process_prev(v)).sum();
    println!("Part 2: {}", sum);
}
