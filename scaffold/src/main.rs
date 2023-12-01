use anyhow::Result;
use args::{Args, SubCommand};
use clap::Parser;

use crate::session::Session;

mod args;
#[macro_use]
mod misc;
mod commands;
mod formatter;
mod session;

const TOKEN_VAR: &str = "AOC_TOKEN";

fn main() -> Result<()> {
    dotenv::dotenv()?;
    let args = Args::parse();

    let session = match &args.token {
        Some(token) => Ok(Session::new(token)),
        None => Session::from_env(),
    };

    match &args.subcommand {
        SubCommand::Verify => commands::verify::verify(&session?, &args.address)?,
        SubCommand::Token(e) => commands::token::token(&session.ok(), e, &args)?,
        SubCommand::Timer(e) => commands::timer::timer(e)?,
        SubCommand::Init(e) => commands::init::init(&session?, e, &args)?,
        SubCommand::Submit(e) => commands::submit::submit(&session?, e, &args)?,
    }

    Ok(())
}
