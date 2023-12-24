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

        const RANGE: RangeInclusive<f64> = 200000000000000.0..=400000000000000.0;
        stones
            .iter()
            .tuple_combinations()
            .filter_map(|(a, b)| a.collision(b))
            .filter(|&pos| RANGE.contains(&pos.x()) && RANGE.contains(&pos.y()))
            .count()
            .into()
    }

    fn part_b(&self, input: &str) -> Answer {
        Answer::Unimplemented
    }
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
    use common::Solution;
    use indoc::indoc;

    use super::Day24;

    const CASE: &str = indoc! {"
        19, 13, 30 @ -2,  1, -2
        18, 19, 22 @ -1, -1, -2
        20, 25, 34 @ -2, -2, -4
        12, 31, 28 @ -1, -2, -1
        20, 19, 15 @  1, -5, -3
    "};

    #[test]
    fn part_a() {
        assert_eq!(Day24.part_a(CASE), 0.into());
    }

    #[test]
    fn part_b() {
        assert_eq!(Day24.part_b(CASE), ().into());
    }
}
