use crate::common::{self, Solution};

use std::collections::HashMap;
use std::hash::Hash;

pub struct Day06 {}

impl Solution for Day06 {
    fn name(&self) -> String {
        "Lanternfish".to_owned()
    }

    fn part_a(&self) -> String {
        let data = Fish::parse_inp(common::load("06"));
        let data = Fish::sim(data, 80);

        data.to_string()
    }

    fn part_b(&self) -> String {
        let data = Fish::parse_inp(common::load("06"));
        let data = Fish::sim(data, 256);

        data.to_string()
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
            .map(|x| Fish {
                timer: x.parse().unwrap(),
            })
            .collect()
    }

    fn sim(fish_vec: Vec<Fish>, days: u32) -> usize {
        let mut fish = HashMap::new();

        for i in fish_vec {
            inc(&mut fish, i, 1);
        }

        for _ in 0..days {
            let mut new_fish: HashMap<Fish, usize> = HashMap::new();
            for i in &fish {
                match i.0.timer {
                    0 => {
                        inc(&mut new_fish, Fish::new(6), *i.1);
                        inc(&mut new_fish, Fish::new(8), *i.1);
                    }
                    _ => inc(&mut new_fish, Fish::new(i.0.timer - 1), *i.1),
                }
            }
            fish = new_fish;
        }

        fish.values().sum()
    }
}

fn inc<T>(map: &mut HashMap<T, usize>, key: T, inc: usize)
where
    T: Eq + Hash,
{
    if map.contains_key(&key) {
        *map.get_mut(&key).unwrap() += inc;
        return;
    }
    map.insert(key, inc);
}
