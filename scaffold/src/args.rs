use clap::Parser;
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
    #[arg(short, long, default_value = "data/{{year}}/{{day:pad(2)}}.txt")]
    pub input_location: String,
    /// A formatter that will be used to get the path for the solution file.
    #[arg(short, long, default_value = "aoc_{{year}}/src/day_{{day:pad(2)}}.rs")]
    pub solution_location: String,
    /// Location formatter of the file importing each solution module.
    #[arg(long, default_value = "aoc_{{year}}/src/lib.rs")]
    pub module_location: String,
    /// A formatter for a new line that will be added to the module file before the marker.
    #[arg(long, default_values_t = ["mod day_{{day:pad(2)}};\n".to_owned(), "&day_{{day:pad(2)}}::Day{{day:pad(2)}},\n    ".to_owned()])]
    pub module_templates: Vec<String>,
    /// A marker is a string that will be found in the module file and is used to determine where to insert the new line.
    /// If not provided, the default markers will be used.
    #[arg(long, default_values_t = ["// [import_marker]".to_owned(), "// [list_marker]".to_owned()])]
    pub module_markers: Vec<String>,
    /// Path to a template file that will be used to create the solution file.
    /// If not provided, a default template will be used.
    #[arg(short = 't', long)]
    pub solution_template: Option<String>,
    /// Don't create a solution file.
    /// Useful if you want to use this command with a different language or organization.
    #[arg(short, long)]
    pub no_scaffold: bool,
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
    #[arg(value_parser = parse_part)]
    pub part: Part,
    /// The year to submit the solution for.
    #[arg(default_value_t = current_year())]
    pub year: u16,
}

#[derive(Debug, Clone, Copy)]
pub enum Part {
    A,
    B,
}

fn parse_part(s: &str) -> Result<Part, String> {
    match s {
        "a" => Ok(Part::A),
        "b" => Ok(Part::B),
        _ => Err("part must be `a` or `b`".to_owned()),
    }
}

impl ToString for Part {
    fn to_string(&self) -> String {
        match self {
            Part::A => "a",
            Part::B => "b",
        }
        .to_owned()
    }
}
