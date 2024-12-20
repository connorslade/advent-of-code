use std::{collections::VecDeque, convert::identity, u32};

use aoc_lib::{direction::cardinal::Direction, matrix::Grid};
use common::{solution, Answer};
use itertools::Itertools;
use nd_vec::{vector, Vec2};

solution!("Race Condition", 20);

fn part_a(input: &str) -> Answer {
    Problem::parse(input).solve(2).into()
}

fn part_b(input: &str) -> Answer {
    Problem::parse(input).solve(20).into()
}

struct Problem {
    board: Grid<char>,
    start: Vec2<usize>,
    end: Vec2<usize>,
}

impl Problem {
    fn parse(input: &str) -> Self {
        let board = Grid::parse(input, identity);

        let start = board.find('S').unwrap();
        let end = board.find('E').unwrap();

        Self { board, start, end }
    }

    fn solve(&self, max_skip: i32) -> u32 {
        let (sc, ec) = (self.cost_map(self.start), self.cost_map(self.end));
        let base_cost = sc[self.end];

        let mut out = 0;

        for (pos, tile) in self.board.iter() {
            if *tile == '#' || sc[pos] == u32::MAX {
                continue;
            }

            for (x, y) in (-max_skip..=max_skip).cartesian_product(-max_skip..=max_skip) {
                let offset = vector!(x, y);
                let dist = offset.manhattan_distance(&Vec2::zero());
                if dist > max_skip {
                    continue;
                }

                let end = pos.try_cast::<i32>().unwrap() + offset;
                if !self.board.contains(end) || self.board[end] == '#' || ec[end] == u32::MAX {
                    continue;
                }

                let cost = sc[pos] + ec[end] + dist as u32;
                out += (cost + 100 <= base_cost) as u32;
            }
        }

        out
    }

    fn cost_map(&self, start: Vec2<usize>) -> Grid<u32> {
        let mut costs = Grid::new(self.board.size, u32::MAX);
        let mut queue = VecDeque::new();
        queue.push_back((start, 0));

        while let Some((pos, dist)) = queue.pop_front() {
            if costs[pos] != u32::MAX {
                continue;
            }

            costs[pos] = dist;
            for dir in Direction::ALL {
                let next = dir.wrapping_advance(pos);
                if let Some(tile) = self.board.get(next) {
                    if matches!(tile, '.' | 'E') {
                        queue.push_back((next, dist + 1));
                    }
                }
            }
        }

        costs
    }
}

#[cfg(test)]
mod test {
    use indoc::indoc;

    const CASE: &str = indoc! {"
        ###############
        #...#...#.....#
        #.#.#.#.#.###.#
        #S#...#.#.#...#
        #######.#.#.###
        #######.#.#...#
        #######.#.###.#
        ###..E#...#...#
        ###.#######.###
        #...###...#...#
        #.#####.#.###.#
        #.#...#.#.#...#
        #.#.#.#.#.#.###
        #...#...#...###
        ###############
    "};

    #[test]
    fn part_a() {
        assert_eq!(super::part_a(CASE), 44.into());
    }

    #[test]
    fn part_b() {
        assert_eq!(super::part_b(CASE), ().into());
    }
}
