use crate::{problem, Solution};
use hashbrown::HashSet;
use std::collections::VecDeque;

type Point = aoc_lib::Point<isize>;
pub struct Day22;

impl Solution for Day22 {
    fn name(&self) -> &'static str {
        "Monkey Map"
    }

    // 37396 - Too Low
    fn part_a(&self) -> String {
        let raw = problem::load_raw(2022, 22);
        let mut world = World::parse(&raw);
        world.run(wrap_1);

        world.password().to_string()
    }

    fn part_b(&self) -> String {
        let _raw = problem::load(2022, 22);
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

#[derive(Debug, Copy, Clone)]
enum Direction {
    Right,
    Down,
    Left,
    Up,
}

impl World {
    fn parse(raw: &str) -> Self {
        let raw = raw.replace('\r', "");
        let (map, instructions) = raw.split_once("\n\n").unwrap();

        let mut walls = HashSet::new();
        let mut open = HashSet::new();

        for (y, line) in map.lines().enumerate() {
            for (x, c) in line.chars().enumerate() {
                let (x, y) = (x as isize, y as isize);
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

fn wrap_1(world: &World, mut pos: Point) -> Option<(Point, Direction)> {
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
