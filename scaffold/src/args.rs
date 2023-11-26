use clap::Parser;
use url::Url;

#[derive(Parser, Debug)]
pub struct Args {
    /// The session token to use for the request.
    /// If not provided, the token will be read from the environment variable `AOC_TOKEN`.
    #[arg(short, long)]
    pub token: Option<String>,
    /// The address of the Advent of Code server to use.
    #[arg(short, long, default_value = "https://adventofcode.com")]
    pub address: Url,
    #[command(subcommand)]
    pub subcommand: SubCommand,
}

#[derive(Parser, Debug)]
pub enum SubCommand {
    /// Verify that the session token provided is still valid
    Verify,
    /// Fetch the puzzle input for a given day and write to a file.
    /// Also creates a base solution file for the given day.
    Init { day: u8, year: Option<u16> },
}
