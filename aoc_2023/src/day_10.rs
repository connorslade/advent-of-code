use std::collections::HashSet;

use nd_vec::{vector, Vec2};

use common::{Answer, Solution};

type Pos = Vec2<usize>;

pub struct Day10;

impl Solution for Day10 {
    fn name(&self) -> &'static str {
        "Pipe Maze"
    }

    fn part_a(&self, input: &str) -> Answer {
        let maze = parse(input);

        let mut pos = maze.start;
        let mut dir = Direction::Right;
        let mut len = 0;

        loop {
            len += 1;
            pos = match dir {
                Direction::Up => vector!(pos.x(), pos.y() - 1),
                Direction::Down => vector!(pos.x(), pos.y() + 1),
                Direction::Left => vector!(pos.x() - 1, pos.y()),
                Direction::Right => vector!(pos.x() + 1, pos.y()),
            };

            let pipe = maze.segments[pos.y()][pos.x()];
            dir = match pipe {
                '|' | '-' => dir,
                '7' => match dir {
                    Direction::Right => Direction::Down,
                    Direction::Up => Direction::Left,
                    _ => unreachable!(),
                },
                'L' => match dir {
                    Direction::Left => Direction::Up,
                    Direction::Down => Direction::Right,
                    _ => unreachable!(),
                },
                'J' => match dir {
                    Direction::Right => Direction::Up,
                    Direction::Down => Direction::Left,
                    _ => unreachable!(),
                },
                'F' => match dir {
                    Direction::Up => Direction::Right,
                    Direction::Left => Direction::Down,
                    _ => unreachable!(),
                },
                'S' => break,
                _ => unreachable!(),
            };
        }

        (len / 2).into()
    }

    fn part_b(&self, input: &str) -> Answer {
        let maze = parse(input);

        let mut pos = maze.start;
        let mut dir = Direction::Right;
        let mut walls = HashSet::new();

        loop {
            walls.insert(pos);
            pos = match dir {
                Direction::Up => vector!(pos.x(), pos.y() - 1),
                Direction::Down => vector!(pos.x(), pos.y() + 1),
                Direction::Left => vector!(pos.x() - 1, pos.y()),
                Direction::Right => vector!(pos.x() + 1, pos.y()),
            };

            let pipe = maze.segments[pos.y()][pos.x()];
            dir = match pipe {
                '|' | '-' => dir,
                '7' => match dir {
                    Direction::Right => Direction::Down,
                    Direction::Up => Direction::Left,
                    _ => unreachable!(),
                },
                'L' => match dir {
                    Direction::Left => Direction::Up,
                    Direction::Down => Direction::Right,
                    _ => unreachable!(),
                },
                'J' => match dir {
                    Direction::Right => Direction::Up,
                    Direction::Down => Direction::Left,
                    _ => unreachable!(),
                },
                'F' => match dir {
                    Direction::Up => Direction::Right,
                    Direction::Left => Direction::Down,
                    _ => unreachable!(),
                },
                'S' => break,
                _ => unreachable!(),
            };
        }

        // Flood fill to find the number of reachable not reachable from the outside

        let pos = vector!(maze.segments[0].len(), maze.segments.len());
        // let pos = vector!(0, 0);
        let mut stack = vec![pos];
        let mut visited = HashSet::new();

        while let Some(pos) = stack.pop() {
            let [x, y] = [pos.x() as isize, pos.y() as isize];
            for pos in [
                vector!(x, y - 1),
                vector!(x, y + 1),
                vector!(x - 1, y),
                vector!(x + 1, y),
                vector!(x + 1, y + 1),
                vector!(x - 1, y + 1),
                vector!(x + 1, y - 1),
                vector!(x - 1, y - 1),
            ] {
                if pos.x() < 0 || pos.y() < 0 {
                    continue;
                }

                let pos = vector!(pos.x() as usize, pos.y() as usize);

                let in_bounds =
                    pos.x() < maze.segments[0].len() + 2 && pos.y() < maze.segments.len() + 2;
                if !walls.contains(&(pos + vector!(1, 1))) && in_bounds && !visited.contains(&pos) {
                    stack.push(pos);
                    visited.insert(pos);
                }
            }
        }
        dbg!(
            (maze.segments.len() + 2) * (maze.segments[0].len() + 2),
            visited.len(),
            walls.len()
        );

        (((maze.segments.len() + 2) * (maze.segments[0].len() + 2)) - visited.len() - walls.len())
            .into()
    }
}

enum Direction {
    Up,
    Down,
    Left,
    Right,
}

struct Maze {
    start: Pos,
    segments: Vec<Vec<char>>,
}

fn parse(input: &str) -> Maze {
    let start = input
        .lines()
        .enumerate()
        .find_map(|(y, l)| {
            l.chars()
                .enumerate()
                .find_map(|(x, c)| if c == 'S' { Some((x, y)) } else { None })
        })
        .unwrap();

    Maze {
        start: vector!(start.0, start.1),
        segments: input.lines().map(|l| l.chars().collect()).collect(),
    }
}

#[cfg(test)]
mod test {
    use common::Solution;
    use indoc::indoc;

    use super::Day10;

    const CASE_A: &str = indoc! {"
        .....
        .S-7.
        .|.|.
        .L-J.
        .....
    "};

    const CASE_B: &str = indoc! {"
        .F----7F7F7F7F-7....
        .|F--7||||||||FJ....
        .||.FJ||||||||L7....
        FJL7L7LJLJ||LJ.L-7..
        L--J.L7...LJS7F-7L7.
        ....F-J..F7FJ|L7L7L7
        ....L7.F7||L7|.L7L7|
        .....|FJLJ|FJ|F7|.LJ
        ....FJL-7.||.||||...
        ....L---J.LJ.LJLJ...
    "};

    #[test]
    fn part_a() {
        assert_eq!(Day10.part_a(CASE_A), 4.into());
    }

    #[test]
    fn part_b() {
        assert_eq!(Day10.part_b(CASE_B), 8.into());
    }
}
