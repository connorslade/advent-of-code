use std::{fs, time::Instant};

use anyhow::{Context, Result};
use common::{human_time, Part};

use crate::{args::RunArgs, get_year};

pub fn run(cmd: &RunArgs) -> Result<()> {
    let day = cmd.day.saturating_sub(1);

    let solutions = get_year(cmd.year);
    let solution = solutions
        .get(day as usize)
        .with_context(|| format!("No solution for day {} in year {}", day, cmd.year))?;

    println!(
        "[*] Running: {} ({})",
        solution.name(),
        cmd.part.to_string().to_uppercase()
    );
    let input = match &cmd.input {
        Some(path) => fs::read_to_string(path)?.trim().replace('\r', ""),
        None => common::load(cmd.year, day + 1)?,
    };

    let start = Instant::now();
    let out = match cmd.part {
        Part::A => solution.part_a(&input),
        Part::B => solution.part_b(&input),
    };

    if cmd.raw {
        println!("{out}");
    } else {
        let time = start.elapsed().as_nanos();
        println!("[+] OUT: {} ({})", out, human_time(time));
    }

    Ok(())
}
