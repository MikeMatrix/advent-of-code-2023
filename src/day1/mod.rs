use std::env;
use std::fs::File;
use std::io::{self, BufRead};
use std::path::Path;

pub fn run() {
    let args: Vec<String> = env::args().collect();
    let file_path = &args[1];

    println!("File {}", file_path);

    if let Ok(lines) = read_lines(file_path) {
        let sum = lines.fold(0, |acc, line| {
            acc + process_line(&line.unwrap_or("".to_string()))
        });

        println!("Sum: {}", sum);
    }
}

fn read_lines<P>(filename: P) -> io::Result<io::Lines<io::BufReader<File>>>
where
    P: AsRef<Path>,
{
    let file = File::open(filename)?;
    Ok(io::BufReader::new(file).lines())
}

static NUMS: [&str; 20] = [
    "0", "1", "2", "3", "4", "5", "6", "7", "8", "9", "zero", "one", "two", "three", "four",
    "five", "six", "seven", "eight", "nine",
];

fn process_line(line: &str) -> u32 {
    let mut matched_indices: Vec<(usize, u32)> = NUMS
        .iter()
        .enumerate()
        .flat_map(|(idx, num)| {
            line.match_indices(num)
                .map(|(position, _)| (position, (idx as u32) % 10))
                .collect::<Vec<_>>()
        })
        .collect();

    matched_indices.sort_by_key(|x| x.0);
    let first = matched_indices.first().unwrap();
    let last = matched_indices.last().unwrap_or(first);

    return (first.1 * 10) + last.1;
}
