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
    rows: Vec<bool>,
    cols: Vec<bool>,
}

fn parse(input: &str) -> Galaxies {
    let mut galaxies = Vec::new();
    let (height, width) = (input.lines().count(), input.lines().next().unwrap().len());

    let mut rows = vec![false; height];
    let mut cols = vec![false; width];

    for (y, line) in input.lines().enumerate() {
        for (x, c) in line.chars().enumerate() {
            if c == '#' {
                galaxies.push(vector!(x, y));
                rows[y] = true;
                cols[x] = true;
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

        for (y, _) in self.rows.iter().enumerate().rev().filter(|(_, &row)| !row) {
            for pos in &mut self.galaxies {
                if pos.y() > y {
                    *pos += vector!(0, multiplier)
                }
            }
        }

        for (x, _) in self.cols.iter().enumerate().rev().filter(|(_, &col)| !col) {
            for pos in &mut self.galaxies {
                if pos.x() > x {
                    *pos += vector!(multiplier, 0)
                }
            }
        }
    }

    fn total_distance(&self) -> usize {
        self.galaxies
            .iter()
            .combinations(2)
            .map(|pair| {
                let (a, b) = (pair[0], pair[1]);
                let dist = (a.x() as isize - b.x() as isize).abs()
                    + (a.y() as isize - b.y() as isize).abs();

                dist as usize
            })
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
