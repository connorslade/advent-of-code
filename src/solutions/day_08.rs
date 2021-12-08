use crate::common::{self, Solution};

use std::collections::HashMap;

const PATS: &[(&str, u32)] = &[
    ("acedgfb", 8),
    ("cdfbe", 5),
    ("gcdfa", 2),
    ("fbcad", 3),
    ("dab", 7),
    ("cefabd", 9),
    ("cdfgeb", 6),
    ("eafb", 4),
    ("cagedb", 0),
    ("ab", 1),
];

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
        let mut inc = 0;

        for i in data {
            let mut wires = HashMap::new();
            // let mut five = Vec::new();
            // let mut nums = HashMap::new();

            for j in i.0 {
                let js = j.to_string();
                match j.len() {
                    2 => *wires.entry(j).or_insert(0) = 1,
                    3 => *wires.entry(j).or_insert(0) = 7,
                    4 => *wires.entry(j).or_insert(0) = 4,
                    7 => *wires.entry(j).or_insert(0) = 8,
                    _ => {
                        for k in PATS {
                            if comp_wires(k.0, &j) {
                                *wires.entry(j.clone()).or_insert(k.1);
                            }
                        }
                    }
                }
            }

            let mut out = String::new();

            for j in &i.1 {
                for k in &wires {
                    println!("{:?}", k);
                    if comp_wires(&j, &k.0) {
                        out.push_str(k.1.to_string().as_str());
                        println!("{}", k.1);
                        continue;
                    }
                }
            }

            // println!("{:?}", wires);

            println!("{:?}", out);
            // if let Ok(by) = out.parse::<u32>() {
            //     inc += by;
            // }
            inc += out.parse::<u32>().unwrap();
        }

        inc.to_string()
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

fn comp_wires(one: &str, two: &str) -> bool {
    for i in one.chars() {
        if !two.contains(i) {
            return false;
        }
    }

    for i in two.chars() {
        if !one.contains(i) {
            return false;
        }
    }

    true
}
