use bitvec::{bitvec, vec::BitVec};
use itertools::Itertools;
use nd_vec::{vector, Vec2};

use common::{Answer, Solution};

type Pos = Vec2<usize>;

pub struct Day11;

impl Solution for Day11 {
    fn name(&self) -> &'static str {
        "Cosmic Expansion"
    }

    fn part_a(&self, input: &str) -> Answer {
        let mut galaxies = parse(input);
        galaxies.expand(2);
        galaxies.total_distance().into()
    }

    fn part_b(&self, input: &str) -> Answer {
        let mut galaxies = parse(input);
        galaxies.expand(1000000);
        galaxies.total_distance().into()
    }
}

struct Galaxies {
    galaxies: Vec<Pos>,
    rows: BitVec,
    cols: BitVec,
}

fn parse(input: &str) -> Galaxies {
    let lines = input.lines().collect::<Vec<_>>();
    let mut galaxies = Vec::new();

    let mut rows = bitvec![0; lines[0].len()];
    let mut cols = bitvec![0; lines.len()];

    for (y, line) in lines.iter().enumerate() {
        for (x, c) in line.chars().enumerate() {
            if c == '#' {
                galaxies.push(vector!(x, y));
                rows.set(y, true);
                cols.set(x, true);
            }
        }
    }

    Galaxies {
        galaxies,
        rows,
        cols,
    }
}

impl Galaxies {
    fn expand(&mut self, mut multiplier: usize) {
        multiplier -= 1;

        for (y, _) in self.rows.iter().enumerate().rev().filter(|x| !x.1.as_ref()) {
            for pos in self.galaxies.iter_mut().filter(|pos| pos.y() > y) {
                *pos += vector!(0, multiplier)
            }
        }

        for (x, _) in self.cols.iter().enumerate().rev().filter(|x| !x.1.as_ref()) {
            for pos in self.galaxies.iter_mut().filter(|pos| pos.x() > x) {
                *pos += vector!(multiplier, 0)
            }
        }
    }

    fn total_distance(&self) -> usize {
        self.galaxies
            .iter()
            .map(|x| x.num_cast::<isize>().unwrap())
            .tuple_combinations()
            .map(|(a, b)| a.manhattan_distance(&b) as usize)
            .sum()
    }
}

#[cfg(test)]
mod test {
    use common::Solution;
    use indoc::indoc;

    use super::Day11;

    const CASE: &str = indoc! {"
        ...#......
        .......#..
        #.........
        ..........
        ......#...
        .#........
        .........#
        ..........
        .......#..
        #...#.....
    "};

    #[test]
    fn part_a() {
        assert_eq!(Day11.part_a(CASE), 374.into());
    }

    #[test]
    fn part_b() {
        assert_eq!(Day11.part_b(CASE), 82000210.into());
    }
}
