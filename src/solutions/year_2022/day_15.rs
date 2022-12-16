use crate::{problem, Solution};

use rayon::prelude::*;

type Point = aoc_lib::Point<isize>;

pub struct Day15;

impl Solution for Day15 {
    fn name(&self) -> &'static str {
        "Beacon Exclusion Zone"
    }

    fn part_a(&self) -> String {
        let raw = problem::load(2022, 15);
        let world = World::parse(&raw);
        let y_level = 2000000; // 10 for example

        let blocked = (world.bounds.0.x..=world.bounds.1.x)
            .into_par_iter()
            .map(|x| Point::new(x, y_level))
            .filter(|x| world.is_sensed(*x))
            .count();

        (blocked - 1).to_string()
    }

    fn part_b(&self) -> String {
        let raw = problem::load(2022, 15);
        let world = World::parse(&raw);
        let bounds = 4000000;

        let distress = world.search(bounds).expect("No distress beacon found");
        (distress.x * 4000000 + distress.y).to_string()
    }
}

struct World {
    sensors: Vec<Sensor>,
    bounds: (Point, Point),
}

#[derive(Debug)]
struct Sensor {
    pos: Point,
    distance: isize,
}

impl World {
    fn parse(raw: &str) -> Self {
        let mut sensors = Vec::new();
        let (mut min_bound, mut max_bound) = (Point::default(), Point::default());

        for i in raw.lines() {
            let sensor = Sensor::parse(i);
            min_bound.x = min_bound.x.min(sensor.pos.x - sensor.distance);
            min_bound.y = min_bound.y.min(sensor.pos.y - sensor.distance);
            max_bound.x = max_bound.x.max(sensor.pos.x + sensor.distance);
            max_bound.y = max_bound.y.max(sensor.pos.y + sensor.distance);
            sensors.push(sensor);
        }

        Self {
            sensors,
            bounds: (min_bound, max_bound),
        }
    }

    fn search(&self, bounds: isize) -> Option<Point> {
        (0..=bounds).into_par_iter().find_map_any(|y| {
            for s in &self.sensors {
                let x1 = s.pos.x - (s.distance - (s.pos.y - y).abs()) - 1;
                let x2 = s.pos.x + (s.distance - (s.pos.y - y).abs()) + 1;

                if x1 > 0 && x1 < bounds && !self.is_sensed(Point::new(x1, y)) {
                    return Some(Point::new(x1, y));
                }

                if x2 > 0 && x2 < bounds && !self.is_sensed(Point::new(x2, y)) {
                    return Some(Point::new(x2, y));
                }
            }

            None
        })
    }

    fn is_sensed(&self, point: Point) -> bool {
        self.sensors
            .iter()
            .any(|x| manhattan_distance(x.pos, point) <= x.distance)
    }
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

        let pos = parse_pos(parts[1]);
        let beacon = parse_pos(parts[2]);

        Self {
            pos,
            distance: manhattan_distance(pos, beacon),
        }
    }
}

fn manhattan_distance(a: Point, b: Point) -> isize {
    (a.x - b.x).abs() + (a.y - b.y).abs()
}
