use crate::common::{self, Solution};

pub struct Day01 {}

impl Solution for Day01 {
    // const NAME: &'static str = "Sonar Sweep";
    fn name(&self) -> String {
        "Sonar Sweep".to_owned()
    }

    fn part_a(&self) -> String {
        let data = common::load("01")
            .lines()
            .map(|x| x.parse::<u32>().unwrap())
            .collect::<Vec<u32>>();
        let mut inc = 0;

        for i in 1..data.len() {
            if data[i - 1] < data[i] {
                inc += 1;
            }
        }

        inc.to_string()
    }

    fn part_b(&self) -> String {
        let d = common::load("01")
            .lines()
            .map(|x| x.parse::<u32>().unwrap())
            .collect::<Vec<u32>>();
        let mut inc = 0;

        for i in 3..d.len() {
            let a = d[i - 1] + d[i - 2] + d[i - 3];
            let b = d[i] + d[i - 1] + d[i - 2];

            if b > a {
                inc += 1;
            }
        }

        inc.to_string()
    }
}
