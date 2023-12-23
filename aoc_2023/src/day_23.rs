use std::{collections::HashSet, convert::identity};

use common::{Answer, Solution};
use nd_vec::{vector, Vec2};

use crate::aoc_lib::{direction::Direction, matrix::Matrix};

type Pos = Vec2<usize>;

pub struct Day23;

impl Solution for Day23 {
    fn name(&self) -> &'static str {
        "A Long Walk"
    }

    fn part_a(&self, input: &str) -> Answer {
        longest_path(
            &parse(input),
            Box::new(HashSet::new()),
            true,
            vector!(1, 0),
            0,
        )
        .into()
    }

    fn part_b(&self, input: &str) -> Answer {
        longest_path(
            &parse(input),
            Box::new(HashSet::new()),
            false,
            vector!(1, 0),
            0,
        )
        .into()
    }
}

fn longest_path(
    map: &Matrix<char>,
    visited: Box<HashSet<Pos>>,
    respect_slopes: bool,
    pos: Pos,
    idx: u32,
) -> u32 {
    if pos == map.size() - vector!(2, 1) {
        return idx;
    }

    let mut longest = 0;
    for dir in Direction::ALL.iter() {
        let Some(new_pos) = dir.try_advance(pos) else {
            continue;
        };

        if map[new_pos] == '#' {
            continue;
        }

        let next = map[new_pos];
        if !(if respect_slopes {
            match dir {
                Direction::Up => next == '^',
                Direction::Down => next == 'v',
                Direction::Left => next == '<',
                Direction::Right => next == '>',
            }
        } else {
            next == '^' || next == 'v' || next == '<' || next == '>'
        } || next == '.')
        {
            continue;
        }

        let mut visited = visited.clone();
        if !visited.insert(pos) {
            continue;
        }

        longest = longest.max(longest_path(map, visited, respect_slopes, new_pos, idx + 1));
    }

    longest
}

fn parse(input: &str) -> Matrix<char> {
    Matrix::new_chars(input, identity)
}

#[cfg(test)]
mod test {
    use common::Solution;
    use indoc::indoc;

    use super::Day23;

    const CASE: &str = indoc! {"
        #.#####################
        #.......#########...###
        #######.#########.#.###
        ###.....#.>.>.###.#.###
        ###v#####.#v#.###.#.###
        ###.>...#.#.#.....#...#
        ###v###.#.#.#########.#
        ###...#.#.#.......#...#
        #####.#.#.#######.#.###
        #.....#.#.#.......#...#
        #.#####.#.#.#########v#
        #.#...#...#...###...>.#
        #.#.#v#######v###.###v#
        #...#.>.#...>.>.#.###.#
        #####v#.#.###v#.#.###.#
        #.....#...#...#.#.#...#
        #.#########.###.#.#.###
        #...###...#...#...#.###
        ###.###.#.###v#####v###
        #...#...#.#.>.>.#.>.###
        #.###.###.#.###.#.#v###
        #.....###...###...#...#
        #####################.#
    "};

    #[test]
    fn part_a() {
        assert_eq!(Day23.part_a(CASE), 94.into());
    }

    #[test]
    fn part_b() {
        assert_eq!(Day23.part_b(CASE), 154.into());
    }
}
