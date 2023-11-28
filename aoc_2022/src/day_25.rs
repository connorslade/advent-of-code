use common::{Answer, Solution};

pub struct Day25;

impl Solution for Day25 {
    fn name(&self) -> &'static str {
        "Full of Hot Air"
    }

    fn part_a(&self, input: &str) -> Answer {
        snafu::encode(input.lines().map(snafu::decode).sum::<i64>()).into()
    }

    fn part_b(&self, _input: &str) -> Answer {
        // No part b for day 25!
        Answer::Unimplemented
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

#[cfg(test)]
mod test {
    use indoc::indoc;

    use super::Day25;
    use common::Solution;

    const CASE: &str = indoc! {r"
        1=-0-2
        12111
        2=0=
        21
        2=01
        111
        20012
        112
        1=-1=
        1-12
        12
        1=
        122
    "};

    #[test]
    fn part_a() {
        assert_eq!(Day25.part_a(CASE), "2=-1=0".into());
    }
}
