use std::{convert::identity, mem};

use aoc_lib::matrix::Grid;
use common::{Answer, solution};
use nd_vec::vector;

solution!("Trash Compactor", 6);

fn part_a(input: &str) -> Answer {
    let (nums, ops) = input.trim().rsplit_once('\n').unwrap();

    let mut problems = Vec::new();
    for line in nums.lines() {
        let nums = line
            .split_whitespace()
            .map(|x| x.parse::<u64>().unwrap())
            .collect::<Vec<_>>();
        if problems.is_empty() {
            problems.resize(nums.len(), Vec::new());
        }

        for (i, num) in nums.iter().enumerate() {
            problems[i].push(*num);
        }
    }

    let ops = ops.split_whitespace().collect::<Vec<_>>();

    let mut out = 0;
    for (nums, op) in problems.iter().zip(ops) {
        match op {
            "*" => out += nums.iter().product::<u64>(),
            "+" => out += nums.iter().sum::<u64>(),
            _ => panic!(),
        }
    }

    out.into()
}

fn part_b(input: &str) -> Answer {
    let grid = Grid::parse(input, identity);
    let size = grid.size();

    let mut pos = vector!(size.x() - 1, 0);

    let mut out = 0;
    let mut nums = Vec::new();
    'outer: loop {
        let mut num = 0;
        while pos.y() < size.y() {
            let chr = grid.get(pos).unwrap();

            if matches!(chr, '+' | '*') {
                nums.push(mem::take(&mut num));
                dbg!(&nums);
                match chr {
                    '+' => out += mem::take(&mut nums).iter().sum::<u64>(),
                    '*' => out += mem::take(&mut nums).iter().product::<u64>(),
                    _ => {}
                }

                if pos.x() == 0 {
                    break 'outer;
                }

                pos = vector!(pos.x() - 2, 0);
                continue 'outer;
            }

            if let Some(digit) = chr.to_digit(10) {
                num = num * 10 + digit as u64;
            }
            pos += vector!(0, 1);
        }
        nums.push(mem::take(&mut num));

        pos = vector!(pos.x() - 1, 0);
    }

    out.into()
}

#[cfg(test)]
mod test {
    const CASE: &str = "123 328  51 64 \n 45 64  387 23 \n  6 98  215 314\n*   +   *   +  ";

    #[test]
    fn part_a() {
        assert_eq!(super::part_a(CASE), 4277556.into());
    }

    #[test]
    fn part_b() {
        assert_eq!(super::part_b(CASE), 3263827.into());
    }
}
