use crate::aoc_util::read_lines::read_lines_vec;

pub fn run() {
    let file_path = "./inputs/day2/input";

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

fn part_1(lines: &Vec<String>) -> () {
    let sum = lines
        .iter()
        .fold(0, |acc, line| acc + part_1_process_line(line));
    println!("Sum (Part1): {}", sum);
}

const GAME_PREFIX_LENGTH: usize = "Game ".len();
const BAG_LIMITS: [u32; 3] = [12, 13, 14];

fn part_1_process_line(line: &String) -> u32 {
    let colon_index = line.find(':').unwrap_or(0);
    if colon_index < GAME_PREFIX_LENGTH {
        return 0;
    }

    let game_index: u32 = line[GAME_PREFIX_LENGTH..colon_index]
        .parse::<u32>()
        .unwrap_or(0);

    let set_data = &line[colon_index + 1..];

    if !part_1_process_data(&set_data) {
        return 0;
    }

    return game_index;
}

fn part_1_process_data(data: &str) -> bool {
    let mut remaining = &data[..];

    while remaining.len() > 0 {
        let next_split = remaining.find(';').unwrap_or(remaining.len());

        let mut current;
        if next_split < remaining.len() {
            (current, remaining) = remaining.split_at(next_split);
            current = current.trim();
            remaining = remaining[1..].trim();
        } else {
            current = remaining;
            remaining = &"";
        }

        if !part_1_process_set(current) {
            return false;
        }
    }

    return true;
}

fn part_1_process_set(set: &str) -> bool {
    let mut max_dice: [u32; 3] = [0, 0, 0];

    let mut remaining = &set[..];

    while remaining.len() > 0 {
        let next_split = remaining.find(',').unwrap_or(remaining.len());

        let mut current;
        if next_split < remaining.len() {
            (current, remaining) = remaining.split_at(next_split);
            current = current.trim();
            remaining = remaining[1..].trim();
        } else {
            current = remaining;
            remaining = &"";
        }

        let data: Vec<_> = current.split(" ").collect();
        let color = data.get(1).unwrap();
        let amount = data.get(0).unwrap().parse::<u32>().unwrap_or(0);

        match color {
            &"red" => {
                max_dice[0] += amount;
                if max_dice[0] > BAG_LIMITS[0] {
                    return false;
                }
            }
            &"green" => {
                max_dice[1] += amount;
                if max_dice[1] > BAG_LIMITS[1] {
                    return false;
                }
            }
            &"blue" => {
                max_dice[2] += amount;
                if max_dice[2] > BAG_LIMITS[2] {
                    return false;
                }
            }
            _ => {}
        }
    }

    return true;
}

fn part_2(lines: &Vec<String>) -> () {
    let sum = lines
        .iter()
        .fold(0, |acc, line| acc + part_2_process_line(line));
    println!("Sum (Part2): {}", sum);
}

fn part_2_process_line(line: &String) -> u32 {
    let colon_index = line.find(':').unwrap_or(0);
    if colon_index < GAME_PREFIX_LENGTH {
        return 0;
    }

    let set_data = &line[colon_index + 1..];

    return part_2_process_data(&set_data);
}

fn part_2_process_data(data: &str) -> u32 {
    let mut remaining = &data[..];
    let mut max: [u32; 3] = [0, 0, 0];

    while remaining.len() > 0 {
        let next_split = remaining.find(';').unwrap_or(remaining.len());

        let mut current;
        if next_split < remaining.len() {
            (current, remaining) = remaining.split_at(next_split);
            current = current.trim();
            remaining = remaining[1..].trim();
        } else {
            current = remaining;
            remaining = &"";
        }

        let result = part_2_process_set(current);
        if result[0] > max[0] {
            max[0] = result[0];
        }
        if result[1] > max[1] {
            max[1] = result[1];
        }
        if result[2] > max[2] {
            max[2] = result[2];
        }
    }

    return max[0] * max[1] * max[2];
}

fn part_2_process_set(set: &str) -> [u32; 3] {
    let mut max_dice: [u32; 3] = [0, 0, 0];

    let mut remaining = &set[..];

    while remaining.len() > 0 {
        let next_split = remaining.find(',').unwrap_or(remaining.len());

        let mut current;
        if next_split < remaining.len() {
            (current, remaining) = remaining.split_at(next_split);
            current = current.trim();
            remaining = remaining[1..].trim();
        } else {
            current = remaining;
            remaining = &"";
        }

        let data: Vec<_> = current.split(" ").collect();
        let color = data.get(1).unwrap();
        let amount = data.get(0).unwrap().parse::<u32>().unwrap_or(0);

        match color {
            &"red" => {
                max_dice[0] += amount;
            }
            &"green" => {
                max_dice[1] += amount;
            }
            &"blue" => {
                max_dice[2] += amount;
            }
            _ => {}
        }
    }

    return max_dice;
}
