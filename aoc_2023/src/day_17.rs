use std::collections::{HashMap, VecDeque};

use common::{Answer, Solution};
use nd_vec::{vector, Vec2};

use crate::aoc_lib::direction::Direction;

type Pos = Vec2<usize>;

pub struct Day17;

impl Solution for Day17 {
    fn name(&self) -> &'static str {
        "Clumsy Crucible"
    }

    fn part_a(&self, input: &str) -> Answer {
        pathfind(parse(input), 1, 3).into()
    }

    fn part_b(&self, input: &str) -> Answer {
        pathfind(parse(input), 4, 10).into()
    }
}

fn pathfind(board: Vec<Vec<u8>>, min_dist: u8, max_dist: u8) -> u32 {
    let mut queue = VecDeque::new();
    let mut visited = HashMap::new();
    let mut res = u32::MAX;

    let end = vector!(board[0].len() - 1, board.len() - 1);
    for dir in [Direction::Down, Direction::Right] {
        let state = State::new(vector!(0, 0), dir, 1);
        queue.push_back((0, state));
        visited.insert(state, 0);
    }

    while let Some((cost, state)) = queue.pop_front() {
        let mut explore = |facing: Direction, turn_distance: u8| {
            if let Some(pos) = facing
                .try_advance(state.pos)
                .filter(|pos| pos.y() < board.len() && pos.x() < board[0].len())
            {
                let state = State::new(pos, facing, turn_distance);
                let cost = cost + board[pos.y()][pos.x()] as u32;

                if !visited.contains_key(&state) || visited.get(&state).unwrap() > &cost {
                    queue.push_back((cost, state));
                    visited.insert(state, cost);
                }
            }
        };

        if state.pos == end && state.turn_distance >= min_dist {
            res = res.min(cost);
            continue;
        }

        if state.turn_distance < max_dist {
            explore(state.facing, state.turn_distance + 1);
        }

        if state.turn_distance >= min_dist {
            explore(state.facing.turn_left(), 1);
            explore(state.facing.turn_right(), 1);
        }
    }

    res
}

fn parse(input: &str) -> Vec<Vec<u8>> {
    input
        .lines()
        .map(|line| line.chars().map(|x| x as u8 - b'0').collect())
        .collect::<Vec<_>>()
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
struct State {
    pos: Pos,
    facing: Direction,
    turn_distance: u8,
}

impl State {
    fn new(pos: Pos, facing: Direction, turn_distance: u8) -> Self {
        Self {
            pos,
            facing,
            turn_distance,
        }
    }
}

#[cfg(test)]
mod test {
    use common::Solution;
    use indoc::indoc;

    use super::Day17;

    const CASE: &str = indoc! {"
        2413432311323
        3215453535623
        3255245654254
        3446585845452
        4546657867536
        1438598798454
        4457876987766
        3637877979653
        4654967986887
        4564679986453
        1224686865563
        2546548887735
        4322674655533
    "};

    #[test]
    fn part_a() {
        assert_eq!(Day17.part_a(CASE), 102.into());
    }

    #[test]
    fn part_b() {
        assert_eq!(Day17.part_b(CASE), 94.into());
    }
}
