use crate::common::{self, Solution};

pub struct Day09 {}

impl Solution for Day09 {
    fn name(&self) -> String {
        "Smoke Basin".to_owned()
    }

    fn part_a(&self) -> String {
        let data = parse(common::load("09"));
        let low = lowest(data);

        low.iter().map(|x| *x + 1).sum::<u32>().to_string()
    }

    fn part_b(&self) -> String {
        let data = common::load("09");

        "".to_string()
    }
}

fn parse(inp: String) -> Vec<Vec<u32>> {
    inp.lines()
        .map(|x| x.chars().map(|f| f.to_digit(10).unwrap()).collect())
        .collect()
}

fn lowest(inp: Vec<Vec<u32>>) -> Vec<u32> {
    inp.iter()
        .enumerate()
        .flat_map(|(i, line)| {
            let inp = &inp;
            line.iter().enumerate().filter_map(move |(j, &h)| {
                if (i == 0 || h < inp[i - 1][j])
                    && (i == inp.len() - 1 || h < inp[i + 1][j])
                    && (j == 0 || h < inp[i][j - 1])
                    && (j == line.len() - 1 || h < inp[i][j + 1])
                {
                    return Some(h);
                }
                None
            })
        })
        .collect::<Vec<u32>>()
}
