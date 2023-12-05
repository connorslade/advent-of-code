use anyhow::Result;
use clap::Parser;

use args::{Args, Commands};
use common::Solution;
mod args;
mod commands;

fn main() -> Result<()> {
    let args = Args::parse();

    match &args.command {
        Commands::Run(cmd) => commands::run::run(cmd)?,
        Commands::List(cmd) => commands::list::list(cmd)?,
    }

    Ok(())
}

fn get_year(year: u16) -> &'static [&'static dyn Solution] {
    match year {
        2021 => &aoc_2021::ALL,
        2022 => &aoc_2022::ALL,
        2023 => aoc_2023::ALL,
        _ => &[],
    }
}
