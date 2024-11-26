use std::{
    io::{self, Write},
    thread,
    time::Duration,
};

use anyhow::{Context, Result};
use chrono::{DateTime, Datelike, Utc};

use crate::args::TimerArgs;

/// The timezone of the Advent of Code release.
const AOC_TIMEZONE: u32 = 5;

pub fn timer(cmd: &TimerArgs) -> Result<()> {
    let mut stop_time = next_release()?;

    if let Some(offset) = cmd.offset {
        stop_time += chrono::Duration::seconds(offset as i64);
    }

    if Utc::now() >= stop_time {
        println!("[*] The next puzzle has already been released.");

        if cmd.offset.is_some() {
            println!("[*] Note: This may be because of the offset you set");
        }

        return Ok(());
    }

    if cmd.quiet {
        println!("[*] Waiting...");
    } else {
        println!("[*] The next puzzle will be released in:");
    }

    loop {
        let now = Utc::now();
        if now >= stop_time {
            break;
        }

        if !cmd.quiet {
            let time_left = (stop_time - now).to_std()?;
            let time_left = Duration::new(time_left.as_secs(), 0);
            print!("\r\x1b[0K[*]  {}", humantime::format_duration(time_left));
            io::stdout().flush()?;
        }

        thread::sleep(Duration::from_secs_f32(cmd.frequency));
    }

    Ok(())
}

fn next_release() -> Result<DateTime<Utc>> {
    let mut next = Utc::now()
        .date_naive()
        .and_hms_opt(AOC_TIMEZONE, 0, 0)
        .unwrap()
        .and_utc();

    let before_event = next.month() != 12;
    let after_event = next.month() == 12 && next.day() > 25;
    if after_event || before_event {
        next = next
            .with_month(12)
            .and_then(|x| x.with_day(1))
            .context("Can not represent the next first of December.")?;
    }

    if after_event {
        next = next
            .with_year(next.year() + 1)
            .context("Can not represent the next first of December.")?;
    }

    if Utc::now() > next {
        next = next
            .date_naive()
            .succ_opt()
            .unwrap()
            .and_time(next.time())
            .and_utc();
    }

    Ok(next)
}
