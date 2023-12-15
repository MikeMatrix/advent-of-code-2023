use crate::aoc_util::read_lines::read_lines_vec;

pub fn run() {
    let file_path = "./inputs/day10/input";

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

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
struct Position {
    x: usize,
    y: usize,
}

#[derive(Debug, Clone, Copy)]
struct PositionOffset {
    x: isize,
    y: isize,
}

impl Position {
    fn add(&self, offset: PositionOffset) -> Option<Position> {
        let x_sum = self.x.checked_add_signed(offset.x);
        let y_sum = self.y.checked_add_signed(offset.y);

        if let Some(x) = x_sum {
            if let Some(y) = y_sum {
                return Some(Position { x, y });
            }
        }

        return None;
    }
}

const NORTH_OFFSET: PositionOffset = PositionOffset { x: 0, y: -1 };
const SOUTH_OFFSET: PositionOffset = PositionOffset { x: 0, y: 1 };
const WEST_OFFSET: PositionOffset = PositionOffset { x: -1, y: 0 };
const EAST_OFFSET: PositionOffset = PositionOffset { x: 1, y: 0 };

const CONNECTIONS: [(char, [PositionOffset; 2]); 6] = [
    ('|', [NORTH_OFFSET, SOUTH_OFFSET]),
    ('-', [WEST_OFFSET, EAST_OFFSET]),
    ('L', [NORTH_OFFSET, EAST_OFFSET]),
    ('J', [NORTH_OFFSET, WEST_OFFSET]),
    ('7', [SOUTH_OFFSET, WEST_OFFSET]),
    ('F', [SOUTH_OFFSET, EAST_OFFSET]),
];

fn get_map(lines: &Vec<String>) -> Vec<Vec<char>> {
    return lines.iter().map(|line| line.chars().collect()).collect();
}

fn get_start_pos(map: &Vec<Vec<char>>) -> Option<Position> {
    for (y, line) in map.iter().enumerate() {
        for (x, char) in line.iter().enumerate() {
            if *char == 'S' {
                return Some(Position {
                    x: x.try_into().unwrap(),
                    y: y.try_into().unwrap(),
                });
            }
        }
    }

    return None;
}

fn get_connection_points(map: &Vec<Vec<char>>, pos: Position) -> [Position; 2] {
    let pipe_type = map[pos.y][pos.x];
    if pipe_type == 'S' {
        let mut result: Vec<Position> = vec![];

        for offset in vec![NORTH_OFFSET, EAST_OFFSET, SOUTH_OFFSET, WEST_OFFSET] {
            let test_pos = pos.add(offset).unwrap();
            if get_connection_points(map, test_pos)
                .iter()
                .any(|p| *p == pos)
            {
                result.push(test_pos);

                if result.len() == 2 {
                    return [result[0], result[1]];
                }
            }
        }

        return [pos, pos];
    }

    match CONNECTIONS.iter().find(|c| c.0 == pipe_type) {
        Some(connection) => {
            let pos1 = pos.add(connection.1[0]);
            let pos2 = pos.add(connection.1[1]);
            return [pos1.unwrap_or(pos), pos2.unwrap_or(pos)];
        }
        None => [pos, pos],
    }
}

fn walk_connection(
    map: &Vec<Vec<char>>,
    prev_position: Position,
    position: Position,
) -> Option<Position> {
    let options = get_connection_points(map, position);

    let valid_options = options
        .iter()
        .filter(|pos| **pos != prev_position && **pos != position)
        .next();

    match valid_options {
        Some(pos) => Some(*pos),
        None => None,
    }
}

fn walk_complete(map: &Vec<Vec<char>>, start: Position, next: Position) {
    let mut prev_position = start;
    let mut curr_position = next;

    let mut path: Vec<Position> = vec![prev_position, curr_position];

    while curr_position != start {
        let next_position = walk_connection(map, prev_position, curr_position).unwrap();

        path.push(next_position);

        prev_position = curr_position;
        curr_position = next_position;
    }

    println!(
        "{:?}, Steps: {}, Furthest: {}",
        path,
        path.len(),
        path.len() / 2
    );
}

fn part_1(lines: &Vec<String>) -> () {
    let map = get_map(lines);

    let start_position = get_start_pos(&map).unwrap();

    let initial_connection_positions = get_connection_points(&map, start_position);

    let left_position = initial_connection_positions[0];

    walk_complete(&map, start_position, left_position);
}

fn part_2(_lines: &Vec<String>) -> () {}
