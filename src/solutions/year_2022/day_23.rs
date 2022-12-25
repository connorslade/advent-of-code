use hashbrown::{hash_map::Entry, HashMap, HashSet};

use crate::{problem, Solution};

type Point = aoc_lib::Point<isize>;

pub struct Day23;

impl Solution for Day23 {
    fn name(&self) -> &'static str {
        "Unstable Diffusion"
    }

    fn part_a(&self) -> String {
        let raw = problem::load(2022, 23);
        let mut world = World::parse(&raw);

        world.draw();
        for _ in 0..10 {
            world.tick();
            world.draw();
        }

        world.count_blank().to_string()
    }

    fn part_b(&self) -> String {
        let _raw = problem::load(2022, 23);
        todo!()
    }
}

#[derive(Debug)]
struct World {
    elves: HashSet<Point>,
    iter: usize,
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

        Self { elves, iter: 0 }
    }

    fn draw(&self) {
        let (min, max) = self._bounds();

        println!("\nITER: {}", self.iter);
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

    fn tick(&mut self) -> bool {
        let mut next = HashSet::new();
        let mut next_to_cur = HashMap::new();
        let mut contested = HashSet::new();

        for pt in &self.elves {
            let Some(next_pt) = self._dir_clear_2(pt) else {
                next.insert(*pt);
                continue;
            };
            match next_to_cur.entry(next_pt) {
                Entry::Occupied(_) => {
                    contested.insert(next_pt);
                    next.insert(*pt);
                }
                Entry::Vacant(entry) => {
                    entry.insert(*pt);
                }
            }
        }

        let moved = self.elves.len() - next.len() - contested.len();
        if moved == 0 {
            return false;
        }

        for (to, from) in next_to_cur.into_iter() {
            next.insert(match contested.contains(&to) {
                true => from,
                false => to,
            });
        }

        self.elves = next;
        self.iter += 1;
        true
    }

    fn count_blank(&self) -> usize {
        let (min, max) = self._bounds();
        let mut ground = 0;

        for y in min.y..=max.y {
            for x in min.x..=max.x {
                if self.elves.contains(&Point::new(x, y)) {
                    continue;
                }
                ground += 1;
            }
        }

        ground
    }

    fn _dir_clear(&self, pos: &Point) -> Option<Point> {
        'o: for i in 0..4 {
            let points = match (self.iter + i) % 4 {
                0 => [Point::new(0, -1), Point::new(-1, -1), Point::new(1, -1)],
                1 => [Point::new(0, 1), Point::new(-1, 1), Point::new(1, 1)],
                2 => [Point::new(1, 0), Point::new(1, -1), Point::new(1, 1)],
                3 => [Point::new(-1, 0), Point::new(-1, -1), Point::new(-1, 1)],
                _ => unreachable!(),
            };

            for i in points {
                if self.elves.contains(&(*pos + i)) {
                    continue 'o;
                }
            }

            return Some(*pos + points[0]);
        }

        None
    }

    fn _dir_clear_2(&self, pos: &Point) -> Option<Point> {
        const LOOKUP: [[Point; 3]; 4] = [
            [Point::new(0, -1), Point::new(-1, -1), Point::new(1, -1)],
            [Point::new(0, 1), Point::new(-1, 1), Point::new(1, 1)],
            [Point::new(1, 0), Point::new(1, -1), Point::new(1, 1)],
            [Point::new(-1, 0), Point::new(-1, -1), Point::new(-1, 1)],
        ];

        let candidates: Vec<_> = (0..4)
            .filter_map(|j| {
                let lookup = LOOKUP[(self.iter + j) % LOOKUP.len()];
                lookup
                    .iter()
                    .all(|dir| !self.elves.contains(&(*pos + *dir)))
                    .then_some(*pos + lookup[0])
            })
            .collect();

        if candidates.is_empty() || candidates.len() == 4 {
            None
        } else {
            Some(candidates[0])
        }
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
