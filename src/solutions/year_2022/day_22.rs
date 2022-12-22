use std::collections::VecDeque;

use hashbrown::HashSet;

use crate::{problem, Solution};

type Point = aoc_lib::Point<usize>;

pub struct Day22;

impl Solution for Day22 {
    fn name(&self) -> &'static str {
        "Monkey Map"
    }

    fn part_a(&self) -> String {
        let raw = problem::load_raw(2022, 22);
        let mut world = World::parse(&raw);
        world.run();

        (100 * (world.pos.y + 1) + 4 * (world.pos.x + 1)).to_string()
    }

    fn part_b(&self) -> String {
        let raw = problem::load(2022, 22);
        todo!()
    }
}

#[derive(Debug)]
struct World {
    // == World ==
    walls: HashSet<Point>,
    open: HashSet<Point>,

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

#[derive(Debug)]
enum Direction {
    Left,
    Right,
    Up,
    Down,
}

impl World {
    fn parse(raw: &str) -> Self {
        let raw = raw.replace('\r', "");
        let (map, instructions) = raw.split_once("\n\n").unwrap();

        let mut walls = HashSet::new();
        let mut open = HashSet::new();

        for (y, line) in map.lines().enumerate() {
            for (x, c) in line.chars().enumerate() {
                match c {
                    '#' => walls.insert(Point::new(x, y)),
                    '.' => open.insert(Point::new(x, y)),
                    ' ' | '\n' => continue,
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
            open,
            dir: Direction::Right,
            instructions: VecDeque::from(Instruction::parse(instructions)),
        }
    }

    fn run(&mut self) {
        while let Some(i) = self.instructions.pop_front() {
            match i {
                Instruction::Turn(d) => self.dir = d.turn(&self.dir),
                Instruction::Move(n) => {
                    for _ in 0..n {
                        let mut new_pos = self.dir.apply(self.pos);
                        if self.walls.contains(&new_pos) {
                            break;
                        }

                        if self.open.contains(&new_pos) {
                            self.pos = new_pos;
                            continue;
                        }

                        // Wrap around the row
                        // If we get to this point, the new_pos is not a point in the map and tharefore should be wrapped around
                        // The map is not rectangular, so we can't just use the width of the map.
                        // First check which direction we're going in, then see if its up/down or left/right
                        // Then step in the opposite direction until we hit a wall, then step back one.
                        loop {
                            new_pos = self.dir.reverse().apply(new_pos);
                            if !self.walls.contains(&new_pos) && self.open.contains(&new_pos) {
                                self.pos = self.dir.apply(new_pos);
                                break;
                            }
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
                    Self::_flush(&mut working, &mut out);
                    out.push(Instruction::Turn(Direction::Left));
                }
                'R' => {
                    Self::_flush(&mut working, &mut out);
                    out.push(Instruction::Turn(Direction::Right));
                }
                _ => continue,
            }
        }

        Self::_flush(&mut working, &mut out);
        out
    }

    fn _flush(working: &mut String, out: &mut Vec<Self>) {
        if !working.is_empty() {
            out.push(Instruction::Move(working.parse().unwrap()));
            working.clear();
        }
    }
}

impl Direction {
    fn apply(&self, pos: Point) -> Point {
        match self {
            Direction::Left => pos - Point::new(1, 0),
            Direction::Right => pos + Point::new(1, 0),
            Direction::Up => pos - Point::new(0, 1),
            Direction::Down => pos + Point::new(0, 1),
        }
    }

    fn turn(&self, dir: &Direction) -> Self {
        match (self, dir) {
            (Direction::Right, Direction::Left) => Direction::Up,
            (Direction::Right, Direction::Right) => Direction::Down,
            (Direction::Right, Direction::Up) => Direction::Right,
            (Direction::Right, Direction::Down) => Direction::Left,
            (Direction::Left, Direction::Left) => Direction::Down,
            (Direction::Left, Direction::Right) => Direction::Up,
            (Direction::Left, Direction::Up) => Direction::Left,
            (Direction::Left, Direction::Down) => Direction::Right,
            _ => unreachable!(),
        }
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
