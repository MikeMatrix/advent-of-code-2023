use crate::aoc_util::read_lines::read_lines_vec;

pub fn run() {
    let file_path = "./inputs/day8/input";

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

fn parse_inputs(lines: &Vec<String>) -> (Vec<usize>, Vec<String>, Vec<[usize; 2]>) {
    let directions: Vec<usize> = lines[0]
        .chars()
        .map(|v| match v {
            'L' => 0,
            'R' => 1,
            _ => 0,
        })
        .collect();

    let mut positions: Vec<String> = vec![];
    let mut paths: Vec<[usize; 2]> = vec![];

    for line in lines[2..].iter() {
        let assignee = line[..=2].to_string();
        let left = line[7..=9].to_string();
        let right = line[12..=14].to_string();

        let assignee_pos: usize = match positions.iter().position(|v| *v == assignee) {
            Some(p) => p,
            None => {
                positions.push(assignee);
                paths.push([usize::MAX, usize::MAX]);
                positions.len() - 1
            }
        };
        let left_pos = match positions.iter().position(|v| *v == left) {
            Some(p) => p,
            None => {
                positions.push(left);
                paths.push([usize::MAX, usize::MAX]);
                positions.len() - 1
            }
        };
        let right_pos = match positions.iter().position(|v| *v == right) {
            Some(p) => p,
            None => {
                positions.push(right);
                paths.push([usize::MAX, usize::MAX]);
                positions.len() - 1
            }
        };

        paths[assignee_pos][0] = left_pos;
        paths[assignee_pos][1] = right_pos;
    }

    return (directions, positions, paths);
}

fn part_1(lines: &Vec<String>) -> () {
    let (directions, positions, paths) = parse_inputs(lines);

    let mut steps = 0;
    let mut current_pos = positions.iter().position(|v| v == "AAA").unwrap();
    for direction in directions.iter().cycle() {
        let current = &positions[current_pos];

        if current == "ZZZ" {
            break;
        }

        current_pos = paths[current_pos][*direction];
        steps += 1;
    }

    println!("Part 1: {}", steps);
}

fn get_cycle_for_position(
    position: usize,
    paths: &Vec<[usize; 2]>,
    directions: &Vec<usize>,
    end_positions: &Vec<usize>,
) -> u64 {
    let mut steps: u64 = 0;
    let mut current = position;
    for direction in directions.iter().cycle() {
        if end_positions.iter().any(|v| *v == current) {
            break;
        }

        current = paths[current][*direction];
        steps += 1;
    }

    return steps;
}

fn part_2(lines: &Vec<String>) -> () {
    let directions: Vec<usize> = lines[0]
        .chars()
        .map(|v| match v {
            'L' => 0,
            'R' => 1,
            _ => 0,
        })
        .collect();

    let mut positions: Vec<String> = vec![];
    let mut paths: Vec<[usize; 2]> = vec![];

    for line in lines[2..].iter() {
        let assignee = line[..=2].to_string();
        let left = line[7..=9].to_string();
        let right = line[12..=14].to_string();

        let assignee_pos: usize = match positions.iter().position(|v| *v == assignee) {
            Some(p) => p,
            None => {
                positions.push(assignee);
                paths.push([usize::MAX, usize::MAX]);
                positions.len() - 1
            }
        };
        let left_pos = match positions.iter().position(|v| *v == left) {
            Some(p) => p,
            None => {
                positions.push(left);
                paths.push([usize::MAX, usize::MAX]);
                positions.len() - 1
            }
        };
        let right_pos = match positions.iter().position(|v| *v == right) {
            Some(p) => p,
            None => {
                positions.push(right);
                paths.push([usize::MAX, usize::MAX]);
                positions.len() - 1
            }
        };

        paths[assignee_pos][0] = left_pos;
        paths[assignee_pos][1] = right_pos;
    }

    let end_positions: Vec<_> = positions
        .iter()
        .enumerate()
        .filter(|(_, v)| v.ends_with("Z"))
        .map(|(pos, _)| pos)
        .collect();

    let mut start_positions = positions
        .iter()
        .enumerate()
        .filter(|(_, v)| v.ends_with("A"))
        .map(|(pos, _)| pos);

    let mut lcm: u64 = get_cycle_for_position(
        start_positions.next().unwrap(),
        &paths,
        &directions,
        &end_positions,
    );

    for pos in start_positions {
        let cycle = get_cycle_for_position(pos, &paths, &directions, &end_positions);
        lcm = num::integer::lcm(lcm, cycle);
    }

    println!("Part 2: {:?}", lcm);
}
