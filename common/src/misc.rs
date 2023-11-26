use std::{fs, io};

/// Load the input for the given year and day.
/// Removes carriage returns and trims leading and trailing whitespace.
pub fn load(year: u32, day: u32) -> io::Result<String> {
    load_raw(year, day).map(|x| x.trim().replace('\r', ""))
}

/// Load the input for the given year and day.
pub fn load_raw(year: u32, day: u32) -> io::Result<String> {
    let file = format!("data/{year}/{:02}.txt", day);
    fs::read_to_string(&file)
}
