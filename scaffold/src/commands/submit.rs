use std::{
    process::{Command, Stdio},
    time::Instant,
};

use anyhow::{Context, Result};
use common::human_time;
use scraper::Html;

use crate::{
    args::{Args, SubmitArgs},
    formatter::Formatter,
    session::{Authenticated, Session},
};

pub fn submit(session: &Session, cmd: &SubmitArgs, args: &Args) -> Result<()> {
    let answer = get_answer(cmd).context("Getting answer")?;

    if cmd.dry_run {
        println!("[*] Aborting due to dry run");
        return Ok(());
    }

    submit_answer(session, cmd, args, &answer).context("Submitting answer")?;
    Ok(())
}

fn get_answer(cmd: &SubmitArgs) -> Result<String> {
    let formats: &[(&str, String)] = &[
        ("day", cmd.day.to_string()),
        ("year", cmd.year.to_string()),
        ("part", cmd.part.to_string()),
    ];
    let command = Formatter::new(&cmd.command)?.format(formats)?;
    let args = shell_words::split(&command)?;
    let executable = which::which(&args[0])?;

    if cmd.dry_run {
        println!("[*] Running command: {}", command);
    }

    let executable = Command::new(&executable)
        .args(&args[1..])
        .stdout(Stdio::piped())
        .stderr(Stdio::piped())
        .spawn()?;

    let start = Instant::now();
    let output = executable.wait_with_output()?;
    let time = start.elapsed().as_nanos();

    if output.status.code() != Some(0) {
        anyhow::bail!(
            "Command failed with status code {}\n{}",
            output.status,
            String::from_utf8_lossy(&output.stderr)
        );
    }

    let output = String::from_utf8_lossy(&output.stdout);
    let answer = cmd
        .extraction_regex
        .captures(&output)
        .context("Failed to extract answer, regex didn't match")?
        .get(cmd.extraction_group)
        .context("Failed to extract answer, too few capture groups")?
        .as_str()
        .trim()
        .to_owned();

    println!("[*] Answer: `{answer}` ({})", human_time(time));

    Ok(answer)
}

fn submit_answer(session: &Session, cmd: &SubmitArgs, args: &Args, answer: &str) -> Result<()> {
    let url = args
        .address
        .join(&format!("{}/day/{}/answer", cmd.year, cmd.day))?;

    // POST https://adventofcode.com/{{year}}/day/{{day}}/answer
    // level={{part:int}}&answer={{answer}}
    let result = ureq::post(url.as_str())
        .authenticated(session)
        .send_form(&[
            ("level", &(cmd.part as u8 + 1).to_string()),
            ("answer", answer),
        ])?;

    let document = Html::parse_document(&result.into_string()?);
    let result = document
        .select(selector!("article p"))
        .next()
        .context("No response message found")?;
    let result_text = result.text().collect::<Vec<_>>().join("");

    // Remove duplicate whitespace
    let result_text = regex!(r"[\[\(].*?[\]\)]").replace_all(&result_text, "");
    let result_text = regex!(r"\s+").replace_all(&result_text, " ");

    println!("[*] {result_text}");
    Ok(())
}
