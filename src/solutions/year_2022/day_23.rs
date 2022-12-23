use hashbrown::{HashSet, HashMap};

use crate::{problem, Solution};

type Point = aoc_lib::Point<isize>;

pub struct Day23;

impl Solution for Day23 {
    fn name(&self) -> &'static str {
        ""
    }

    fn part_a(&self) -> String {
        let raw = problem::load(2022, 23);
        let mut world = World::parse(&raw);
        world.draw();

        dbg!(world);
        todo!()
    }

    fn part_b(&self) -> String {
        let raw = problem::load(2022, 23);
        todo!()
    }
}

#[derive(Debug)]
struct World {
    elves: HashSet<Point>,
}

impl World {
    fn parse(raw: &str) -> Self {
        let mut elves = HashSet::new();

        for (y, line) in raw.lines().enumerate() {
            for (x, c) in line.char_indices() {
                if c == '#' {
                    elves.insert(Point::new(x as isize, y as isize));
                }
            }
        }

        Self { elves }
    }

    fn draw(&self) {
        let (min, max) = self._bounds();

        for y in min.y..=max.y {
            for x in min.x..=max.x {
                if self.elves.contains(&Point::new(x, y)) {
                    print!("#");
                    continue;
                }
                print!(".");
            }
            println!()
        }
    }

    fn step(&mut self) -> bool {
        // Step 1 Move Gen
        // let mut moves = HashMap::new();

        todo!()
    }

    // dir (0-3)
    // right, down, left, up
    fn _dir_clear(&self, pos: Point, dir: u8) -> bool {
        let points = match dir {
            0 => [Point::new(1, 0), Point::new(1, -1), Point::new(1, 1)],
            1 => [Point::new(0, 1), Point::new(-1, 1), Point::new(1, 1)],
            2 => [Point::new(-1, 0), Point::new(-1, -1), Point::new(-1, 1)],
            3 => [Point::new(0, -1), Point::new(-1, -1), Point::new(1, -1)],
            _ => unreachable!()
        };

        for i in points {
            if self.elves.contains(&(pos + i)) {
                return false;
            }
        }

        true
    }

    fn _bounds(&self) -> (Point, Point) {
        let mut min = Point::new(isize::MAX, isize::MAX);
        let mut max = Point::new(isize::MIN, isize::MIN);

        for i in self.elves.iter() {
            min = min.min(i);
            max = max.max(i);
        }

        (min, max)
    }
}
