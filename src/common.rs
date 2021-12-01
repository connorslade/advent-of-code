use std::fs;

pub fn load(day: &str) -> String {
    let file = format!("data/{}.txt", day);
    fs::read_to_string(&file).expect(&format!("Error reading file {}", file))
}

pub struct Solution {
    pub name: String,
    pub run: fn(),
}

impl Solution {
    pub fn new(name: &str, run: fn()) -> Solution {
        Solution {
            name: name.to_owned(),
            run,
        }
    }
}
