use derive_more::{Add, AddAssign, Mul, Sub};
use std::{collections::HashSet, vec};

use crate::{problem, Solution};

pub struct Day09;

impl Solution for Day09 {
    fn name(&self) -> &'static str {
        "Rope Bridge"
    }

    fn part_a(&self) -> String {
        let raw = problem::load(2022, 9);
        process(&raw, 1).to_string()
    }

    fn part_b(&self) -> String {
        let raw = problem::load(2022, 9);
        process(&raw, 9).to_string()
    }
}

fn process(raw: &str, count: usize) -> usize {
    let orders = raw.lines().map(Point::from_line).collect::<Vec<_>>();
    let mut tail_pios = HashSet::new();
    let mut knots = vec![Point::new(0, 0); count + 1];

    tail_pios.insert(*knots.last().unwrap());
    for (dir, count) in orders {
        for _ in 0..count {
            knots[0] += dir;

            for i in 1..knots.len() {
                let diff = knots[i - 1] - knots[i];
                if diff.abs().max() <= 1 {
                    continue;
                }

                knots[i] += diff.normalize();
            }
            tail_pios.insert(*knots.last().unwrap());
        }
    }

    tail_pios.len()
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Add, AddAssign, Mul, Sub)]
struct Point {
    x: i32,
    y: i32,
}

impl Point {
    fn new(x: i32, y: i32) -> Self {
        Self { x, y }
    }

    fn normalize(&self) -> Self {
        Self {
            x: self.x.signum(),
            y: self.y.signum(),
        }
    }

    fn abs(&self) -> Self {
        Self {
            x: self.x.abs(),
            y: self.y.abs(),
        }
    }

    fn max(&self) -> i32 {
        self.x.max(self.y)
    }

    // Direction, count
    fn from_line(imp: &str) -> (Self, u32) {
        let (direction, count) = imp.split_once(' ').unwrap();
        let count = count.parse::<i32>().unwrap();

        let out = match direction {
            "R" => Self::new(1, 0),
            "L" => Self::new(-1, 0),
            "U" => Self::new(0, -1),
            "D" => Self::new(0, 1),
            _ => panic!("Invalid direction"),
        };

        (out, count as u32)
    }
}
