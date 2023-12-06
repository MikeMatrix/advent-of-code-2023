use crate::aoc_util::read_lines::read_lines_vec;

pub fn run() {
    let file_path = "./inputs/day6/input";

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

fn calculate_distance(race_duration: u64, charge_duration: u64) -> u64 {
    return (race_duration - charge_duration) * charge_duration;
}

fn parse_inputs(lines: &Vec<String>, combine: bool) -> Vec<(u64, u64)> {
    let mut line_0_data: Vec<String> = lines[0]
        .split_whitespace()
        .skip(1)
        .map(|v| v.to_string())
        .collect();
    let mut line_1_data: Vec<String> = lines[1]
        .split_whitespace()
        .skip(1)
        .map(|v| v.to_string())
        .collect();

    if combine {
        line_0_data = vec![line_0_data.join("").to_string()];
        line_1_data = vec![line_1_data.join("").to_string()];
    }

    let line_0 = line_0_data.iter().map(|v| v.parse::<u64>().unwrap());
    let line_1 = line_1_data.iter().map(|v| v.parse::<u64>().unwrap());

    return line_0.zip(line_1).collect();
}

fn find_result(race: (u64, u64)) -> u64 {
    let first = (1..(race.0 - 1))
        .find(|charge_duration| calculate_distance(race.0, *charge_duration) > race.1)
        .unwrap();
    let last = (1..(race.0 - 1))
        .rev()
        .find(|charge_duration| calculate_distance(race.0, *charge_duration) > race.1)
        .unwrap();

    println!("Race ({}ms, {}mm): {} - {}", race.0, race.1, first, last);

    return last - first + 1;
}

fn part_1(lines: &Vec<String>) -> () {
    let data = parse_inputs(lines, false);

    let result = data
        .iter()
        .map(|race| find_result(*race))
        .fold(1, |acc, cur| acc * cur);

    println!("Part 1: {}", result);
}

fn part_2(lines: &Vec<String>) -> () {
    let data = parse_inputs(lines, true);

    let result = data
        .iter()
        .map(|race| find_result(*race))
        .fold(1, |acc, cur| acc * cur);

    println!("Part 2: {}", result);
}
