use common::Solution;

pub struct Day01;

impl Solution for Day01 {
    fn name(&self) -> &'static str {
        "Sonar Sweep"
    }

    fn part_a(&self, input: &str) -> String {
        let data = input
            .lines()
            .map(|x| x.parse::<u32>().unwrap())
            .collect::<Vec<u32>>();

        let inc = data.windows(2).filter(|x| x[0] < x[1]).count();
        inc.to_string()
    }

    fn part_b(&self, input: &str) -> String {
        let d = input
            .lines()
            .map(|x| x.parse::<u32>().unwrap())
            .collect::<Vec<u32>>();

        let inc = d.windows(4).filter(|x| x[2] > x[0]).count();
        inc.to_string()
    }
}
