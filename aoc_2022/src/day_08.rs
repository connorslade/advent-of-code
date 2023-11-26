use common::{Answer, Solution};

pub struct Day08;

impl Solution for Day08 {
    fn name(&self) -> &'static str {
        "Treetop Tree House"
    }

    fn part_a(&self, input: &str) -> Answer {
        let trees = parse_trees(input);
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

        count.into()
    }

    fn part_b(&self, input: &str) -> Answer {
        let trees = parse_trees(input);
        let mut count = 0;

        for row in 0..trees.len() {
            for col in 0..trees[row].len() {
                let mut ctx = (1, trees[row][col]);
                process_slice(&mut ctx, trees[..row].iter().map(|x| x[col]).rev());
                process_slice(&mut ctx, trees[row][..col].iter().rev().copied());
                process_slice(&mut ctx, trees[row + 1..].iter().map(|x| x[col]));
                process_slice(&mut ctx, trees[row][col + 1..].iter().copied());
                count = count.max(ctx.0);
            }
        }

        count.into()
    }
}

fn parse_trees(inp: &str) -> Vec<Vec<usize>> {
    inp.lines()
        .map(|i| {
            i.chars()
                .filter(|x| x.is_ascii_digit())
                .filter_map(|x| x.to_digit(10))
                .map(|x| x as usize)
                .collect()
        })
        .collect()
}

fn process_slice((local_best, height): &mut (usize, usize), iter: impl Iterator<Item = usize>) {
    let mut score = 0;

    for i in iter {
        score += 1;
        if i >= *height {
            break;
        }
    }

    *local_best *= score;
}
