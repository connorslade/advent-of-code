use crate::{problem, Solution};

pub struct Day25;

impl Solution for Day25 {
    fn name(&self) -> &'static str {
        "Full of Hot Air"
    }

    fn part_a(&self) -> String {
        let raw = problem::load(2022, 25);
        snafu::encode(raw.lines().map(snafu::decode).sum::<i64>()).to_string()
    }

    fn part_b(&self) -> String {
        String::new()
    }
}

mod snafu {

    pub fn decode(s: &str) -> i64 {
        let mut value = 0;

        for (i, c) in s.chars().rev().enumerate() {
            value += match c {
                '0'..='2' => c as i64 - '0' as i64,
                '-' => -1,
                '=' => -2,
                _ => panic!("Invalid character"),
            } * 5_i64.pow(i as u32);
        }

        value
    }

    pub fn encode(real: i64) -> String {
        let mut out = String::new();
        let mut num = real;

        while num > 0 {
            let index = (num % 5) as usize;
            out.push("012=-".as_bytes()[index] as char);
            num -= [0, 1, 2, -2, -1][index];
            num /= 5;
        }

        out.chars().rev().collect::<String>()
    }
}
