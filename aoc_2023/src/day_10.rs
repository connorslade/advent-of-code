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
        let mut len = 0;

        'outer: for mut dir in Direction::ALL {
            loop {
                pos = dir.step(pos).unwrap();
                len += 1;

                match turn(dir, maze.segments[pos.y()][pos.x()]) {
                    TurnResult::Turn(new_dir) => dir = new_dir,
                    TurnResult::End => break 'outer,
                    TurnResult::Fail => break,
                }
            }

            len = 0;
            pos = maze.start;
        }

        (len / 2).into()
    }

    fn part_b(&self, input: &str) -> Answer {
        let maze = parse(input);

        let (org_x, org_y) = (maze.segments[0].len(), maze.segments.len());
        let mut org_walls = 0;

        let mut start = maze.start;
        let mut segments = maze
            .segments
            .iter()
            .enumerate()
            .flat_map(|(y, line)| {
                let mut new = vec![Vec::with_capacity(line.len() * 2); 2];

                for (x, c) in line.iter().enumerate() {
                    if c != &'.' {
                        org_walls += 1;
                    }

                    match c {
                        '|' => {
                            new[0].extend_from_slice(&['|', '.']);
                            new[1].extend_from_slice(&['|', '.']);
                        }
                        '-' => {
                            new[0].extend_from_slice(&['-', '-']);
                            new[1].extend_from_slice(&['.', '.']);
                        }
                        '7' => {
                            new[0].extend_from_slice(&['7', '.']);
                            new[1].extend_from_slice(&['|', '.']);
                        }
                        'F' => {
                            new[0].extend_from_slice(&['F', '-']);
                            new[1].extend_from_slice(&['|', '.']);
                        }
                        'L' => {
                            new[0].extend_from_slice(&['L', '-']);
                            new[1].extend_from_slice(&['.', '.']);
                        }
                        'J' => {
                            new[0].extend_from_slice(&['J', '.']);
                            new[1].extend_from_slice(&['.', '.']);
                        }
                        'S' => {
                            start = vector!(x * 2, y * 2);
                            new[0].extend_from_slice(&[
                                'S',
                                match maze.segments[y][x + 1] {
                                    '-' | '7' | 'J' => '-',
                                    _ => '.',
                                },
                            ]);
                            new[1].extend_from_slice(&[
                                match maze.segments[y + 1][x] {
                                    '|' | 'L' | 'J' => '|',
                                    _ => '.',
                                },
                                '.',
                            ]);
                        }
                        '.' => {
                            new[0].extend_from_slice(&['.', '.']);
                            new[1].extend_from_slice(&['.', '.']);
                        }
                        _ => unreachable!(),
                    }
                }

                new
            })
            .collect::<Vec<_>>();

        let mut pos = start;
        let mut walls = HashSet::new();

        'outer: for mut dir in Direction::ALL {
            loop {
                walls.insert(pos);
                pos = match dir.step(pos) {
                    Some(pos) => pos,
                    None => break,
                };

                match turn(dir, segments[pos.y()][pos.x()]) {
                    TurnResult::Turn(new_dir) => dir = new_dir,
                    TurnResult::End => break 'outer,
                    TurnResult::Fail => break,
                }
            }

            pos = start;
            walls.clear();
        }

        for y in 0..segments.len() {
            for x in 0..segments[0].len() {
                if !walls.contains(&vector!(x, y)) && segments[y][x] != '.' {
                    segments[y][x] = '.';

                    if x % 2 == 0 && y % 2 == 0 {
                        org_walls -= 1;
                    }
                }
            }
        }

        // Flood fill to find the number of reachable not reachable from the outside
        let mut stack = vec![];

        // Add perimeter to stack
        for x in 0..segments[0].len() {
            stack.push(vector!(x, 0));
            stack.push(vector!(x, segments.len() - 1));
        }

        for y in 0..segments.len() {
            stack.push(vector!(0, y));
            stack.push(vector!(segments[0].len() - 1, y));
        }

        stack.retain(|x| segments[x.y()][x.x()] == '.');

        let mut visited = HashSet::new();
        let mut outside = 0;

        while let Some(pos) = stack.pop() {
            let [x, y] = [pos.x() as isize, pos.y() as isize];
            for pos in [
                vector!(x, y - 1),
                vector!(x, y + 1),
                vector!(x - 1, y),
                vector!(x + 1, y),
            ] {
                if pos.x() < 0 || pos.y() < 0 {
                    continue;
                }

                let pos = vector!(pos.x() as usize, pos.y() as usize);

                let in_bounds = pos.x() < segments[0].len() && pos.y() < segments.len();
                if in_bounds && !visited.contains(&pos) && segments[pos.y()][pos.x()] == '.' {
                    stack.push(pos);
                    visited.insert(pos);

                    if pos.x() % 2 == 0 && pos.y() % 2 == 0 {
                        outside += 1;
                    }
                }
            }
        }

        (org_x * org_y - outside - org_walls).into()
    }
}

#[derive(Debug, Clone, Copy)]
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

enum TurnResult {
    Turn(Direction),
    End,
    Fail,
}

impl Direction {
    const ALL: [Direction; 4] = [
        Direction::Up,
        Direction::Down,
        Direction::Left,
        Direction::Right,
    ];

    fn step(&self, pos: Pos) -> Option<Pos> {
        Some(match self {
            Direction::Up if pos.y() > 0 => vector!(pos.x(), pos.y() - 1),
            Direction::Down => vector!(pos.x(), pos.y() + 1),
            Direction::Left if pos.x() > 0 => vector!(pos.x() - 1, pos.y()),
            Direction::Right => vector!(pos.x() + 1, pos.y()),
            _ => return None,
        })
    }
}

fn turn(facing: Direction, tile: char) -> TurnResult {
    TurnResult::Turn(match (tile, facing) {
        ('|' | '-', _) => facing,
        ('7', Direction::Right) => Direction::Down,
        ('7', Direction::Up) => Direction::Left,
        ('L', Direction::Left) => Direction::Up,
        ('L', Direction::Down) => Direction::Right,
        ('J', Direction::Right) => Direction::Up,
        ('J', Direction::Down) => Direction::Left,
        ('F', Direction::Up) => Direction::Right,
        ('F', Direction::Left) => Direction::Down,
        ('S', _) => return TurnResult::End,
        _ => return TurnResult::Fail,
    })
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
        FF7FSF7F7F7F7F7F---7
        L|LJ||||||||||||F--J
        FL-7LJLJ||||||LJL-77
        F--JF--7||LJLJ7F7FJ-
        L---JF-JLJ.||-FJLJJ7
        |F|F-JF---7F7-L7L|7|
        |FFJF7L7F-JF7|JL---7
        7-L-JL7||F7|L7F-7F7|
        L.L7LFJ|||||FJL7||LJ
        L7JLJL-JLJLJL--JLJ.L
    "};

    #[test]
    fn part_a() {
        assert_eq!(Day10.part_a(CASE_A), 4.into());
    }

    #[test]
    fn part_b() {
        assert_eq!(Day10.part_b(CASE_B), 10.into());
    }
}
