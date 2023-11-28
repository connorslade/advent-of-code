use hashbrown::HashSet;
use std::collections::VecDeque;

use crate::aoc_lib;
use common::{Answer, Solution};

type Point = aoc_lib::Point<isize>;
pub struct Day22;

impl Solution for Day22 {
    fn name(&self) -> &'static str {
        "Monkey Map"
    }

    fn part_a(&self, input: &str) -> Answer {
        let mut world = World::parse(input);
        world.run(wrap_2d);
        world.password().into()
    }

    fn part_b(&self, input: &str) -> Answer {
        let mut world = World::parse(input);
        world.run(wrap_3d);
        world.password().into()
    }
}

#[derive(Debug)]
struct World {
    // == World ==
    walls: HashSet<Point>,
    open: HashSet<Point>,
    side_len: usize,

    // == Player ==
    pos: Point,
    dir: Direction,
    instructions: VecDeque<Instruction>,
}

#[derive(Debug)]
enum Instruction {
    Move(usize),
    Turn(Direction),
}

#[derive(Debug, Copy, Clone)]
enum Direction {
    Right,
    Down,
    Left,
    Up,
}

impl World {
    fn parse(raw: &str) -> Self {
        let (map, instructions) = raw.split_once("\n\n").unwrap();

        let mut walls = HashSet::new();
        let mut open = HashSet::new();
        let mut side_len = 0;

        for (y, line) in map.lines().enumerate() {
            for (x, c) in line.chars().enumerate() {
                let (x, y) = (x as isize, y as isize);

                if y == 0 && c != ' ' {
                    side_len += 1;
                }

                match c {
                    '#' => walls.insert(Point::new(x, y)),
                    '.' => open.insert(Point::new(x, y)),
                    ' ' => continue,
                    _ => panic!("Invalid character: {}", c),
                };
            }
        }

        let start = open
            .iter()
            .filter(|p| p.y == 0)
            .min_by_key(|p| p.x)
            .unwrap();

        Self {
            walls,
            pos: *start,
            side_len,
            open,
            dir: Direction::Right,
            instructions: VecDeque::from(Instruction::parse(instructions)),
        }
    }

    fn password(&self) -> usize {
        (1000 * (self.pos.y + 1) + 4 * (self.pos.x + 1) + self.dir as isize) as usize
    }

    fn run(&mut self, wrap: fn(&Self, Point) -> Option<(Point, Direction)>) {
        while let Some(i) = self.instructions.pop_front() {
            match i {
                Instruction::Turn(d) => self.dir = d.turn(&self.dir),
                Instruction::Move(n) => {
                    for _ in 0..n {
                        let new_pos = self.dir.apply(self.pos);
                        if self.walls.contains(&new_pos) {
                            break;
                        }

                        if self.open.contains(&new_pos) {
                            self.pos = new_pos;
                            continue;
                        }

                        if let Some((new_pos, new_dir)) = wrap(self, new_pos) {
                            self.pos = new_pos;
                            self.dir = new_dir;
                        }
                    }
                }
            }
        }
    }
}

impl Instruction {
    /// The directions from these instructions are relative to the current direction.
    /// So if we're facing right, and we turn left, we're now facing up.
    fn parse(raw: &str) -> Vec<Self> {
        let mut out = Vec::new();
        let mut working = String::new();

        for i in raw.chars() {
            match i {
                i if i.is_ascii_digit() => working.push(i),
                'L' => {
                    Self::flush(&mut working, &mut out);
                    out.push(Instruction::Turn(Direction::Left));
                }
                'R' => {
                    Self::flush(&mut working, &mut out);
                    out.push(Instruction::Turn(Direction::Right));
                }
                _ => continue,
            }
        }

        Self::flush(&mut working, &mut out);
        out
    }

    fn flush(working: &mut String, out: &mut Vec<Self>) {
        if !working.is_empty() {
            out.push(Instruction::Move(working.parse().unwrap()));
            working.clear();
        }
    }
}

impl Direction {
    fn from_index(i: usize) -> Self {
        match i {
            0 => Direction::Right,
            1 => Direction::Down,
            2 => Direction::Left,
            3 => Direction::Up,
            _ => panic!("Invalid direction index: {}", i),
        }
    }

    fn apply(&self, pos: Point) -> Point {
        match self {
            Direction::Left => pos - Point::new(1, 0),
            Direction::Right => pos + Point::new(1, 0),
            Direction::Up => pos - Point::new(0, 1),
            Direction::Down => pos + Point::new(0, 1),
        }
    }

    fn turn(&self, dir: &Direction) -> Self {
        Self::from_index(match self {
            Direction::Right => (*dir as usize + 1) % 4,
            Direction::Left => (*dir as usize + 3) % 4,
            _ => panic!("Invalid turn direction: {:?}", self),
        })
    }

    fn reverse(&self) -> Self {
        match self {
            Direction::Left => Direction::Right,
            Direction::Right => Direction::Left,
            Direction::Up => Direction::Down,
            Direction::Down => Direction::Up,
        }
    }
}

fn wrap_2d(world: &World, mut pos: Point) -> Option<(Point, Direction)> {
    loop {
        pos = world.dir.reverse().apply(pos);
        if !world.walls.contains(&pos) && !world.open.contains(&pos) {
            let pos = world.dir.apply(pos);
            if world.walls.contains(&pos) {
                break None;
            }

            break Some((pos, world.dir));
        }
    }
}

fn wrap_3d(world: &World, mut _pos: Point) -> Option<(Point, Direction)> {
    let _ = world.side_len;
    todo!()
}

#[cfg(test)]
mod test {
    use common::Solution;
    use indoc::indoc;

    use super::Day22;

    const CASE: &str = indoc! {"
                ...#
                .#..
                #...
                ....
        ...#.......#
        ........#...
        ..#....#....
        ..........#.
                ...#....
                .....#..
                .#......
                ......#.

        10R5L5R10L4R5L5
    "};

    #[test]
    fn part_a() {
        assert_eq!(Day22.part_a(CASE), 6032.into());
    }

    #[test]
    fn part_b() {
        assert_eq!(Day22.part_b(CASE), 5031.into());
    }
}

/*
                1 1 1 1
                1 1 1 1
                1 1 1 1
                1 1 1 1
2 2 2 2 3 3 3 3 4 4 4 4
2 2 2 2 3 3 3 3 4 4 4 4
2 2 2 2 3 3 3 3 4 4 4 4
2 2 2 2 3 3 3 3 4 4 4 4
                5 5 5 5 6 6 6 6
                5 5 5 5 6 6 6 6
                5 5 5 5 6 6 6 6
                5 5 5 5 6 6 6 6

*/
