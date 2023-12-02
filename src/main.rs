use clap::{Parser, Subcommand};

mod day1;
mod day2;
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
}

fn main() {
    let args = Cli::parse();

    match args.command {
        Commands::Day1 => {
            day1::run();
        }
        Commands::Day2 => {
            day2::run();
        }
    }
}
