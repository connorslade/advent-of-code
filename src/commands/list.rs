use anyhow::Result;

use crate::{args::ListArgs, get_year};

pub fn list(cmd: &ListArgs) -> Result<()> {
    let solutions = get_year(cmd.year);
    println!("[*] Solutions for {}:", cmd.year);

    for (i, e) in solutions.iter().enumerate() {
        let last = i + 1 == solutions.len();
        println!(
            " {} Day {}: {}",
            if last { "└" } else { "├" },
            e.day,
            e.name
        );
    }

    Ok(())
}
