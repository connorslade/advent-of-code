use crate::{problem, Solution};

pub struct Day01;

impl Solution for Day01 {
    fn name(&self) -> &'static str {
        "Sonar Sweep"
    }

    fn part_a(&self) -> String {
        let data = problem::load(2021, 1)
            .lines()
            .map(|x| x.parse::<u32>().unwrap())
            .collect::<Vec<u32>>();

        let inc = data.windows(2).filter(|x| x[0] < x[1]).count();
        inc.to_string()
    }

    fn part_b(&self) -> String {
        let d = problem::load(2021, 1)
            .lines()
            .map(|x| x.parse::<u32>().unwrap())
            .collect::<Vec<u32>>();

        let inc = d.windows(4).filter(|x| x[2] > x[0]).count();
        inc.to_string()
    }
}
