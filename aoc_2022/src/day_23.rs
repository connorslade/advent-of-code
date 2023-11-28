use hashbrown::{hash_map::Entry, HashMap, HashSet};

use crate::aoc_lib;
use common::{Answer, Solution};

type Point = aoc_lib::Point<isize>;

pub struct Day23;

impl Solution for Day23 {
    fn name(&self) -> &'static str {
        "Unstable Diffusion"
    }

    fn part_a(&self, input: &str) -> Answer {
        let mut world = World::parse(input);

        for _ in 0..10 {
            world.tick();
        }

        world.count_blank().into()
    }

    fn part_b(&self, input: &str) -> Answer {
        let mut world = World::parse(input);
        let mut iters = 1;

        while world.tick() {
            iters += 1;
        }

        iters.into()
    }
}

// [North, South, West, East]
const LOOKUP: [[Point; 3]; 4] = [
    [Point::new(0, -1), Point::new(-1, -1), Point::new(1, -1)],
    [Point::new(0, 1), Point::new(-1, 1), Point::new(1, 1)],
    [Point::new(-1, 0), Point::new(-1, -1), Point::new(-1, 1)],
    [Point::new(1, 0), Point::new(1, -1), Point::new(1, 1)],
];

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
        let (min, max) = self.bounds();

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

        for elf in &self.elves {
            let Some(next_pt) = self.next_move(elf) else {
                next.insert(*elf);
                continue;
            };
            match next_to_cur.entry(next_pt) {
                Entry::Occupied(_) => {
                    contested.insert(next_pt);
                    next.insert(*elf);
                }
                Entry::Vacant(entry) => {
                    entry.insert(*elf);
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
        let (min, max) = self.bounds();
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

    fn next_move(&self, pos: &Point) -> Option<Point> {
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

    fn bounds(&self) -> (Point, Point) {
        let mut min = Point::new(isize::MAX, isize::MAX);
        let mut max = Point::new(isize::MIN, isize::MIN);

        for i in self.elves.iter() {
            min = min.min(i);
            max = max.max(i);
        }

        (min, max)
    }
}

#[cfg(test)]
mod test {
    use common::Solution;
    use indoc::indoc;

    use super::{Day23, World};

    const CASE: &str = indoc! {"
        ..............
        ..............
        .......#......
        .....###.#....
        ...#...#.#....
        ....#...##....
        ...#.###......
        ...##.#.##....
        ....#..#......
        ..............
        ..............
        ..............
    "};

    #[test]
    fn simulate() {
        const CASE: &str = indoc! {"
            .....
            ..##.
            ..#..
            .....
            ..##.
            .....
        "};

        let mut world = World::parse(CASE);
        while world.tick() {}

        assert_eq!(world.count_blank(), 25);
    }

    #[test]
    fn part_a() {
        assert_eq!(Day23.part_a(CASE), 110.into());
    }

    #[test]
    fn part_b() {
        assert_eq!(Day23.part_b(CASE), 20.into());
    }
}
