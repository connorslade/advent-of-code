use crate::common::{self, Solution};

pub struct Day07 {}

impl Solution for Day07 {
    fn name(&self) -> String {
        "The Treachery of Whales".to_owned()
    }

    fn part_a(&self) -> String {
        let data = parse_crabs(common::load("07"));

        let min = data.iter().min().unwrap();
        let max = data.iter().max().unwrap();

        let mut this_min = u32::MAX;
        for i in *min..=*max {
            let cost = move_crabs(&data, i);
            if cost < this_min {
                this_min = cost;
            }
        }

        this_min.to_string()
    }

    fn part_b(&self) -> String {
        let data = parse_crabs(common::load("07"));

        let min = data.iter().min().unwrap();
        let max = data.iter().max().unwrap();

        let mut this_min = u32::MAX;
        for i in *min..=*max {
            let cost = move_crabs_b(&data, i);
            if cost < this_min {
                this_min = cost;
            }
        }

        this_min.to_string()
    }
}

fn parse_crabs(inp: String) -> Vec<u32> {
    inp.lines()
        .next()
        .unwrap()
        .split(',')
        .map(|x| x.parse().unwrap())
        .collect::<Vec<u32>>()
}

fn move_crabs(crabs: &[u32], to: u32) -> u32 {
    let mut cost = 0;
    for i in crabs {
        cost += (*i as i32 - to as i32).abs();
    }

    cost as u32
}

fn move_crabs_b(crabs: &[u32], to: u32) -> u32 {
    let mut cost = 0;
    for crab in crabs {
        cost += (0..=(*crab as i32 - to as i32).abs()).sum::<i32>();
    }

    cost as u32
}
