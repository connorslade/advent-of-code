use crate::{problem, Solution};

use std::collections::HashMap;
use std::hash::Hash;

pub struct Day06;

impl Solution for Day06 {
    fn name(&self) -> &'static str {
        "Lanternfish"
    }

    fn part_a(&self) -> String {
        let data = Fish::parse_inp(problem::load(2021, 6));
        let out = Fish::sim(data, 80);

        out.to_string()
    }

    fn part_b(&self) -> String {
        let data = Fish::parse_inp(problem::load(2021, 6));
        let out = Fish::sim(data, 256);

        out.to_string()
    }
}

#[derive(Debug, Clone, Copy, Hash, Eq, PartialEq)]
struct Fish {
    timer: u32,
}

impl Fish {
    fn new(timer: u32) -> Fish {
        Fish { timer }
    }

    fn parse_inp(inp: String) -> Vec<Fish> {
        inp.lines()
            .next()
            .unwrap()
            .split(',')
            .map(|x| Fish::new(x.parse().unwrap()))
            .collect()
    }

    fn sim(fish_vec: Vec<Fish>, days: u32) -> usize {
        let mut fish = HashMap::new();

        for i in fish_vec {
            *fish.entry(i).or_insert(0) += 1;
        }

        for _ in 0..days {
            let mut new_fish: HashMap<Fish, usize> = HashMap::new();
            for i in &fish {
                if i.0.timer == 0 {
                    *new_fish.entry(Fish::new(6)).or_insert(0) += *i.1;
                    *new_fish.entry(Fish::new(8)).or_insert(0) += *i.1;
                    continue;
                }

                *new_fish.entry(Fish::new(i.0.timer - 1)).or_insert(0) += *i.1;
            }
            fish = new_fish;
        }

        fish.values().sum()
    }
}
