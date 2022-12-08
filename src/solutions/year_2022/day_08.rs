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
                let height = trees[row][col];
                let mut local_best = 1;

                for arr in [
                    trees[..row]
                        .iter()
                        .map(|x| x[col])
                        .rev()
                        .collect::<Vec<_>>(),
                    trees[row][..col].iter().rev().copied().collect::<Vec<_>>(),
                    trees[row + 1..].iter().map(|x| x[col]).collect::<Vec<_>>(),
                    trees[row][col + 1..].iter().copied().collect::<Vec<_>>(),
                ] {
                    let mut dir_score = 0;
                    for k in arr {
                        dir_score += 1;
                        if k >= height {
                            break;
                        }
                    }
                    local_best *= dir_score;
                }
                count = count.max(local_best);
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
