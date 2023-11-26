use std::fs;

pub trait Solution {
    fn name(&self) -> &'static str;
    fn part_a(&self) -> String;
    fn part_b(&self) -> String;

    fn is_dummy(&self) -> bool {
        false
    }
}

/// Load the input for the given year and day.
/// Removes carriage returns and trims leading and trailing whitespace.
pub fn load(year: u32, day: u32) -> String {
    load_raw(year, day).trim().replace('\r', "")
}

/// Load the input for the given year and day.
pub fn load_raw(year: u32, day: u32) -> String {
    let file = format!("data/{year}/{:02}.txt", day);
    fs::read_to_string(&file).unwrap_or_else(|_| panic!("Error reading file {}", file))
}

pub struct DummySolution;

impl Solution for DummySolution {
    fn name(&self) -> &'static str {
        unreachable!()
    }

    fn part_a(&self) -> String {
        unreachable!()
    }

    fn part_b(&self) -> String {
        unreachable!()
    }

    fn is_dummy(&self) -> bool {
        true
    }
}
