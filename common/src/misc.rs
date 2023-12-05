use std::{fs, io};

/// Load the input for the given year and day.
/// Removes carriage returns and trims leading and trailing whitespace.
pub fn load(year: u16, day: u32) -> io::Result<String> {
    load_raw(year, day).map(|x| x.trim().replace('\r', ""))
}

/// Load the input for the given year and day.
pub fn load_raw(year: u16, day: u32) -> io::Result<String> {
    let file = format!("data/{year}/{:02}.txt", day);
    fs::read_to_string(file)
}

pub fn human_time(time: u128) -> String {
    const TIME_UNITS: &[&str] = &["ns", "μs", "ms", "s"];

    let mut time = time;
    for i in TIME_UNITS {
        if time < 1000 {
            return format!("{}{}", time, i);
        }
        time /= 1000;
    }

    format!("{}{}", time, TIME_UNITS.last().unwrap())
}
