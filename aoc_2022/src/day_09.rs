use std::vec;

use hashbrown::HashSet;

use crate::aoc_lib;
use common::Solution;

type Point = aoc_lib::Point<i32>;

pub struct Day09;

impl Solution for Day09 {
    fn name(&self) -> &'static str {
        "Rope Bridge"
    }

    fn part_a(&self, input: &str) -> String {
        process(input, 1).to_string()
    }

    fn part_b(&self, input: &str) -> String {
        process(input, 9).to_string()
    }
}

fn process(raw: &str, count: usize) -> usize {
    let orders = raw.lines().map(from_line).collect::<Vec<_>>();
    let mut tail_pios = HashSet::new();
    let mut knots = vec![Point::new(0, 0); count + 1];

    tail_pios.insert(*knots.last().unwrap());
    for (dir, count) in orders {
        for _ in 0..count {
            knots[0] += dir;

            for i in 1..knots.len() {
                let diff = knots[i - 1] - knots[i];
                if diff.abs().max_value() <= 1 {
                    continue;
                }

                knots[i] += diff.normalize();
            }
            tail_pios.insert(*knots.last().unwrap());
        }
    }

    tail_pios.len()
}

// Direction, count
fn from_line(imp: &str) -> (Point, u32) {
    let (direction, count) = imp.split_once(' ').unwrap();
    let count = count.parse::<i32>().unwrap();

    let out = match direction {
        "R" => Point::new(1, 0),
        "L" => Point::new(-1, 0),
        "U" => Point::new(0, -1),
        "D" => Point::new(0, 1),
        _ => panic!("Invalid direction"),
    };

    (out, count as u32)
}
