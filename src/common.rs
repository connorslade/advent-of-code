use std::fs;
use std::io;
use std::io::Write;

const TIME_UNITS: &[&str] = &["ns", "μs", "ms", "s"];

pub fn load(day: &str) -> String {
    let file = format!("data/{}.txt", day);
    fs::read_to_string(&file).unwrap_or_else(|_| panic!("Error reading file {}", file))
}

pub trait Solution {
    fn name(&self) -> String;
    fn part_a(&self) -> String;
    fn part_b(&self) -> String;
}

pub fn time_unit(time: u128) -> String {
    let mut time = time;
    for i in TIME_UNITS {
        if time < 1000 {
            return format!("{}{}", time, i);
        }
        time /= 1000;
    }

    format!("{}{}", time, TIME_UNITS.last().unwrap())
}

pub fn input(inp: &str) -> Option<String> {
    print!("{}", inp);

    let mut buff = String::new();
    io::stdout().flush().ok()?;
    io::stdin().read_line(&mut buff).ok()?;
    while buff.ends_with('\n') || buff.ends_with('\r') {
        buff.pop();
    }

    Some(buff)
}
