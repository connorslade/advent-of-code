use common::{Answer, Solution};
use nd_vec::vector;

use crate::aoc_lib::direction::Direction;

pub struct Day18;

impl Solution for Day18 {
    fn name(&self) -> &'static str {
        "Lavaduct Lagoon"
    }

    fn part_a(&self, input: &str) -> Answer {
        solve(&parse_a(input)).into()
    }

    fn part_b(&self, input: &str) -> Answer {
        solve(&parse_b(input)).into()
    }
}

fn solve(instructions: &[(Direction, u32)]) -> i64 {
    let mut pos = vector!(0, 0);
    let mut perimeter = 0;
    let mut area = 0;

    for (dir, steps) in instructions.iter().copied() {
        let dir = dir.as_vector();
        let cng = dir * (steps as i64);
        pos += cng;

        perimeter += steps as i64;
        area += pos.x() * cng.y();
    }

    area + perimeter / 2 + 1
}

fn parse_b(input: &str) -> Vec<(Direction, u32)> {
    input
        .lines()
        .map(|line| {
            let hex = &line[line.find('#').unwrap() + 1..line.len() - 1];
            let steps = u32::from_str_radix(&hex[0..5], 16).unwrap();
            let dir = match &hex[5..6] {
                "0" => Direction::Right,
                "1" => Direction::Down,
                "2" => Direction::Left,
                "3" => Direction::Up,
                _ => panic!("Invalid direction"),
            };
            (dir, steps)
        })
        .collect()
}

fn parse_a(input: &str) -> Vec<(Direction, u32)> {
    input
        .lines()
        .map(|line| {
            let mut parts = line.split_whitespace();
            let dir = match parts.next().unwrap() {
                "R" => Direction::Right,
                "L" => Direction::Left,
                "U" => Direction::Up,
                "D" => Direction::Down,
                _ => panic!("Invalid direction"),
            };
            let steps = parts.next().unwrap();
            (dir, steps.parse().unwrap())
        })
        .collect()
}

#[cfg(test)]
mod test {
    use common::Solution;
    use indoc::indoc;

    use super::Day18;

    const CASE: &str = indoc! {"
        R 6 (#70c710)
        D 5 (#0dc571)
        L 2 (#5713f0)
        D 2 (#d2c081)
        R 2 (#59c680)
        D 2 (#411b91)
        L 5 (#8ceee2)
        U 2 (#caa173)
        L 1 (#1b58a2)
        U 2 (#caa171)
        R 2 (#7807d2)
        U 3 (#a77fa3)
        L 2 (#015232)
        U 2 (#7a21e3)
    "};

    #[test]
    fn part_a() {
        assert_eq!(Day18.part_a(CASE), 62.into());
    }

    #[test]
    fn part_b() {
        assert_eq!(Day18.part_b(CASE), 952408144115i64.into());
    }
}
