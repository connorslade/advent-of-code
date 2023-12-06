use common::{Answer, Solution};

pub struct Day06;

impl Solution for Day06 {
    fn name(&self) -> &'static str {
        "Wait For It"
    }

    fn part_a(&self, input: &str) -> Answer {
        let races = parse(input);
        races
            .iter()
            .map(|x| x.ways_to_win())
            .fold(1, |a, b| a * b)
            .into()
    }

    fn part_b(&self, input: &str) -> Answer {
        let race = Race {
            time: 44826981,
            distance: 202107611381458,
        };

        race.ways_to_win().into()
    }
}

#[derive(Debug)]
struct Race {
    time: u64,
    distance: u64,
}

fn parse(input: &str) -> Vec<Race> {
    let mut out = Vec::new();

    let mut lines = input.lines();
    let time_races = lines.next().unwrap().split_whitespace().skip(1);
    let distance_races = lines.next().unwrap().split_whitespace().skip(1);
    for (time, distance) in time_races.zip(distance_races) {
        let race = Race {
            time: time.parse::<u64>().unwrap(),
            distance: distance.parse::<u64>().unwrap(),
        };
        out.push(race);
    }

    out
}

impl Race {
    fn ways_to_win(&self) -> u32 {
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
