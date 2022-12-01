use crate::{problem, Solution};

pub struct Day01;

impl Solution for Day01 {
    fn name(&self) -> &'static str {
        "Calorie Counting"
    }

    fn part_a(&self) -> String {
        let raw = problem::load(2022, 1);
        let elfs = get_elfs(&raw);

        elfs.last().unwrap().to_string()
    }

    fn part_b(&self) -> String {
        let raw = problem::load(2022, 1);
        let elfs = get_elfs(&raw);

        let mut sum = 0;
        for i in elfs.iter().rev().take(3) {
            sum += i;
        }

        sum.to_string()
    }
}

fn get_elfs(data: &str) -> Vec<u32> {
    let mut out = data
        .replace('\r', "")
        .split("\n\n")
        .map(|x| x.lines().map(|x| x.parse::<u32>().unwrap()).sum())
        .collect::<Vec<_>>();
    out.sort();
    out
}
