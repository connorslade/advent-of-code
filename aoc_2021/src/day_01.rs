use common::{Answer, Solution};

pub struct Day01;

impl Solution for Day01 {
    fn name(&self) -> &'static str {
        "Sonar Sweep"
    }

    fn part_a(&self, input: &str) -> Answer {
        let data = input
            .lines()
            .map(|x| x.parse::<u32>().unwrap())
            .collect::<Vec<u32>>();

        data.windows(2).filter(|x| x[0] < x[1]).count().into()
    }

    fn part_b(&self, input: &str) -> Answer {
        let d = input
            .lines()
            .map(|x| x.parse::<u32>().unwrap())
            .collect::<Vec<u32>>();

        d.windows(4).filter(|x| x[2] > x[0]).count().into()
    }
}
