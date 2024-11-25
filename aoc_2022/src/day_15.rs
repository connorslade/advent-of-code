use common::{solution, Answer};

use nd_vec::vector;
use rayon::prelude::*;

type Point = nd_vec::Vec2<isize>;

solution!("Beacon Exclusion Zone", (2022, 00));

fn part_a(input: &str) -> Answer {
    let world = World::parse(input);
    let y_level = 2000000; // 10 for example

    let blocked = (world.bounds.0.x()..=world.bounds.1.x())
        .into_par_iter()
        .map(|x| vector!(x, y_level))
        .filter(|x| world.is_sensed(*x))
        .count();

    (blocked - 1).into()
}

fn part_b(input: &str) -> Answer {
    let world = World::parse(input);
    let bounds = 4000000;

    let distress = world.search(bounds).expect("No distress beacon found");
    (distress.x() * 4000000 + distress.y()).into()
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
            min_bound = min_bound.min(&(sensor.pos - sensor.distance));
            max_bound = max_bound.max(&(sensor.pos + sensor.distance));
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
                let x1 = s.pos.x() - (s.distance - (s.pos.y() - y).abs()) - 1;
                let x2 = s.pos.x() + (s.distance - (s.pos.y() - y).abs()) + 1;

                if x1 > 0 && x1 < bounds && !self.is_sensed(vector!(x1, y)) {
                    return Some(vector!(x1, y));
                }

                if x2 > 0 && x2 < bounds && !self.is_sensed(vector!(x2, y)) {
                    return Some(vector!(x2, y));
                }
            }

            None
        })
    }

    fn is_sensed(&self, point: Point) -> bool {
        self.sensors
            .iter()
            .any(|x| x.pos.manhattan_distance(&point) <= x.distance)
    }
}

impl Sensor {
    fn parse(raw: &str) -> Self {
        let parts = raw.split("at ").collect::<Vec<_>>();
        let parse_pos = |pos: &str| {
            let parts = pos.split(", ").collect::<Vec<_>>();
            vector!(
                parts[0].trim_start_matches("x=").parse().unwrap(),
                parts[1]
                    .trim_start_matches("y=")
                    .split(':')
                    .next()
                    .unwrap()
                    .parse()
                    .unwrap()
            )
        };

        let pos = parse_pos(parts[1]);
        let beacon = parse_pos(parts[2]);

        Self {
            pos,
            distance: pos.manhattan_distance(&beacon),
        }
    }
}
