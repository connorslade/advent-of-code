use crate::{problem, Solution};

use std::mem;

pub struct Day19;

impl Solution for Day19 {
    fn name(&self) -> &'static str {
        ""
    }

    fn part_a(&self) -> String {
        let raw = problem::load(2022, 19);
        let robots = parse(&raw);

        let mut geodes = Vec::new();
        for i in robots.into_iter().take(1) {
            geodes.push(simulate(i));
        }

        dbg!(&geodes);
        geodes
            .iter()
            .enumerate()
            .map(|(i, e)| e * (1 + i as u32))
            .sum::<u32>()
            .to_string()
    }

    fn part_b(&self) -> String {
        let _raw = problem::load(2022, 19);
        todo!()
    }
}

fn simulate(costs: [RobotType; 4]) -> u32 {
    // [ore, clay, obsidian, geode]
    let mut resources = [0, 0, 0, 0];
    let mut robots = [1, 0, 0, 0];
    let mut ticks = 24;

    while ticks > 0 {
        let mut new_robots = robots.clone();
        while let Some(i) = RobotType::best_buildable(&costs, &resources) {
            println!("Shound build robot {:?}", i);
            i.build(&mut resources);
            println!(" ^ new resource count: {:?}", resources);
            new_robots[i.index()] += 1;
        }

        for r_type in 0..4 {
            for _ in 0..robots[r_type] {
                resources[r_type] += 1;
            }
        }

        println!(
            "mins: {} | ores: {:?} | robots: {:?}",
            24 - ticks,
            resources,
            robots
        );
        ticks -= 1;
        mem::swap(&mut robots, &mut new_robots);
    }

    resources[3]
}

#[derive(Debug, Clone, Copy)]
enum RobotType {
    // (ore cost)
    Ore(u8),
    // (ore cost)
    Clay(u8),
    // (ore cost, clay cost)
    Obsidian(u8, u8),
    // (ore cost, obsidian cost)
    Geode(u8, u8),
}

fn parse(raw: &str) -> Vec<[RobotType; 4]> {
    raw.lines().map(RobotType::parse).collect()
}

impl RobotType {
    fn parse(raw: &str) -> [Self; 4] {
        let mut out = [Self::Ore(0); 4];

        fn first_word(s: &str) -> &str {
            s.split_whitespace().next().unwrap()
        }

        for (i, e) in raw
            .split(['.', ':'])
            .skip(1)
            .filter(|x| !x.is_empty())
            .enumerate()
        {
            let cost = e.split_once("costs ").unwrap();
            match i {
                0 => out[0] = Self::Ore(first_word(cost.1).parse().unwrap()),
                1 => out[1] = Self::Clay(first_word(cost.1).parse().unwrap()),
                2 => {
                    let cost = cost.1.split_once(" and ").unwrap();
                    out[2] = Self::Obsidian(
                        first_word(cost.0).parse().unwrap(),
                        first_word(cost.1).parse().unwrap(),
                    );
                }
                3 => {
                    let cost = cost.1.split_once(" and ").unwrap();
                    out[3] = Self::Geode(
                        first_word(cost.0).parse().unwrap(),
                        first_word(cost.1).parse().unwrap(),
                    );
                }
                _ => unreachable!(),
            }
        }

        out
    }

    fn index(&self) -> usize {
        match self {
            Self::Ore(_) => 0,
            Self::Clay(_) => 1,
            Self::Obsidian(_, _) => 2,
            Self::Geode(_, _) => 3,
        }
    }

    fn best_buildable(costs: &[Self; 4], resources: &[u32; 4]) -> Option<RobotType> {
        let mut out = None;

        for e in costs.iter() {
            let can_build = match e {
                Self::Ore(c) => resources[0] >= *c as u32,
                Self::Clay(c) => resources[0] >= *c as u32,
                Self::Obsidian(o, c) => resources[0] >= *o as u32 && resources[1] >= *c as u32,
                Self::Geode(o, c) => resources[0] >= *o as u32 && resources[2] >= *c as u32,
            };

            if can_build {
                out = Some(e);
            }
        }

        out.copied()
    }

    fn build(&self, resources: &mut [u32; 4]) {
        match self {
            Self::Ore(c) => resources[0] -= *c as u32,
            Self::Clay(c) => resources[0] -= *c as u32,
            Self::Obsidian(o, c) => {
                resources[0] -= *o as u32;
                resources[1] -= *c as u32;
            }
            Self::Geode(o, c) => {
                resources[0] -= *o as u32;
                resources[2] -= *c as u32;
            }
        }
    }
}
