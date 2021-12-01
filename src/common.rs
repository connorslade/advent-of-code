use std::fs;

pub fn load(day: &str) -> String {
    let file = format!("data/{}.txt", day);
    fs::read_to_string(&file).expect(&format!("Error reading file {}", file))
}
