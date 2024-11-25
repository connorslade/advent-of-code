use common::{solution, Answer};
use itertools::Itertools;

solution!("Wait For It", (2023, 06));

fn part_a(input: &str) -> Answer {
    parse_a(input)
        .iter()
        .map(Race::ways_to_win)
        .product::<u64>()
        .into()
}

fn part_b(input: &str) -> Answer {
    parse_b(input).ways_to_win().into()
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
        let a = ((self.time * self.time - 4 * self.distance) as f32).sqrt();
        let x1 = ((self.time as f32 - a) / 2.0 + 1.0).floor();
        let x2 = ((self.time as f32 + a) / 2.0 - 1.0).ceil();
        (x2 - x1 + 1.0) as u64
    }
}

#[cfg(test)]
mod test {
    use indoc::indoc;

    const CASE: &str = indoc! {"
        Time:      7  15   30
        Distance:  9  40  200
    "};

    #[test]
    fn part_a() {
        assert_eq!(super::part_a(CASE), 288.into());
    }

    #[test]
    fn part_b() {
        assert_eq!(super::part_b(CASE), 71503.into());
    }
}
