use std::env;

use common::Solution;

mod common;
mod solutions;

const DEFAULT_YEAR: u32 = 2022;

fn main() {
    println!("Advent of Code Solutions");
    println!("      Connor Slade      \n");

    let solutions = solutions::get_year(DEFAULT_YEAR);
    if solutions.is_empty() {
        println!("No solutions for {}", DEFAULT_YEAR);
        return;
    }

    // Use run args for day and part
    // Run like: cargo run -- <day><a | b>
    // Ex: cargo run -- 0a
    if let Some(run_arg) = env::args().nth(1) {
        let part = run_arg.chars().last().unwrap().to_string();
        let mut run_arg = run_arg.chars();
        run_arg.next_back().unwrap();
        return run(solutions, run_arg.as_str().parse().unwrap(), part);
    };

    for (i, item) in solutions.iter().enumerate() {
        println!("[{}] {}", i, item.name());
    }

    let run_index = common::input("\nIndex ❯ ").unwrap();
    let run_index = match run_index.parse::<usize>() {
        Ok(i) => i,
        Err(_) => return println!("Das not a number..."),
    };

    if run_index >= solutions.len() {
        return println!("[*] Invaild Id");
    }

    let part = common::input("Part (A / B) ❯ ").unwrap();
    run(solutions, run_index, part);
}

fn run(solutions: &[&'static dyn Solution], run_index: usize, part: String) {
    let this_sol = solutions[run_index];

    println!("[*] Running: {} ({})", this_sol.name(), part.to_uppercase());

    let start = std::time::Instant::now();
    let out = match part.to_lowercase().as_str() {
        "a" => this_sol.part_a(),
        "b" => this_sol.part_b(),
        _ => return println!("[-] Invalid Part"),
    };
    let time = start.elapsed().as_nanos();

    println!("[+] OUT: {} ({})", out, common::time_unit(time));
}
