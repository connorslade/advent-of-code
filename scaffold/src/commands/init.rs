use std::{
    fs::{self, File},
    io::Write,
    path::Path,
};

use anyhow::Result;
use url::Url;

use crate::{
    args::{Args, InitArgs},
    formatter::Formatter,
    misc::current_year,
    session::{Authenticated, Session},
};

pub fn init(session: &Session, cmd: &InitArgs, args: &Args) -> Result<()> {
    write_input(session, cmd, args)?;
    Ok(())
}

fn write_input(session: &Session, cmd: &InitArgs, args: &Args) -> Result<()> {
    let file_location = Formatter::new(&cmd.input_location)?
        .format::<&[_]>(&[("year", cmd.year), ("day", cmd.day as u16)])?;

    let path = Path::new(&file_location);
    if let Some(parent) = path.parent() {
        if !parent.exists() {
            fs::create_dir_all(parent)?;
        }
    }

    let mut file = File::create(path)?;
    let input = fetch_input(session, &args.address, cmd.day, Some(cmd.year))?;
    file.write_all(input.as_bytes())?;
    println!("[*] Wrote input to {file_location}");
    Ok(())
}

fn fetch_input(session: &Session, base: &Url, day: u8, year: Option<u16>) -> Result<String> {
    let year = year.unwrap_or_else(current_year);
    println!("[*] Fetching input for {day}/{year}");

    let url = base.join(&format!("{year}/day/{day}/input"))?;
    let body = ureq::get(url.as_str())
        .authenticated(session)
        .call()?
        .into_string()?;

    Ok(body)
}
