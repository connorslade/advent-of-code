use crate::common::{self, Solution};

pub struct Day09 {}

impl Solution for Day09 {
    fn name(&self) -> String {
        "".to_owned()
    }

    fn part_a(&self) -> String {
        let data = parse(common::load("09"));
        let low = lowest(data);

        low.iter().map(|x| *x + 1).sum::<u32>().to_string()
    }

    fn part_b(&self) -> String {
        let data = parse(common::load("09"));

        "".to_string()
    }
}

fn parse(inp: String) -> Vec<Vec<u32>> {
    let mut out = Vec::new();

    for i in inp.lines() {
        out.push(
            i.chars()
                .map(|x| x.to_digit(10).unwrap())
                .collect::<Vec<u32>>(),
        );
    }

    out
}

fn lowest(inp: Vec<Vec<u32>>) -> Vec<u32> {
    let mut out = Vec::new();

    for i in 0..inp.len() {
        for j in 0..inp[i].len() {}
    }

    out
}
