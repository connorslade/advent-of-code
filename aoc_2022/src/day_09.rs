use std::vec;

use hashbrown::HashSet;

use common::{solution, Answer};
use nd_vec::vector;

type Point = nd_vec::Vec2<i32>;

solution!("Rope Bridge", 9);

fn part_a(input: &str) -> Answer {
    process(input, 1).into()
}

fn part_b(input: &str) -> Answer {
    process(input, 9).into()
}

fn process(raw: &str, count: usize) -> usize {
    let orders = raw.lines().map(from_line).collect::<Vec<_>>();
    let mut tail_pios = HashSet::new();
    let mut knots = vec![vector!(0, 0); count + 1];

    tail_pios.insert(*knots.last().unwrap());
    for (dir, count) in orders {
        for _ in 0..count {
            knots[0] += dir;

            for i in 1..knots.len() {
                let diff = knots[i - 1] - knots[i];
                if diff.abs().max_component() <= 1 {
                    continue;
                }

                knots[i] += diff.signum();
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
        "R" => vector!(1, 0),
        "L" => vector!(-1, 0),
        "U" => vector!(0, -1),
        "D" => vector!(0, 1),
        _ => panic!("Invalid direction"),
    };

    (out, count as u32)
}
