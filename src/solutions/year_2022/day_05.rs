use crate::{problem, Solution};

pub struct Day05;

impl Solution for Day05 {
    fn name(&self) -> &'static str {
        "Supply Stacks"
    }

    fn part_a(&self) -> String {
        let raw = problem::load(2022, 5).replace("\r", "");
        process(&raw, true)
    }

    fn part_b(&self) -> String {
        let raw = problem::load(2022, 5).replace("\r", "");
        process(&raw, false)
    }
}

// TODO: Clean this up
fn process(raw: &str, part: bool) -> String {
    let (cretes, orders) = raw.split_once("\n\n").unwrap();
    let mut crates = parse_crates(cretes);

    for i in orders.trim().lines() {
        let parts = i.split_whitespace().collect::<Vec<_>>();
        let count = parts[1].parse::<usize>().unwrap();
        let from = parts[3].parse::<usize>().unwrap() - 1;
        let to = parts[5].parse::<usize>().unwrap() - 1;

        if part {
            for _ in 0..count {
                let old = crates[from].remove(0);
                crates[to].insert(0, old);
            }
            continue;
        }

        let mut working = Vec::new();
        for _ in 0..count {
            working.push(crates[from].remove(0));
        }

        for i in working.iter().rev() {
            crates[to].insert(0, *i);
        }
    }

    let mut out = String::new();
    for i in crates.iter().filter(|x| !x.is_empty()) {
        out.push(i[0]);
    }

    out
}

fn parse_crates(raw: &str) -> Vec<Vec<char>> {
    let mut out = vec![Vec::new(); 9];

    for i in raw.lines().filter(|x| !x.starts_with(" 1")) {
        for j in 0..9 {
            if let Some(x) = i.chars().nth(1 + j * 4) {
                if x.is_whitespace() {
                    continue;
                }

                out[j].push(x);
            }
        }
    }

    out
}
