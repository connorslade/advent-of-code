use std::collections::HashSet;

use crate::{problem, Solution};

pub struct Day03;

impl Solution for Day03 {
    fn name(&self) -> &'static str {
        "Rucksack Reorganization"
    }

    fn part_a(&self) -> String {
        let raw = problem::load(2022, 3);
        let mut out = 0;

        for i in raw.trim().lines() {
            let mut bolth = i[0..i.len() / 2].chars().collect::<Vec<_>>();
            let pocket_2 = i[i.len() / 2..].chars().collect::<Vec<_>>();
            bolth.retain(|x| pocket_2.contains(x));
            bolth.dedup();

            out += score_item(bolth[0]) as usize;
        }

        out.to_string()
    }

    fn part_b(&self) -> String {
        let raw = problem::load(2022, 3);
        let mut out = 0;

        for i in raw.trim().lines().collect::<Vec<_>>().chunks(3) {
            let mut all = HashSet::new();

            for j in i {
                all.extend(j.chars());
            }

            for j in i {
                all.retain(|x| j.contains(*x));
            }

            debug_assert!(all.len() == 1);
            out += score_item(*all.iter().next().unwrap()) as usize;
        }

        out.to_string()
    }
}

fn score_item(char_: char) -> u8 {
    match char_ as u8 {
        97..=122 => char_ as u8 - 96,
        65..=90 => char_ as u8 - 38,
        _ => panic!(),
    }
}
