use crate::common::{self, Solution};

use std::collections::HashMap;

pub struct Day08 {}

impl Solution for Day08 {
    fn name(&self) -> String {
        "Seven Segment Search".to_owned()
    }

    fn part_a(&self) -> String {
        let data = parse(common::load("08"));
        let mut inc = 0;

        for i in data {
            inc +=
                i.1.iter()
                    .filter(|x| [2, 3, 4, 7].contains(&x.len()))
                    .count();
        }

        inc.to_string()
    }

    fn part_b(&self) -> String {
        let data = parse(common::load("08"));
        // let mut inc = 0;

        for i in data {
            let mut wires = HashMap::new();
            let mut nums = HashMap::new();

            // for j in i.0 {
            //     match j.len() {
            //         2 => {
            //             *wires.entry(j).or_insert(0) = 1;
            //             *nums.entry(1).or_insert("") = j.clone().as_str();
            //         }
            //     }
            // }
            // ...
            // Work in progress
            // sorry i was tired
            // its 12:40
        }

        "".to_string()
    }
}

fn parse(inp: String) -> Vec<(Vec<String>, Vec<String>)> {
    let mut out = Vec::new();

    for i in inp.lines() {
        let mut parts = i.split('|');

        let test = parts
            .next()
            .unwrap()
            .split(' ')
            .filter(|x| !x.is_empty())
            .map(|x| x.to_owned())
            .collect::<Vec<String>>();

        let check = parts
            .next()
            .unwrap()
            .split(' ')
            .filter(|x| !x.is_empty())
            .map(|x| x.to_owned())
            .collect::<Vec<String>>();

        out.push((test, check));
    }

    out
}
