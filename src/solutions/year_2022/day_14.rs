use crate::{problem, Solution};

type Point = aoc_lib::Point<usize>;

pub struct Day14;

impl Solution for Day14 {
    fn name(&self) -> &'static str {
        "Regolith Reservoir"
    }

    fn part_a(&self) -> String {
        let raw = problem::load(2022, 14);
        let mut world = World::parse(&raw);
        let bounds = world.lines.iter().map(|x| x.0.y).max().unwrap();

        'o: loop {
            world.sand.push(Point::new(500, 0));
            while !world.tick(None) {
                if world.sand.last().unwrap().y > bounds {
                    break 'o;
                }
            }
        }

        (world.sand.len() - 1).to_string()
    }

    fn part_b(&self) -> String {
        let raw = problem::load(2022, 14);
        let mut world = World::parse(&raw);
        let bounds = world.lines.iter().map(|x| x.0.y).max().unwrap();
        let mut i = 1;

        loop {
            if i == 0 {
                break;
            }

            i = 0;
            world.sand.push(Point::new(500, 0));
            while !world.tick(Some(bounds + 2)) {
                i += 1;
            }
        }

        world.sand.len().to_string()
    }
}

#[derive(Debug)]
struct World {
    lines: Vec<Line>,
    sand: Vec<Point>,
}

#[derive(Debug)]
struct Line(Point, Point);

impl World {
    fn parse(raw: &str) -> Self {
        let mut lines = Vec::new();

        for i in raw.lines() {
            for j in i.split(" -> ").collect::<Vec<_>>().windows(2) {
                lines.push(Line::parse(j[0], j[1]));
            }
        }

        Self {
            lines,
            sand: Vec::new(),
        }
    }

    fn tick(&mut self, floor_y: Option<usize>) -> bool {
        let sand = *self.sand.last().unwrap();
        if !self.pos_blocked(Point::new(sand.x, sand.y + 1), floor_y) {
            self.sand.last_mut().unwrap().y += 1;
            return false;
        }

        if !self.pos_blocked(Point::new(sand.x - 1, sand.y + 1), floor_y) {
            let last = self.sand.last_mut().unwrap();
            last.x -= 1;
            last.y += 1;
            return false;
        }

        if !self.pos_blocked(Point::new(sand.x + 1, sand.y + 1), floor_y) {
            let last = self.sand.last_mut().unwrap();
            last.x += 1;
            last.y += 1;
            return false;
        }

        true
    }

    fn pos_blocked(&self, pos: Point, floor_y: Option<usize>) -> bool {
        self.lines.iter().any(|l| l.has_point(pos))
            || self.sand.contains(&pos)
            || floor_y.map(|y| pos.y >= y).unwrap_or(false)
    }
}

impl Line {
    fn parse(a: &str, b: &str) -> Self {
        fn point(x: &str) -> Point {
            let mut x = x.split(',').map(|x| x.parse::<usize>().unwrap());
            Point {
                x: x.next().unwrap(),
                y: x.next().unwrap(),
            }
        }

        Self(point(a), point(b))
    }

    fn has_point(&self, pos: Point) -> bool {
        let order_range = |a: usize, b: usize| a.min(b)..=a.max(b);
        let (a, b) = (self.0, self.1);

        if a.x == b.x {
            return pos.x == a.x && order_range(a.y, b.y).contains(&pos.y);
        }

        pos.y == a.y && order_range(a.x, b.x).contains(&pos.x)
    }
}
