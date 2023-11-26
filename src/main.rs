use std::time::Instant;

use ::common::Solution;
use clap::{Parser, Subcommand};

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
            let day = day.saturating_sub(1);

            let solutions = get_year(year);
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
            println!("[+] OUT: {} ({})", out, time_unit(time));
        }
        Commands::List { year } => {
            let year = year.unwrap_or(DEFAULT_YEAR);
            let solutions = get_year(year);
            println!("[*] Solutions for {year}:");

            for (i, e) in solutions.iter().enumerate() {
                println!(
                    " {} Day {}: {}",
                    if i + 1 == solutions.len() {
                        "└"
                    } else {
                        "├"
                    },
                    i + 1,
                    e.name()
                );
            }
        }
    }
}

fn get_year(year: u32) -> &'static [&'static dyn Solution] {
    match year {
        2021 => &aoc_2021::ALL,
        2022 => &aoc_2022::ALL,
        _ => &[],
    }
}

const TIME_UNITS: &[&str] = &["ns", "μs", "ms", "s"];

pub fn time_unit(time: u128) -> String {
    let mut time = time;
    for i in TIME_UNITS {
        if time < 1000 {
            return format!("{}{}", time, i);
        }
        time /= 1000;
    }

    format!("{}{}", time, TIME_UNITS.last().unwrap())
}
