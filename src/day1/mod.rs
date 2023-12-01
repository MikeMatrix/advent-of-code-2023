use crate::aoc_util::read_lines::read_lines_vec;

pub fn run() {
    let file_path = "./inputs/day1.txt";

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
        .fold(0, |acc, line| acc + process_line(line, NUMS[..10].to_vec()));
    println!("Sum (Part 1): {}", sum);
}

fn part_2(lines: &Vec<String>) -> () {
    let sum = lines
        .iter()
        .fold(0, |acc, line| acc + process_line(&line, NUMS.to_vec()));
    println!("Sum (Part 2): {}", sum);
}

static NUMS: [&str; 20] = [
    "0", "1", "2", "3", "4", "5", "6", "7", "8", "9", "zero", "one", "two", "three", "four",
    "five", "six", "seven", "eight", "nine",
];

fn process_line(line: &String, nums: Vec<&str>) -> u32 {
    let mut matched_indices: Vec<(usize, u32)> = nums
        .iter()
        .enumerate()
        .flat_map(|(idx, num)| {
            let number = idx as u32 % 10;
            line.match_indices(num)
                .map(|(position, _)| (position, number))
                .collect::<Vec<_>>()
        })
        .collect();

    matched_indices.sort_by_key(|x| x.0);
    let first = matched_indices.first().unwrap();
    let last = matched_indices.last().unwrap_or(first);

    return (first.1 * 10) + last.1;
}
