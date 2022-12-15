use hashbrown::HashSet;

use crate::{problem, Solution};

type Point = aoc_lib::Point<isize>;

pub struct Day15;

impl Solution for Day15 {
    fn name(&self) -> &'static str {
        ""
    }

    // NOT:
    // - 4892837

    fn part_a(&self) -> String {
        let raw = problem::load(2022, 15);
        let mut blocked = HashSet::new();
        let sensors = parse(&raw);
        for i in &sensors {
            blocked.extend(i.safe_points(&sensors, 2000000));
        }

        (blocked.len() + 1).to_string()
    }

    fn part_b(&self) -> String {
        let raw = problem::load(2022, 15);
        todo!()
    }
}

#[derive(Debug)]
struct Sensor {
    pos: Point,
    // Position of the closest beacon
    beacon: Point,
}

impl Sensor {
    fn parse(raw: &str) -> Self {
        let parts = raw.split("at ").collect::<Vec<_>>();
        let parse_pos = |pos: &str| {
            let parts = pos.split(", ").collect::<Vec<_>>();
            Point::new(
                parts[0].trim_start_matches("x=").parse().unwrap(),
                parts[1]
                    .trim_start_matches("y=")
                    .split(':')
                    .next()
                    .unwrap()
                    .parse()
                    .unwrap(),
            )
        };

        Self {
            pos: parse_pos(parts[1]),
            beacon: parse_pos(parts[2]),
        }
    }

    // Returns the points that are within the manhattan distance of the sensor to the beacon
    fn safe_points(&self, sensors: &[Sensor], y: isize) -> Vec<Point> {
        let mut points = Vec::new();
        let beacon_distance = manhattan_distance(self.beacon, self.pos);

        for x in (self.pos.x - beacon_distance)..=(self.beacon.x + beacon_distance) {
            let point = Point::new(x, y);
            if manhattan_distance(point, self.pos) <= beacon_distance
            // && !sensors.iter().any(|x| x.beacon == point)
            {
                points.push(point);
            }
        }

        points
    }
}

fn parse(raw: &str) -> Vec<Sensor> {
    raw.lines().map(Sensor::parse).collect()
}

fn manhattan_distance(a: Point, b: Point) -> isize {
    (a.x - b.x).abs() + (a.y - b.y).abs()
}
