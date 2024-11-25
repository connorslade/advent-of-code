use std::collections::HashSet;

use nd_vec::{vector, Vec2};

use aoc_lib::direction::Direction;
use common::{Answer, ISolution};

type Pos = Vec2<usize>;

const START_PIECES: [(char, [Direction; 2]); 4] = [
    ('F', [Direction::Down, Direction::Right]),
    ('7', [Direction::Down, Direction::Left]),
    ('J', [Direction::Up, Direction::Left]),
    ('L', [Direction::Up, Direction::Right]),
];

pub struct Day10;

impl ISolution for Day10 {
    fn name(&self) -> &'static str {
        "Pipe Maze"
    }

    fn part_a(&self, input: &str) -> Answer {
        let maze = parse(input);
        (maze.walls().walls.len() / 2).into()
    }

    fn part_b(&self, input: &str) -> Answer {
        let mut maze = parse(input);
        let walls = maze.walls();

        maze.remove_garbage(&walls.walls);
        maze.segments[maze.start.y()][maze.start.x()] = walls.start_piece;

        let mut inside = 0;
        for y in 0..maze.segments.len() {
            for x in 0..maze.segments[y].len() {
                if maze.segments[y][x] != '.' {
                    continue;
                }

                let mut within = false;
                let mut riding = Riding::None;
                for x in (0..x).rev() {
                    if let Some(wall) = walls.walls.get(&vector!(x, y)) {
                        let chr = maze.segments[wall.y()][wall.x()];
                        if chr == '|' {
                            within ^= true;
                        } else if chr == '7' {
                            riding = Riding::Down;
                        } else if chr == 'J' {
                            riding = Riding::Up;
                        } else if chr == 'F' || chr == 'L' {
                            if (riding == Riding::Up && chr != 'L')
                                || (riding == Riding::Down && chr != 'F')
                            {
                                within ^= true;
                            }

                            riding = Riding::None;
                        }
                    }
                }

                inside += within as usize;
            }
        }

        inside.into()
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
enum Riding {
    None,
    Up,
    Down,
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

struct Walls {
    walls: HashSet<Pos>,
    start_piece: char,
}

impl Maze {
    fn walls(&self) -> Walls {
        let mut pos = self.start;
        let mut walls = HashSet::new();
        let mut start_approaches = [Direction::Up; 2];

        'outer: for mut dir in Direction::ALL {
            start_approaches[0] = dir;
            loop {
                walls.insert(pos);
                pos = match dir.try_advance(pos) {
                    Some(p) => p,
                    None => break,
                };

                match turn(dir, self.segments[pos.y()][pos.x()]) {
                    TurnResult::Turn(new_dir) => dir = new_dir,
                    TurnResult::End => {
                        start_approaches[1] = dir.opposite();
                        break 'outer;
                    }
                    TurnResult::Fail => break,
                }
            }

            walls.clear();
            pos = self.start;
        }

        let start_piece = START_PIECES
            .iter()
            .find(|(_, approaches)| start_approaches == *approaches)
            .map(|(piece, _)| *piece)
            .unwrap();

        Walls { walls, start_piece }
    }

    fn remove_garbage(&mut self, walls: &HashSet<Pos>) {
        for y in 0..self.segments.len() {
            for x in 0..self.segments[y].len() {
                if !walls.contains(&vector!(x, y)) {
                    self.segments[y][x] = '.';
                }
            }
        }
    }
}

enum TurnResult {
    Turn(Direction),
    End,
    Fail,
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
    use common::ISolution;
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
