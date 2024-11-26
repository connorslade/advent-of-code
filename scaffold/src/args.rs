use std::{borrow::Cow, path::PathBuf, str::FromStr};

use clap::Parser;
use common::Part;
use regex::Regex;
use url::Url;

use crate::misc::current_year;

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
    /// Update the token stored in `AOC_TOKEN`.
    Token(TokenArgs),
    /// Waits for the next midnight (EST) from December first to the twenty-fifth then returns.
    /// Chaining this command with another command, like init, will ensure that the input is fetched as soon as it is available.
    Timer(TimerArgs),
    /// Fetch the puzzle input for a given day and write to a file.
    /// Also creates a base solution file for the given day.
    Init(InitArgs),
    /// Submit a solution to the Advent of Code server.
    Submit(SubmitArgs),
}

#[derive(Parser, Debug)]
pub struct TokenArgs {
    /// The session token you grabbed from the website.
    pub token: String,
}

#[derive(Parser, Debug)]
pub struct TimerArgs {
    /// Time in seconds to offset the timer by.
    /// A positive value will make the timer start later.
    #[arg(short, long)]
    pub offset: Option<f32>,
    /// The frequency in seconds to print the timer to stdout.
    #[arg(short, long, default_value_t = 1.0)]
    pub frequency: f32,
    /// Don't print the timer to stdout.
    #[arg(short, long)]
    pub quiet: bool,
}

#[derive(Parser, Debug)]
pub struct InitArgs {
    /// A formatter that will be used to get the path for the input file.
    #[arg(short = 'l', long, default_value = "data/{{year}}/{{day:pad(2)}}.txt")]
    pub input_location: String,
    /// An inserter is a string that will be inserted into a file at a specific marker.
    /// The argument is formatted as `{location}:{marker}:{template}` or if you want to use multiple markers `{location}:{marker}:{template}:{marker}:{template}:...`.
    /// This argument can be provided multiple times.
    /// Some uses of inserters are automatically importing the new source into a module or adding the day to the list of days.
    #[arg(short, long)]
    pub inserter: Vec<Insertion>,
    /// Cancel adding an inserter to import the newly scaffolded into its years's lib.rs and adding it to the SOLUTIONS array.
    /// Equivalent to `"aoc_{{year}}/src/lib.rs|// [import_marker]|mod day_{{day:pad(2)}};\\n|// [list_marker]|day_{{day:pad(2)}}::SOLUTION,\\n    "`.
    #[arg(long)]
    pub no_default_insertions: bool,
    /// Location formatter of the file importing each solution module.
    #[arg(long, default_value = "aoc_{{year}}/src/lib.rs")]
    pub module_location: String,
    /// A formatter that will be used to get the path for the solution file.
    #[arg(short, long, default_value = "aoc_{{year}}/src/day_{{day:pad(2)}}.rs")]
    pub solution_location: String,
    /// Path to a template file that will be used to create the solution file.
    /// If not provided, a default template will be used.
    #[arg(short = 't', long)]
    pub solution_template: Option<PathBuf>,
    /// Don't create a solution file.
    /// Useful if you want to use this command with a different language or organization.
    #[arg(short, long)]
    pub no_scaffold: bool,
    /// Allows overwriting the existing solution file.
    #[arg(long)]
    pub allow_overwrite: bool,
    /// Automatically open the solution file in your editor.
    /// Only works if you are not using `--no-scaffold`.
    /// Configure the editor with the `--editor` argument.
    #[arg(short, long)]
    pub auto_open: bool,
    /// Command to open a file in your editor.
    #[arg(short, long, default_value = "code {{file}}")]
    pub editor: String,

    /// The day to fetch the input for.
    pub day: u8,
    /// The year to fetch the input for.
    #[arg(default_value_t = current_year())]
    pub year: u16,
}

#[derive(Parser, Debug)]
pub struct SubmitArgs {
    /// Command to run to get the solution for the given day.
    #[arg(
        short,
        long,
        default_value = "cargo r -r -- run {{day}} {{part}} {{year}}"
    )]
    pub command: String,
    /// A regex that will be used to extract the solution from the output of the command.
    #[arg(long, default_value = r"OUT: (.*) \(")]
    pub extraction_regex: Regex,
    /// The group of the regex that contains the solution.
    #[arg(long, default_value = "1")]
    pub extraction_group: usize,
    /// Don't actually submit the solution.
    /// Useful for testing that the command and extraction regex are correct.
    #[arg(short, long)]
    pub dry_run: bool,

    /// The day to submit the solution for.
    pub day: u8,
    /// The part to submit the solution for.
    pub part: Part,
    /// The year to submit the solution for.
    #[arg(default_value_t = current_year())]
    pub year: u16,
}

#[derive(Debug, Clone)]
pub struct Insertion {
    pub location: Cow<'static, str>,
    // (Marker, Template)
    pub parts: Vec<(Cow<'static, str>, Cow<'static, str>)>,
}

impl FromStr for Insertion {
    type Err = &'static str;

    /// `{location}:{marker}:{template}`
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let mut parts = s.split('|').collect::<Vec<_>>();

        if parts.len() < 3 {
            return Err("Expected at least 3 parts");
        }

        let location = parts.remove(0).to_owned();
        let parts = parts
            .chunks_exact(2)
            .map(|x| (x[0].to_owned().into(), x[1].to_owned().into()))
            .collect::<Vec<_>>();

        Ok(Self {
            location: Cow::Owned(location),
            parts,
        })
    }
}
