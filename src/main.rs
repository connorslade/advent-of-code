use std::env;

mod common;
mod solutions;

fn main() {
    println!("Advent of Code 2021 Solutions");
    println!("        Connor Slade        \n");

    // Use run args for day and part
    // Run like: cargo run -- <day><a | b>
    // Ex: cargo run -- 0a
    if let Some(run_arg) = env::args().nth(1) {
        let part = run_arg.chars().last().unwrap().to_string();
        let mut run_arg = run_arg.chars();
        run_arg.next_back().unwrap();
        return run(run_arg.as_str().parse().unwrap(), part);
    };

    for (i, item) in solutions::ALL.iter().enumerate() {
        println!("[{}] {}", i, item.name());
    }

    let run_index = common::input("\nIndex ❯ ").unwrap();
    let run_index = match run_index.parse::<usize>() {
        Ok(i) => i,
        Err(_) => return println!("Das not a number..."),
    };

    if run_index >= solutions::ALL.len() {
        return println!("[*] Invaild Id");
    }

    let part = common::input("Part (A / B) ❯ ").unwrap();
    run(run_index, part);
}

fn run(run_index: usize, part: String) {
    let this_sol = solutions::ALL[run_index];

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
