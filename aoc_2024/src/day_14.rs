use std::io::stdin;

use common::{solution, Answer};
use itertools::Itertools;
use nd_vec::{vector, Vec2};

solution!("Restroom Redoubt", 14);

fn part_a(input: &str) -> Answer {
    let mut problem = Problem::parse(input);

    // println!();
    for _ in 0..100 {
        problem.tick();
    }
    // problem.debug();
    problem.score().into()
}

fn part_b(input: &str) -> Answer {
    let mut problem = Problem::parse(input);

    // println!();
    for i in 0..10_000 {
        problem.tick();
        // println!("{} v", i + 1);
        // problem.debug();
        // println!();
        // println!("{}", problem.total_distance());

        if problem.total_distance() < 6_000_000 {
            println!("{}", problem.total_distance());
            println!("{} v", i + 1);
            problem.debug();
            stdin().read_line(&mut String::new());
        }
    }
    // problem.debug();
    problem.score().into()
}

// rename
#[derive(Debug)]
struct Problem {
    bounds: Vec2<i32>,
    robots: Vec<Robot>,
}

#[derive(Debug)]
struct Robot {
    pos: Vec2<i32>,
    vel: Vec2<i32>,
}

impl Problem {
    fn parse(input: &str) -> Self {
        let robots = input.lines().map(|x| Robot::parse(x)).collect::<Vec<_>>();

        let mut bounds = vector!(0, 0);
        for robot in robots.iter() {
            bounds = vector!(robot.pos.x().max(bounds.x()), robot.pos.y().max(bounds.y()));
        }
        bounds += vector!(1, 1);

        Self { robots, bounds }
    }

    fn tick(&mut self) {
        self.robots.iter_mut().for_each(|x| x.tick(self.bounds));
    }

    fn total_distance(&self) -> i32 {
        let mut sum = 0;
        for (a, b) in self.robots.iter().tuple_combinations() {
            sum += a.pos.manhattan_distance(&b.pos);
        }
        sum
    }

    fn debug(&self) {
        let half_bounds = self.bounds / 2;

        for y in (0..self.bounds.y()) {
            for x in (0..self.bounds.x()) {
                let robots = self
                    .robots
                    .iter()
                    .filter(|r| r.pos == vector!(x, y))
                    .count();

                if robots == 0 {
                    print!(".");
                } else {
                    print!("{robots}");
                }
            }
            println!();
        }
    }

    fn score(&self) -> u32 {
        let half_bounds = self.bounds / 2;

        // 0 1 2 3 4

        let mut quadrants = [0; 4];
        for robot in self.robots.iter() {
            let pos = robot.pos;

            if pos.x() == half_bounds.x() || pos.y() == half_bounds.y() {
                continue;
            }

            let width = (0..=half_bounds.x()).contains(&pos.x());
            let height = (0..=half_bounds.y()).contains(&pos.y());

            quadrants[((width as usize) << 1) | height as usize] += 1;
        }

        quadrants.iter().product()
    }
}

impl Robot {
    fn parse(input: &str) -> Self {
        let (p, v) = input.split_once(" ").unwrap();
        let parse_coord = |string: &str| {
            let (x, y) = string[2..].split_once(',').unwrap();
            vector!(x.parse().unwrap(), y.parse().unwrap())
        };

        Self {
            pos: parse_coord(p),
            vel: parse_coord(v),
        }
    }

    fn tick(&mut self, bounds: Vec2<i32>) {
        self.pos += self.vel;

        while self.pos.x() < 0 {
            self.pos += vector!(bounds.x(), 0);
        }

        while self.pos.x() >= bounds.x() {
            self.pos -= vector!(bounds.x(), 0);
        }

        while self.pos.y() < 0 {
            self.pos += vector!(0, bounds.y());
        }

        while self.pos.y() >= bounds.y() {
            self.pos -= vector!(0, bounds.y());
        }
    }
}

#[cfg(test)]
mod test {
    use indoc::indoc;

    const CASE: &str = indoc! {"
        p=0,4 v=3,-3
        p=6,3 v=-1,-3
        p=10,3 v=-1,2
        p=2,0 v=2,-1
        p=0,0 v=1,3
        p=3,0 v=-2,-2
        p=7,6 v=-1,-3
        p=3,0 v=-1,-2
        p=9,3 v=2,3
        p=7,3 v=-1,2
        p=2,4 v=2,-3
        p=9,5 v=-3,-3
    "};

    #[test]
    fn part_a() {
        assert_eq!(super::part_a(CASE), 12.into());
    }
}
