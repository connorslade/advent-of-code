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

    let session = match args.token {
        Some(token) => Ok(Session::new(token)),
        None => Session::from_env(),
    };

    match args.subcommand {
        SubCommand::Verify => commands::verify::verify(&session?, &args.address)?,
        SubCommand::Token { token } => commands::token::token(&session.ok(), token, &args.address)?,
        SubCommand::Timer(args) => commands::timer::timer(args)?,
        _ => todo!(),
    }

    Ok(())
}
