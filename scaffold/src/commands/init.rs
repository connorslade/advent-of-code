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
    if !cmd.no_scaffold {
        write_scaffold(cmd, args)?;
    }

    write_input(session, cmd, args)?;
    Ok(())
}

fn write_scaffold(cmd: &InitArgs, args: &Args) -> Result<()> {
    let args: &[(&str, u16)] = &[("year", cmd.year), ("day", cmd.day as u16)];
    let file_location = Formatter::new(&cmd.solution_location)?.format(args)?;
    let mut file = create_file(&Path::new(&file_location))?;

    println!("[*] Loading template");
    let template = match cmd.solution_template {
        Some(ref path) => fs::read_to_string(path)?,
        None => include_str!("../../template.txt").to_owned(),
    };
    let template = Formatter::new(&template)?.format(args)?;

    file.write_all(template.as_bytes())?;
    println!("[*] Wrote scaffold to {file_location}");
    Ok(())
}

fn write_input(session: &Session, cmd: &InitArgs, args: &Args) -> Result<()> {
    let file_location = Formatter::new(&cmd.input_location)?
        .format::<&[_]>(&[("year", cmd.year), ("day", cmd.day as u16)])?;

    let mut file = create_file(&Path::new(&file_location))?;
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

fn create_file(path: &Path) -> Result<File> {
    if let Some(parent) = path.parent() {
        if !parent.exists() {
            fs::create_dir_all(parent)?;
        }
    }

    Ok(File::create(path)?)
}
