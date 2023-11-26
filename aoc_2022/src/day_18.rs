use hashbrown::HashSet;

use crate::aoc_lib::Point3;
use common::Solution;

pub struct Day18;

impl Solution for Day18 {
    fn name(&self) -> &'static str {
        "Boiling Boulders"
    }

    fn part_a(&self, input: &str) -> String {
        let world = World::parse(input);

        let mut open_faces = 0;

        for i in &world.points {
            open_faces += 6 - world.neighbors(i);
        }

        open_faces.to_string()
    }

    fn part_b(&self, input: &str) -> String {
        let world = World::parse(input);

        let outside = world.flood_fill(Point3::new(0, 0, 0));
        let mut out = 0;
        for i in &world.points {
            for j in NEIGHBORS {
                let n = *i + j;
                if !world.points.contains(&n) && outside.contains(&n) {
                    out += 1;
                }
            }
        }

        out.to_string()
    }
}

struct World {
    points: HashSet<Point3>,
}

const NEIGHBORS: [Point3; 6] = [
    Point3::new(1, 0, 0),
    Point3::new(-1, 0, 0),
    Point3::new(0, 1, 0),
    Point3::new(0, -1, 0),
    Point3::new(0, 0, 1),
    Point3::new(0, 0, -1),
];

impl World {
    fn parse(raw: &str) -> Self {
        Self {
            points: HashSet::from_iter(parse(raw).into_iter()),
        }
    }

    fn neighbors(&self, point: &Point3) -> usize {
        let mut out = 0;

        for i in NEIGHBORS {
            out += self.points.contains(&(*point + i)) as usize;
        }

        out
    }

    fn bounds(&self) -> (Point3, Point3) {
        let mut min = Point3::new(i32::MAX, i32::MAX, i32::MAX);
        let mut max = Point3::new(i32::MIN, i32::MIN, i32::MIN);

        for i in &self.points {
            min = min.min(i);
            max = max.max(i);
        }

        (min, max)
    }

    fn flood_fill(&self, start: Point3) -> HashSet<Point3> {
        let bounds = self.bounds();
        let mut steam = HashSet::new();
        let mut new = vec![start];

        while let Some(s) = new.pop() {
            steam.insert(s);
            for n in NEIGHBORS {
                let n = s + n;
                if n.x > bounds.1.x + 1 || n.x < bounds.0.x - 1 {
                    continue;
                }
                if n.y > bounds.1.y + 1 || n.y < bounds.0.y - 1 {
                    continue;
                }
                if n.z > bounds.1.z + 1 || n.z < bounds.0.z - 1 {
                    continue;
                }
                if !self.points.contains(&n) && !steam.contains(&n) && !new.contains(&n) {
                    new.push(n);
                }
            }
        }

        steam
    }
}

fn parse(raw: &str) -> Vec<Point3> {
    let mut out = Vec::new();

    for i in raw.lines() {
        let mut parts = i.split(',');
        out.push(Point3::new(
            parts.next().unwrap().parse().unwrap(),
            parts.next().unwrap().parse().unwrap(),
            parts.next().unwrap().parse().unwrap(),
        ));
    }

    out
}
