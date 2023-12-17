use std::collections::{HashMap, VecDeque};

use common::{Answer, Solution};
use nd_vec::{vector, Vec2};

type Pos = Vec2<usize>;

pub struct Day17;

// this is quite the spaghetti
impl Solution for Day17 {
    fn name(&self) -> &'static str {
        "Clumsy Crucible"
    }

    fn part_a(&self, input: &str) -> Answer {
        let board = parse(input);

        #[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
        struct State {
            pos: Pos,
            facing: Direction,
            turn_distance: u8,
            steps: u32,
        }

        let start = State {
            pos: vector!(0, 0),
            facing: Direction::Right,
            turn_distance: 0,
            steps: 0,
        };

        let path = pathfinding::directed::dijkstra::dijkstra(
            &start,
            |x| {
                let mut next = Vec::new();

                // Straight
                if x.turn_distance < 2 {
                    if let Some(next_pos) = x.facing.advance(x.pos) {
                        if next_pos.x() < board.len() && next_pos.y() < board[0].len() {
                            let cell = board[next_pos.x()][next_pos.y()] as u32;
                            next.push((
                                State {
                                    pos: next_pos,
                                    facing: x.facing,
                                    turn_distance: x.turn_distance + 1,
                                    steps: x.steps + cell,
                                },
                                cell,
                            ));
                        }
                    }
                }

                // Left
                if let Some(next_facing) = x.facing.turn_left().advance(x.pos) {
                    if next_facing.x() < board.len() && next_facing.y() < board[0].len() {
                        let cell = board[next_facing.x()][next_facing.y()] as u32;
                        next.push((
                            State {
                                pos: next_facing,
                                facing: x.facing.turn_left(),
                                turn_distance: 0,
                                steps: x.steps + cell,
                            },
                            cell,
                        ));
                    }
                }

                // Right
                if let Some(next_facing) = x.facing.turn_right().advance(x.pos) {
                    if next_facing.x() < board.len() && next_facing.y() < board[0].len() {
                        let cell = board[next_facing.x()][next_facing.y()] as u32;
                        next.push((
                            State {
                                pos: next_facing,
                                facing: x.facing.turn_right(),
                                turn_distance: 0,
                                steps: x.steps + cell,
                            },
                            cell,
                        ));
                    }
                }

                next
            },
            |x| x.pos == vector!(board.len() - 1, board[0].len() - 1),
        );

        path.unwrap().1.into()
    }

    fn part_b(&self, input: &str) -> Answer {
        let board = parse(input);
        pathfind(&board, 4, 10).into()
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
struct State {
    pos: Pos,
    facing: Direction,
    turn_distance: u8,
    steps: u32,
}

fn pathfind(board: &[Vec<u8>], lower: u8, upper: u8) -> u32 {
    let mut res = u32::MAX;
    let mut queue = VecDeque::new();
    let mut visited = HashMap::new();

    let end = vector!(board[0].len() - 1, board.len() - 1);
    for dir in [Direction::Down, Direction::Right] {
        let state = (vector!(0, 0), dir, 1);
        queue.push_back((state, 0));
        visited.insert(state, 0);
    }

    while let Some(((pos, facing, turn_distance), cost)) = queue.pop_front() {
        let mut push = |dir: Direction, turn_distance: u8| {
            if let Some(next) = dir.advance(pos) {
                if next.y() < board.len() && next.x() < board[0].len() {
                    let state = (next, dir, turn_distance);
                    let cell = board[next.y()][next.x()] as u32;
                    let cost = cost + cell;

                    if !visited.contains_key(&state) || visited[&state] > cost {
                        queue.push_back((state, cost));
                        visited.insert(state, cost);
                    }
                }
            }
        };

        if pos == end && turn_distance >= 4 {
            res = res.min(cost);
            continue;
        }

        if turn_distance < upper {
            push(facing, turn_distance + 1);
        }

        if turn_distance >= lower {
            push(facing.turn_left(), 1);
            push(facing.turn_right(), 1);
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
enum Direction {
    Up,
    Down,
    Left,
    Right,
}

impl Direction {
    fn advance(&self, pos: Pos) -> Option<Pos> {
        Some(match self {
            Self::Up if pos.y() > 0 => pos - vector!(0, 1),
            Self::Down => pos + vector!(0, 1),
            Self::Left if pos.x() > 0 => pos - vector!(1, 0),
            Self::Right => pos + vector!(1, 0),
            _ => return None,
        })
    }

    fn turn_left(&self) -> Self {
        match self {
            Self::Up => Self::Left,
            Self::Down => Self::Right,
            Self::Left => Self::Down,
            Self::Right => Self::Up,
        }
    }

    fn turn_right(&self) -> Self {
        match self {
            Self::Up => Self::Right,
            Self::Down => Self::Left,
            Self::Left => Self::Up,
            Self::Right => Self::Down,
        }
    }
}

#[cfg(test)]
mod test {
    use common::Solution;
    use indoc::indoc;

    use super::Day17;

    // const CASE: &str = indoc! {"
    //     111111111111
    //     999999999991
    //     999999999991
    //     999999999991
    //     999999999991
    // "};

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
