use crate::{problem, Solution};

pub struct Day08;

impl Solution for Day08 {
    fn name(&self) -> &'static str {
        "Treetop Tree House"
    }

    fn part_a(&self) -> String {
        let raw = problem::load(2022, 8);
        let trees = parse_trees(&raw);
        let mut count = 0;

        for row in 0..trees.len() {
            for col in 0..trees[row].len() {
                let height = trees[row][col];
                if trees[..row].iter().all(|x| x[col] < height)
                    || trees[row][..col].iter().all(|x| x < &height)
                    || trees[row + 1..].iter().all(|x| x[col] < height)
                    || trees[row][col + 1..].iter().all(|x| x < &height)
                {
                    count += 1;
                }
            }
        }

        count.to_string()
    }

    fn part_b(&self) -> String {
        let raw = problem::load(2022, 8);
        let trees = parse_trees(&raw);
        let mut count = 0;

        for row in 0..trees.len() {
            for col in 0..trees[row].len() {
                let h = trees[row][col];
                let mut b = 1;
                // h -> current height
                // b -> best score

                process_slice(&mut b, h, trees[..row].iter().map(|x| x[col]).rev());
                process_slice(&mut b, h, trees[row][..col].iter().rev().copied());
                process_slice(&mut b, h, trees[row + 1..].iter().map(|x| x[col]));
                process_slice(&mut b, h, trees[row][col + 1..].iter().copied());
                count = count.max(b);
            }
        }

        count.to_string()
    }
}

fn parse_trees(inp: &str) -> Vec<Vec<usize>> {
    let mut out = Vec::new();

    for i in inp.lines() {
        let mut tree = Vec::new();
        for j in i.split("").filter(|x| !x.is_empty()) {
            tree.push(j.parse().unwrap());
        }
        out.push(tree);
    }

    out
}

fn process_slice(local_best: &mut usize, height: usize, iter: impl Iterator<Item = usize>) {
    let mut score = 0;
    for i in iter {
        score += 1;
        if i >= height {
            break;
        }
    }
    *local_best *= score;
}
