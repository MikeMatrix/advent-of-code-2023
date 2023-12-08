use crate::aoc_util::read_lines::read_lines_vec;

pub fn run() {
    let file_path = "./inputs/day20/input";

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

fn part_1(_lines: &Vec<String>) -> () {}

fn part_2(_lines: &Vec<String>) -> () {}
