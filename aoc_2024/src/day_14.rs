use common::{solution, Answer};
use nd_vec::{vector, Vec2};

solution!("Restroom Redoubt", 14);

// Part A was easy enough, just implement the logic and run it for 100 ticks. To
// be more efferent, instead of adding the velocity to the position of each
// robot on each tick, you can just add vel * ticks then basically modulo to put
// it back in bounds.
fn part_a(input: &str) -> Answer {
    let mut problem = Problem::parse(input);
    problem.tick(100);
    problem.score().into()
}

// When I read todays part B, I was just so confused for a while. To find boards
// that were likely showing some pattern, I just sum the distances from each
// robot to the board center, when this drops below a set threshold, I assume
// that is the tree. You can uncomment the .debug() call to actually see the
// tree.
fn part_b(input: &str) -> Answer {
    let mut problem = Problem::parse(input);

    for i in 0.. {
        problem.tick(1);
        if problem.total_distance() < 20_000 {
            // problem.debug();
            return (i + 1).into();
        }
    }

    unreachable!()
}

struct Problem {
    bounds: Vec2<i32>,
    robots: Vec<Robot>,
}

struct Robot {
    pos: Vec2<i32>,
    vel: Vec2<i32>,
}

impl Problem {
    fn parse(input: &str) -> Self {
        let robots = input.lines().map(Robot::parse).collect::<Vec<_>>();

        let mut bounds = vector!(0, 0);
        robots.iter().for_each(|robot| {
            bounds = vector!(robot.pos.x().max(bounds.x()), robot.pos.y().max(bounds.y()))
        });

        Self {
            robots,
            bounds: bounds + vector!(1, 1),
        }
    }

    fn tick(&mut self, n: i32) {
        self.robots.iter_mut().for_each(|x| x.tick(self.bounds, n));
    }

    fn total_distance(&self) -> i32 {
        let middle = self.bounds / 2;
        self.robots
            .iter()
            .map(|x| x.pos.manhattan_distance(&middle))
            .sum()
    }

    #[allow(unused)]
    fn debug(&self) {
        for y in 0..self.bounds.y() {
            for x in 0..self.bounds.x() {
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

        let mut quadrants = [0; 4];
        for pos in self.robots.iter().map(|x| x.pos) {
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

    fn tick(&mut self, bounds: Vec2<i32>, n: i32) {
        self.pos += self.vel * n;
        self.pos = vector!(
            (self.pos.x() % bounds.x() + bounds.x()) % bounds.x(),
            (self.pos.y() % bounds.y() + bounds.y()) % bounds.y()
        );
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
