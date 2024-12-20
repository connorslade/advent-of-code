use std::{
    cmp::Ordering,
    collections::{BinaryHeap, HashMap},
};

use aoc_lib::{direction::cardinal::Direction, matrix::Grid};
use common::{solution, Answer};
use nd_vec::{vector, Vec2};

solution!("Clumsy Crucible", 17);

type Pos = Vec2<usize>;

fn part_a(input: &str) -> Answer {
    pathfind(parse(input), 1, 3).into()
}

fn part_b(input: &str) -> Answer {
    pathfind(parse(input), 4, 10).into()
}

fn parse(input: &str) -> Grid<u8> {
    Grid::parse(input, |c| c as u8 - b'0')
}

fn pathfind(board: Grid<u8>, min_dist: u8, max_dist: u8) -> u32 {
    let mut queue = BinaryHeap::new();
    let mut visited = HashMap::new();
    let mut res = u32::MAX;

    let end = board.size() - vector!(1, 1);
    for dir in [Direction::Down, Direction::Right] {
        let state = State::new(vector!(0, 0), dir, 1);
        queue.push(QueueItem { state, cost: 0 });
        visited.insert(state, 0);
    }

    while let Some(item) = queue.pop() {
        let state = item.state;
        let mut explore = |facing: Direction, turn_distance: u8| {
            if let Some(pos) = facing
                .try_advance(state.pos)
                .filter(|pos| board.contains(*pos))
            {
                let state = State::new(pos, facing, turn_distance);
                let cost = item.cost + board[pos] as u32;

                if !visited.contains_key(&state) || visited.get(&state).unwrap() > &cost {
                    queue.push(QueueItem { state, cost });
                    visited.insert(state, cost);
                }
            }
        };

        if state.pos == end && state.turn_distance >= min_dist {
            res = res.min(item.cost);
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

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
struct State {
    pos: Pos,
    facing: Direction,
    turn_distance: u8,
}

#[derive(PartialEq, Eq)]
struct QueueItem {
    state: State,
    cost: u32,
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

impl Ord for QueueItem {
    fn cmp(&self, other: &Self) -> Ordering {
        other.cost.cmp(&self.cost)
    }
}

impl PartialOrd for QueueItem {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        Some(self.cmp(other))
    }
}

#[cfg(test)]
mod test {
    use indoc::indoc;

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
        assert_eq!(super::part_a(CASE), 102.into());
    }

    #[test]
    fn part_b() {
        assert_eq!(super::part_b(CASE), 94.into());
    }
}
