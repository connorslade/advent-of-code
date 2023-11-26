use anyhow::Result;
use args::{Args, SubCommand};
use clap::Parser;

use crate::session::Session;

mod args;
#[macro_use]
mod misc;
mod commands;
mod session;

fn main() -> Result<()> {
    let args = Args::parse();
    let session = match args.token {
        Some(token) => Session::new(token),
        None => Session::from_env()?,
    };

    match args.subcommand {
        SubCommand::Verify => commands::verify::verify(&session, &args.address)?,
        SubCommand::Init { day, year } => {
            todo!()
        }
    }

    Ok(())
}
