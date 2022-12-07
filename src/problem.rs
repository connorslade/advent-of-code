use std::fs;

pub trait Solution {
    fn name(&self) -> &'static str;
    fn part_a(&self) -> String;
    fn part_b(&self) -> String;
}

pub fn load(year: u32, day: u32) -> String {
    let file = format!("data/{year}/{:02}.txt", day);
    fs::read_to_string(&file)
        .unwrap_or_else(|_| panic!("Error reading file {}", file))
        .trim()
        .replace('\r', "")
}
