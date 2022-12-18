use hashbrown::{hash_map::Entry, HashMap};

use crate::{problem, Solution};

type Point = aoc_lib::Point<i64>;

pub struct Day17;

impl Solution for Day17 {
    fn name(&self) -> &'static str {
        "Pyroclastic Flow"
    }

    fn part_a(&self) -> String {
        let raw = problem::load(2022, 17);
        let mut world = World::new(&raw);
        process(&mut world, 2022).to_string()
    }

    fn part_b(&self) -> String {
        let raw = problem::load(2022, 17);
        let mut world = World::new(&raw);
        process(&mut world, 1000000000000).to_string()
    }
}

const WIDTH: usize = 7;
const ROCKS: [&[Point]; 5] = [
    &[
        Point::new(0, 0),
        Point::new(1, 0),
        Point::new(2, 0),
        Point::new(3, 0),
    ],
    &[
        Point::new(1, 0),
        Point::new(0, 1),
        Point::new(1, 1),
        Point::new(2, 1),
        Point::new(1, 2),
    ],
    &[
        Point::new(2, 2),
        Point::new(2, 1),
        Point::new(0, 0),
        Point::new(1, 0),
        Point::new(2, 0),
    ],
    &[
        Point::new(0, 0),
        Point::new(0, 1),
        Point::new(0, 2),
        Point::new(0, 3),
    ],
    &[
        Point::new(0, 0),
        Point::new(1, 0),
        Point::new(0, 1),
        Point::new(1, 1),
    ],
];

fn process(world: &mut World, rocks: i64) -> i64 {
    let mut seen = HashMap::new();
    let mut cycle_height = 0;
    let mut i = 0;

    while i < rocks {
        world.add_rock();
        i += 1;

        if world.max_height < 8 {
            continue;
        }

        let state = world.state();
        match seen.entry(state) {
            Entry::Vacant(e) => {
                e.insert((i, world.max_height));
            }
            Entry::Occupied(e) => {
                let (old_i, old_height) = e.get();
                let num_rocks_in_cycle = i - old_i;
                let num_cycles = (rocks - i) / num_rocks_in_cycle;
                i += num_rocks_in_cycle * num_cycles;
                cycle_height += num_cycles * (world.max_height - old_height);
                seen.clear();
            }
        }
    }

    world.max_height + cycle_height
}

#[derive(Debug, Clone, Copy)]
struct Rock {
    rock_id: usize,
    pos: Point,
}

#[derive(Debug, Clone)]
struct World {
    rocks: Vec<Rock>,
    max_height: i64,

    airflow: Vec<char>,
    flow_index: usize,
}

impl World {
    fn new(raw: &str) -> Self {
        Self {
            rocks: Vec::new(),
            max_height: 0,
            airflow: raw.chars().collect(),
            flow_index: 0,
        }
    }

    fn state(&self) -> (Vec<Vec<bool>>, usize, usize) {
        let mut world = vec![[false; WIDTH]; self.max_height as usize + 1];
        for rock in &self.rocks {
            for point in rock.points() {
                world[point.y as usize][point.x as usize] = true;
            }
        }

        let mut rows = Vec::new();
        for row in world.into_iter().rev().take(8) {
            rows.push(row.into_iter().collect::<Vec<_>>());
        }

        (
            rows,
            self.flow_index,
            self.rocks.last().map(|x| x.rock_id).unwrap_or(0),
        )
    }

    fn add_rock(&mut self) {
        let next_rock = self.rocks.last().map(|x| x.rock_id + 1).unwrap_or(0) % ROCKS.len();

        self.rocks.push(Rock {
            rock_id: next_rock,
            pos: Point::new(2, self.max_height + 3),
        });

        let rock_index = self.rocks.len() - 1;
        loop {
            let next_flow = self.next_flow();
            self.try_move_rock(rock_index, next_flow);
            let gravity = self.try_move_rock(rock_index, Point::new(0, -1));
            if !gravity {
                break;
            }
        }
        self.max_height = self.max_height.max(
            self.rocks[rock_index]
                .points()
                .iter()
                .map(|x| x.y)
                .max()
                .unwrap()
                + 1,
        );
    }

    // Returns true if rock was moved.
    fn try_move_rock(&mut self, rock_index: usize, delta: Point) -> bool {
        let mut rock = self.rocks[rock_index];
        rock.pos += delta;

        // Check if rock in new position is valid, meaning it doesn't collide with any other rock or the wall.
        for point in ROCKS[rock.rock_id] {
            let new_pos = rock.pos + *point;
            if new_pos.x < 0 || new_pos.x >= WIDTH as i64 || new_pos.y < 0 {
                return false;
            }

            if self
                .rocks
                .iter()
                .take(rock_index)
                .any(|r| r.points().iter().any(|x| *x == new_pos))
            {
                return false;
            }
        }

        self.rocks[rock_index] = rock;
        true
    }

    fn next_flow(&mut self) -> Point {
        let flow = self.airflow[self.flow_index];
        self.flow_index = (self.flow_index + 1) % self.airflow.len();

        match flow {
            '>' => Point::new(1, 0),
            '<' => Point::new(-1, 0),
            _ => panic!("Invalid flow: {}", flow),
        }
    }
}

impl Rock {
    fn points(&self) -> Vec<Point> {
        ROCKS[self.rock_id]
            .iter()
            .map(|point| self.pos + *point)
            .collect()
    }
}
