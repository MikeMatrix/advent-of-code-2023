use core::fmt;

use crate::aoc_util::read_lines::read_lines_vec;

pub fn run() {
    let file_path = "./inputs/day3/input";

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

#[derive(Clone)]
struct Position {
    x: i32,
    y: i32,
}

impl fmt::Display for Position {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "({}, {})", self.x, self.y)?;
        Ok(())
    }
}

#[derive(Clone)]
struct Symbol {
    character: char,
    position: Position,
}

#[derive(Clone)]
struct Part {
    part_number: u32,
    position_start: Position,
    position_end: Position,
}

impl Part {
    fn is_adjacent_to(&self, marker: &Symbol) -> bool {
        if marker.position.y < self.position_start.y - 1 {
            return false;
        }

        if marker.position.y > self.position_start.y + 1 {
            return false;
        }

        if marker.position.x < self.position_start.x - 1 {
            return false;
        }

        if marker.position.x > self.position_end.x + 1 {
            return false;
        }

        return true;
    }
}

fn part_1(lines: &Vec<String>) -> () {
    let (markers, part_numbers) = parts_and_markers(lines);

    let sum: u32 = part_numbers
        .iter()
        .filter(|part| {
            return markers.iter().any(|marker| part.is_adjacent_to(marker));
        })
        .fold(0, |acc, part| acc + part.part_number);

    println!("Sum (Part 1): {}", sum);
}

fn part_2(lines: &Vec<String>) -> () {
    let (markers, part_numbers) = parts_and_markers(lines);

    let multiply_markers = markers.iter().filter(|marker| marker.character == '*');

    let sum: u32 = multiply_markers
        .map(|marker| -> u32 {
            let parts: Vec<_> = part_numbers
                .iter()
                .filter(|part| part.is_adjacent_to(marker))
                .collect();

            if parts.len() == 2 {
                return parts[0].part_number * parts[1].part_number;
            }

            return 0;
        })
        .fold(0, |acc, cur| acc + cur);

    println!("Sum (Part 2): {}", sum);
}

fn parts_and_markers(lines: &Vec<String>) -> (Vec<Symbol>, Vec<Part>) {
    let mut markers: Vec<Symbol> = Vec::new();
    let mut part_numbers: Vec<Part> = Vec::new();

    for (y, line) in lines.iter().enumerate() {
        for (x, character) in line
            .chars()
            .enumerate()
            .filter(|(_, character)| *character != '.')
        {
            let position = Position {
                x: x.try_into().unwrap(),
                y: y.try_into().unwrap(),
            };
            if !character.is_digit(10) {
                markers.push(Symbol {
                    character,
                    position,
                });
                continue;
            }

            let number = character.to_digit(10).unwrap();

            match part_numbers.last_mut() {
                Some(part) => {
                    if part.position_end.y == position.y && part.position_end.x == position.x - 1 {
                        part.part_number *= 10;
                        part.part_number += number;
                        part.position_end = position.clone();
                    } else {
                        part_numbers.push(Part {
                            part_number: number,
                            position_start: position.clone(),
                            position_end: position.clone(),
                        });
                    }
                }
                None => {
                    part_numbers.push(Part {
                        part_number: number,
                        position_start: position.clone(),
                        position_end: position.clone(),
                    });
                }
            }
        }
    }

    return (markers, part_numbers);
}
