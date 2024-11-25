use std::{fmt::Display, ops::RangeInclusive};

use common::{Answer, ISolution};
use itertools::Itertools;
use nd_vec::{vector, Vec2, Vec3};
use num_traits::Num;

pub struct Day24;

impl ISolution for Day24 {
    fn name(&self) -> &'static str {
        "Never Tell Me The Odds"
    }

    fn part_a(&self, input: &str) -> Answer {
        let stones = parse(input);
        solve_a(&stones, 200000000000000.0..=400000000000000.0).into()
    }

    fn part_b(&self, input: &str) -> Answer {
        let stones = parse(input);

        const BRUTE_RANGE: RangeInclusive<i64> = -1000..=1000;

        let mut possible_x_vel = Vec::new();
        let mut possible_y_vel = Vec::new();
        let mut possible_z_vel = Vec::new();

        let mut iter = stones.iter().tuple_combinations();
        while possible_x_vel.len() != 1 || possible_y_vel.len() != 1 || possible_z_vel.len() != 1 {
            let (a, b) = iter.next().expect("No solution found");
            let process = |possible: &mut Vec<i64>, idx: usize| {
                let pos = (a.pos.as_slice()[idx], b.pos.as_slice()[idx]);
                let vel = (a.vel.as_slice()[idx], b.vel.as_slice()[idx]);

                if vel.0 != vel.1 {
                    return;
                }

                let delta = (pos.0 - pos.1).abs();
                let this = BRUTE_RANGE
                    .clone()
                    .filter(|i| i != &vel.0 && delta % (i - vel.0) == 0)
                    .collect_vec();

                possible.retain(|v| this.contains(v));
                if possible.is_empty() {
                    possible.extend(this);
                }
            };

            process(&mut possible_x_vel, 0);
            process(&mut possible_y_vel, 1);
            process(&mut possible_z_vel, 2);
        }

        let (a, b) = (stones[0].as_float(), stones[1].as_float());
        let (xv, yv, zv) = (
            possible_x_vel[0] as f64,
            possible_y_vel[0] as f64,
            possible_z_vel[0] as f64,
        );

        let ma = (a.vel.y() - yv) / (a.vel.x() - xv);
        let mb = (b.vel.y() - yv) / (b.vel.x() - xv);

        let ca = a.pos.y() - ma * a.pos.x();
        let cb = b.pos.y() - mb * b.pos.x();

        let x = (cb - ca) / (ma - mb);
        let y = ma * x + ca;
        let t = (x - a.pos.x()) / (a.vel.x() - xv);
        let z = a.pos.z() + (a.vel.z() - zv) * t;

        ((x + y + z) as i64).into()
    }
}

fn solve_a(stones: &[HailStone<i64>], range: RangeInclusive<f64>) -> usize {
    stones
        .iter()
        .tuple_combinations()
        .filter_map(|(a, b)| a.collision(b))
        .filter(|&pos| range.contains(&pos.x()) && range.contains(&pos.y()))
        .count()
}

#[derive(Debug, Clone, Copy)]
struct HailStone<T: Num + Copy + Display> {
    pos: Vec3<T>,
    vel: Vec3<T>,
}

impl HailStone<i64> {
    fn as_float(&self) -> HailStone<f64> {
        HailStone {
            pos: self.pos.num_cast().unwrap(),
            vel: self.vel.num_cast().unwrap(),
        }
    }

    fn collision(&self, other: &Self) -> Option<Vec2<f64>> {
        let (a, b) = (self.as_float(), other.as_float());

        let a_slope = a.vel.y() / a.vel.x();
        let a_intercept = a.pos.y() - a_slope * a.pos.x();

        let b_slope = b.vel.y() / b.vel.x();
        let b_intercept = b.pos.y() - b_slope * b.pos.x();

        let x_pos = (b_intercept - a_intercept) / (a_slope - b_slope);
        let y_pos = a_slope * x_pos + a_intercept;

        (x_pos.is_normal()
            && y_pos.is_normal()
            && !(a.vel.x() > 0.0 && x_pos < a.pos.x())
            && !(a.vel.x() < 0.0 && x_pos > a.pos.x())
            && !(b.vel.x() > 0.0 && x_pos < b.pos.x())
            && !(b.vel.x() < 0.0 && x_pos > b.pos.x()))
        .then(|| vector!(x_pos, y_pos))
    }
}

fn parse(input: &str) -> Vec<HailStone<i64>> {
    let mut out = Vec::new();

    for line in input.lines() {
        let parts = line.split_whitespace().collect::<Vec<_>>();
        let parse = |i| parts[i as usize].trim_end_matches(',').parse().unwrap();
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
