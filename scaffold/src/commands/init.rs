use std::{
    fs::{self, File},
    io::Write,
    path::{Path, PathBuf},
    process::Command,
};

use anyhow::{Context, Result};
use scraper::Html;
use url::Url;

use crate::{
    args::{Args, InitArgs},
    formatter::Formatter,
    session::{Authenticated, Session},
};

pub fn init(session: &Session, cmd: &InitArgs, args: &Args) -> Result<()> {
    let input = fetch_input(session, &args.address, cmd.day, cmd.year)?;
    let formats: &[(&str, String)] = &[
        ("year", cmd.year.to_string()),
        ("day", cmd.day.to_string()),
        ("problem_name", input.name.to_string()),
    ];

    write_input(cmd, input, formats)?;

    if !cmd.no_scaffold {
        let path = write_scaffold(cmd, formats)?;
        modify_module(cmd, formats)?;
        run_inserters(cmd, formats)?;

        if cmd.auto_open {
            let command =
                Formatter::new(&cmd.editor)?.format::<&[_]>(&[("file", path.to_string_lossy())])?;
            let args = shell_words::split(&command)?;
            let executable = which::which(&args[0])?;
            println!("[*] Opening solution file");
            Command::new(&executable)
                .args(&args[1..])
                .spawn()
                .with_context(|| {
                    format!(
                        "Opening editor with `{}` [{:?}]",
                        executable.to_string_lossy(),
                        &args[1..]
                    )
                })?;
        }
    }

    Ok(())
}

fn write_scaffold(cmd: &InitArgs, formats: &[(&str, String)]) -> Result<PathBuf> {
    let location = Formatter::new(&cmd.solution_location)?.format(formats)?;
    let file_location = Path::new(&location);
    let mut file = create_file(&file_location, cmd.allow_overwrite)?;

    println!("[*] Loading template");
    let template = match cmd.solution_template {
        Some(ref path) => fs::read_to_string(path)?,
        None => include_str!("../../template.txt").to_owned(),
    };
    let template = Formatter::new(&template)?.format(formats)?;

    file.write_all(template.as_bytes())?;
    println!("[*] Wrote scaffold to {location}");
    Ok(file_location.to_path_buf())
}

fn run_inserters(cmd: &InitArgs, formats: &[(&str, String)]) -> Result<()> {
    for inserter in &cmd.inserter {
        let file_path = Formatter::new(&inserter.location)?.format(formats)?;
        let mut file = fs::read_to_string(&file_path)
            .with_context(|| format!("Failed to read {file_path}"))?;

        for (marker, template) in &inserter.parts {
            let marker = file
                .find(marker)
                .with_context(|| format!("Marker `{}` was not found", marker))?;
            let new_line = Formatter::new(template)?
                .format(formats)?
                .replace("\\n", "\n");
            file.insert_str(marker, &new_line);
        }

        fs::write(&file_path, file)?;
        println!("[*] Modified {}", file_path);
    }

    Ok(())
}

fn modify_module(cmd: &InitArgs, formats: &[(&str, String)]) -> Result<()> {
    let module_file = Formatter::new(&cmd.module_location)?.format(formats)?;
    let mut file = fs::read_to_string(&module_file)?;

    for (marker, template) in cmd.module_markers.iter().zip(cmd.module_templates.iter()) {
        let marker = file
            .find(marker)
            .with_context(|| format!("Marker `{marker}` was not found"))?;
        let new_line = Formatter::new(template)?.format(formats)?;
        file.insert_str(marker, &new_line);
    }

    fs::write(&module_file, file)?;
    println!("[*] Modified module {module_file}");
    Ok(())
}

fn write_input(cmd: &InitArgs, input: ProblemInput, formats: &[(&str, String)]) -> Result<()> {
    let file_location = Formatter::new(&cmd.input_location)?.format(formats)?;
    let mut file = create_file(&Path::new(&file_location), true)?;
    file.write_all(input.body.as_bytes())?;
    println!("[*] Wrote input to {file_location}");
    Ok(())
}

fn fetch_input(session: &Session, base: &Url, day: u8, year: u16) -> Result<ProblemInput> {
    println!("[*] Fetching input for {day}/{year}");

    let input_url = base.join(&format!("{year}/day/{day}/input"))?;
    let problem_url = base.join(&format!("{year}/day/{day}"))?;

    let body = ureq::get(input_url.as_str())
        .authenticated(session)
        .call()?
        .into_string()?;
    let problem = ureq::get(problem_url.as_str()).call()?.into_string()?;

    let problem = Html::parse_document(&problem);
    let title = problem
        .select(selector!("article h2"))
        .next()
        .context("No title found")?;
    let title = title.text().next().context("Title was empty")?;

    let name = regex!(r"--- Day \d+: (.+) ---")
        .captures(&title)
        .map(|x| x.get(1).unwrap().as_str().to_owned())
        .context("Title did not match expected format")?;

    Ok(ProblemInput { name, body })
}

struct ProblemInput {
    name: String,
    body: String,
}

fn create_file(path: &Path, allow_overwrite: bool) -> Result<File> {
    if let Some(parent) = path.parent() {
        if !parent.exists() {
            fs::create_dir_all(parent)?;
        }
    }

    if !allow_overwrite && path.exists() {
        return Err(anyhow::anyhow!("File already exists: {}", path.display()));
    }

    Ok(File::create(path)?)
}
