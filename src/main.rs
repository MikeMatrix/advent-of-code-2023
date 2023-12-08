use clap::{Parser, Subcommand};

mod day01;
mod day02;
mod day03;
mod day04;
mod day05;
mod day06;
mod day07;
mod day08;
mod day09;
mod day10;
mod day11;
mod day12;
mod day13;
mod day14;
mod day15;
mod day16;
mod day17;
mod day18;
mod day19;
mod day20;
mod day21;
mod day22;
mod day23;
mod day24;
mod aoc_util {
    pub mod read_lines;
}

#[derive(Debug, Parser)]
#[command(name = "aoc")]
#[command(about = "Advent of Code 2023 runner")]
struct Cli {
    #[command(subcommand)]
    command: Commands,
}

#[derive(Debug, Subcommand)]
enum Commands {
    Day1,
    Day2,
    Day3,
    Day4,
    Day5,
    Day6,
    Day7,
    Day8,
    Day9,
    Day10,
    Day11,
    Day12,
    Day13,
    Day14,
    Day15,
    Day16,
    Day17,
    Day18,
    Day19,
    Day20,
    Day21,
    Day22,
    Day23,
    Day24,
}

fn main() {
    let args = Cli::parse();

    match args.command {
        Commands::Day1 => day01::run(),
        Commands::Day2 => day02::run(),
        Commands::Day3 => day03::run(),
        Commands::Day4 => day04::run(),
        Commands::Day5 => day05::run(),
        Commands::Day6 => day06::run(),
        Commands::Day7 => day07::run(),
        Commands::Day8 => day08::run(),
        Commands::Day9 => day09::run(),
        Commands::Day10 => day10::run(),
        Commands::Day11 => day11::run(),
        Commands::Day12 => day12::run(),
        Commands::Day13 => day13::run(),
        Commands::Day14 => day14::run(),
        Commands::Day15 => day15::run(),
        Commands::Day16 => day16::run(),
        Commands::Day17 => day17::run(),
        Commands::Day18 => day18::run(),
        Commands::Day19 => day19::run(),
        Commands::Day20 => day20::run(),
        Commands::Day21 => day21::run(),
        Commands::Day22 => day22::run(),
        Commands::Day23 => day23::run(),
        Commands::Day24 => day24::run(),
    }
}
