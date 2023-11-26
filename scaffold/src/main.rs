use anyhow::Result;
use args::{Args, SubCommand};
use clap::Parser;

use crate::session::Session;

mod args;
#[macro_use]
mod misc;
mod commands;
mod session;

const TOKEN_VAR: &str = "AOC_TOKEN";

fn main() -> Result<()> {
    let args = Args::parse();

    match args.subcommand {
        SubCommand::Verify => commands::verify::verify(&session(args.token)?, &args.address)?,
        SubCommand::Token { token } => {
            commands::token::token(&session(args.token).ok(), token, &args.address)?
        }
        SubCommand::Init { day, year } => commands::init::init(&session(args.token)?, day, year)?,
    }

    Ok(())
}

fn session(token: Option<String>) -> Result<Session> {
    match token {
        Some(token) => Ok(Session::new(token)),
        None => Session::from_env(),
    }
}
