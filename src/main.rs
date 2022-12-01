use std::time::Instant;

use clap::{Parser, Subcommand};

use problem::Solution;
mod common;
mod problem;
mod solutions;

#[derive(Parser)]
#[command(name = "advent_of_code")]
#[command(author = "Connor Slade <connor@connorcode.com>")]
struct Cli {
    #[command(subcommand)]
    command: Commands,
}

#[derive(Subcommand)]
enum Commands {
    #[command(about = "Run a solution to a problem")]
    Run {
        day: u32,
        part: char,
        year: Option<u32>,
    },
    #[command(about = "List all solutions for a given year")]
    List { year: Option<u32> },
}

const DEFAULT_YEAR: u32 = 2022;

fn main() {
    let args = Cli::parse();

    match args.command {
        Commands::Run { year, day, part } => {
            let year = year.unwrap_or(DEFAULT_YEAR);
            let solutions = solutions::get_year(year);
            let solution = match solutions.get(day as usize) {
                Some(s) => s,
                None => {
                    println!("No solution for day {} in year {}", day, year);
                    return;
                }
            };

            println!("[*] Running: {} ({})", solution.name(), part.to_uppercase());

            let start = Instant::now();
            let out = match part.to_lowercase().to_string().as_str() {
                "a" => solution.part_a(),
                "b" => solution.part_b(),
                _ => return println!("[-] Invalid Part {}", part),
            };

            let time = start.elapsed().as_nanos();
            println!("[+] OUT: {} ({})", out, common::time_unit(time));
        }
        Commands::List { year } => {
            let year = year.unwrap_or(DEFAULT_YEAR);
            let solutions = solutions::get_year(year);
            println!("[*] Solutions for {year}:");

            for (i, e) in solutions.iter().enumerate() {
                println!(
                    " {} Day {}: {}",
                    if i + 1 == solutions.len() {
                        "└"
                    } else {
                        "├"
                    },
                    i,
                    e.name()
                );
            }
        }
    }
}
