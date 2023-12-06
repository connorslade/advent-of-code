use common::{Answer, Solution};
use itertools::Itertools;

pub struct Day06;

impl Solution for Day06 {
    fn name(&self) -> &'static str {
        "Wait For It"
    }

    fn part_a(&self, input: &str) -> Answer {
        parse_a(input)
            .iter()
            .map(Race::ways_to_win)
            .product::<u64>()
            .into()
    }

    fn part_b(&self, input: &str) -> Answer {
        parse_b(input).ways_to_win().into()
    }
}

#[derive(Debug)]
struct Race {
    time: u64,
    distance: u64,
}

fn parse_a(input: &str) -> Vec<Race> {
    let (a, b) = input
        .lines()
        .map(|x| {
            x.split_whitespace()
                .skip(1)
                .map(|x| x.parse::<u64>().unwrap())
        })
        .next_tuple()
        .unwrap();
    a.zip(b)
        .map(|(time, distance)| Race { time, distance })
        .collect::<Vec<_>>()
}

fn parse_b(input: &str) -> Race {
    let (time, distance) = input
        .lines()
        .map(|x| {
            x.split_whitespace()
                .skip(1)
                .collect::<String>()
                .parse::<u64>()
                .unwrap()
        })
        .next_tuple()
        .unwrap();

    Race { time, distance }
}

impl Race {
    fn ways_to_win(&self) -> u64 {
        let mut out = 0;

        for i in 0..self.time {
            let distance = i * (self.time - i);
            if distance > self.distance {
                out += 1;
            }
        }

        out
    }
}

#[cfg(test)]
mod test {
    use common::Solution;
    use indoc::indoc;

    use super::Day06;

    const CASE: &str = indoc! {"
        Time:      7  15   30
        Distance:  9  40  200
    "};

    #[test]
    fn part_a() {
        assert_eq!(Day06.part_a(CASE), 288.into());
    }

    #[test]
    fn part_b() {
        assert_eq!(Day06.part_b(CASE), 71503.into());
    }
}
