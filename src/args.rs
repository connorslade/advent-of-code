use clap::{Parser, Subcommand};

#[derive(Parser)]
#[command(name = "advent_of_code")]
#[command(author = "Connor Slade <connor@connorcode.com>")]
pub struct Args {
    #[command(subcommand)]
    pub command: Commands,
}

#[derive(Subcommand)]
pub enum Commands {
    #[command(about = "Run a solution to a problem")]
    Run {
        day: u32,
        part: char,
        year: Option<u32>,
    },
    #[command(about = "List all solutions for a given year")]
    List { year: Option<u32> },
}
