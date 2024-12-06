use std::{fs, path::PathBuf, time::Instant};

use anyhow::Result;
use common::Part;

use crate::{args::RunAllArgs, get_year};

pub fn run(cmd: &RunAllArgs) -> Result<()> {
    let solutions = get_year(cmd.year);

    println!("[*] Running all {} solutions\n", cmd.year);

    for solution in solutions {
        let path = PathBuf::from(format!("data/{}/{:02}.txt", cmd.year, solution.day));
        let input = fs::read_to_string(&*path)?.trim().replace('\r', "");

        println!("[*] Day {}", solution.day);

        for part in [Part::A, Part::B] {
            let start = Instant::now();
            let out = match part {
                Part::A => (solution.part_a)(&input),
                Part::B => (solution.part_b)(&input),
            };

            println!(" | Part {part}: {} ({:.2?})", out, start.elapsed());
        }
    }

    Ok(())
}
