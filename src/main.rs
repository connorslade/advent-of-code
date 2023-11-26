use std::time::Instant;

use args::{Args, Commands};
use clap::Parser;
use common::Solution;
mod args;

const DEFAULT_YEAR: u32 = 2022;

fn main() {
    let args = Args::parse();

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
            println!("[+] OUT: {} ({})", out, human_time(time));
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

pub fn human_time(time: u128) -> String {
    const TIME_UNITS: &[&str] = &["ns", "μs", "ms", "s"];

    let mut time = time;
    for i in TIME_UNITS {
        if time < 1000 {
            return format!("{}{}", time, i);
        }
        time /= 1000;
    }

    format!("{}{}", time, TIME_UNITS.last().unwrap())
}
