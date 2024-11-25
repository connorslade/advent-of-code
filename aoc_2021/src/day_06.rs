use common::{solution, Answer};

use std::hash::Hash;

use hashbrown::HashMap;

solution!("Lanternfish", (2022, 00));

fn part_a(input: &str) -> Answer {
    let data = Fish::parse_inp(input);
    Fish::sim(data, 80).into()
}

fn part_b(input: &str) -> Answer {
    let data = Fish::parse_inp(input);
    Fish::sim(data, 256).into()
}

#[derive(Debug, Clone, Copy, Hash, Eq, PartialEq)]
struct Fish {
    timer: u32,
}

impl Fish {
    fn new(timer: u32) -> Fish {
        Fish { timer }
    }

    fn parse_inp(inp: &str) -> Vec<Fish> {
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
