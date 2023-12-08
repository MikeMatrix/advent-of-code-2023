use crate::aoc_util::read_lines::read_lines_vec;

pub fn run() {
    let file_path = "./inputs/day5/input";

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

#[derive(Clone, Debug)]
struct Translation {
    from: u32,
    to: u32,
    size: u32,
}

impl Translation {
    fn translate(&self, from: u32) -> Option<u32> {
        if self.from > from {
            return None;
        }

        let offset = from - self.from;

        return Some(self.to + offset);
    }

    fn from_in_range(&self, from: u32) -> bool {
        if self.from > from {
            return false;
        }

        if (self.from + (self.size - 1)) < from {
            return false;
        }

        return true;
    }
}

#[derive(Clone, Debug)]
struct Mapper {
    translations: Vec<Translation>,
}

impl Mapper {
    fn get_mapping(&self, from: u32) -> u32 {
        let translation = self.translations.iter().find(|t| t.from_in_range(from));

        match translation {
            Some(t) => t.translate(from).unwrap(),
            None => from,
        }
    }

    fn parse(&mut self, value: &String) -> () {
        let split: Vec<_> = value.split_whitespace().collect();

        self.translations.push(Translation {
            from: split[1].parse().unwrap(),
            to: split[0].parse().unwrap(),
            size: split[2].parse().unwrap(),
        });
    }
}

fn parse_lines(lines: &Vec<String>, pairs: bool) -> (Vec<u32>, Vec<Mapper>) {
    let mut seeds: Vec<u32> = vec![];
    let mut mappers: Vec<Mapper> = vec![];

    for line in lines {
        if line.is_empty() {
            continue;
        }

        if line.starts_with("seeds: ") {
            let parsed_seeds: Vec<u32> = line[7..]
                .split_whitespace()
                .map(|v| v.parse::<u32>().unwrap())
                .collect();

            if !pairs {
                seeds = parsed_seeds;
                continue;
            }

            let mut val: u32 = 0;
            for (idx, parsed) in parsed_seeds.iter().enumerate() {
                if idx % 2 == 0 {
                    val = *parsed;
                } else {
                    let mut new_seeds: Vec<u32> = (0..*parsed).map(|v| val + v).collect();
                    seeds.append(&mut new_seeds);
                }
            }

            continue;
        }

        if line.ends_with("map:") {
            mappers.push(Mapper {
                translations: vec![],
            });
            continue;
        }

        let last_mapper = mappers.last_mut();
        if let Some(mapper) = last_mapper {
            mapper.parse(line);
        }
    }

    return (seeds, mappers);
}

fn part_1(lines: &Vec<String>) -> () {
    let (seeds, mappers) = parse_lines(lines, false);

    let mut min_location: u32 = u32::MAX;
    for seed in seeds {
        let mut current_val = seed;
        for mapper in mappers.iter() {
            current_val = mapper.get_mapping(current_val);
        }

        if current_val < min_location {
            min_location = current_val;
        }

        println!("Seed {} to location {}", seed, current_val);
    }

    println!("Part 1: {}", min_location);
}

fn part_2(lines: &Vec<String>) -> () {
    let (seeds, mappers) = parse_lines(lines, true);

    let mut min_location: u32 = u32::MAX;
    for seed in seeds {
        let mut current_val = seed;
        for mapper in mappers.iter() {
            current_val = mapper.get_mapping(current_val);
        }

        if current_val < min_location {
            min_location = current_val;
        }
    }

    println!("Part 2: {}", min_location);
}
