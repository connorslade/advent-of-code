use std::ops::RangeInclusive;

use common::{Answer, Solution};
use itertools::Itertools;
use nd_vec::{vector, Vec2, Vec3};

type Pos = Vec3<f64>;

pub struct Day24;

impl Solution for Day24 {
    fn name(&self) -> &'static str {
        "Never Tell Me The Odds"
    }

    fn part_a(&self, input: &str) -> Answer {
        let stones = parse(input);
        solve_a(&stones, 200000000000000.0..=400000000000000.0).into()
    }

    fn part_b(&self, input: &str) -> Answer {
        let stones = parse(input);

        // Not a very satisfying solution, but it seems like everyone else is using z3.
        // just pipe it into mathematica :')
        // Solve[{ ... }]
        for (i, stone) in stones.into_iter().enumerate().take(3) {
            print!("{}+{}*t{i}==x+vx*t{i}, ", stone.pos.x(), stone.vel.x());
            print!("{}+{}*t{i}==y+vy*t{i}, ", stone.pos.y(), stone.vel.y());
            print!("{}+{}*t{i}==z+vz*t{i}", stone.pos.z(), stone.vel.z());

            if i < 2 {
                print!(", ");
            }
        }
        println!();

        Answer::Unimplemented
    }
}

fn solve_a(stones: &[HailStone], range: RangeInclusive<f64>) -> usize {
    stones
        .iter()
        .tuple_combinations()
        .filter_map(|(a, b)| a.collision(b))
        .filter(|&pos| range.contains(&pos.x()) && range.contains(&pos.y()))
        .count()
}

#[derive(Debug)]
struct HailStone {
    pos: Pos,
    vel: Pos,
}

impl HailStone {
    fn collision(&self, other: &Self) -> Option<Vec2<f64>> {
        let a_slope = self.vel.y() / self.vel.x();
        let a_intercept = self.pos.y() - a_slope * self.pos.x();

        let b_slope = other.vel.y() / other.vel.x();
        let b_intercept = other.pos.y() - b_slope * other.pos.x();

        let x_pos = (b_intercept - a_intercept) / (a_slope - b_slope);
        let y_pos = a_slope * x_pos + a_intercept;

        (x_pos.is_normal()
            && y_pos.is_normal()
            && !(self.vel.x() > 0.0 && x_pos < self.pos.x())
            && !(self.vel.x() < 0.0 && x_pos > self.pos.x())
            && !(other.vel.x() > 0.0 && x_pos < other.pos.x())
            && !(other.vel.x() < 0.0 && x_pos > other.pos.x()))
        .then(|| vector!(x_pos, y_pos))
    }
}

fn parse(input: &str) -> Vec<HailStone> {
    let mut out = Vec::new();

    for line in input.lines() {
        let parts = line.split_whitespace().collect::<Vec<_>>();
        let parse = |i| {
            parts[i as usize]
                .trim_end_matches(',')
                .parse::<f64>()
                .unwrap()
        };
        out.push(HailStone {
            pos: vector!(parse(0), parse(1), parse(2)),
            vel: vector!(parse(4), parse(5), parse(6)),
        });
    }

    out
}

#[cfg(test)]
mod test {
    use indoc::indoc;

    use super::{parse, solve_a};

    const CASE: &str = indoc! {"
        19, 13, 30 @ -2,  1, -2
        18, 19, 22 @ -1, -1, -2
        20, 25, 34 @ -2, -2, -4
        12, 31, 28 @ -1, -2, -1
        20, 19, 15 @  1, -5, -3
    "};

    #[test]
    fn part_a() {
        assert_eq!(solve_a(&parse(CASE), 7.0..=37.0), 2);
    }
}
