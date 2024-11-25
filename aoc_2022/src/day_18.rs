use hashbrown::HashSet;

use common::{solution, Answer};
use nd_vec::{vector, Vector};

solution!("Boiling Boulders", 18);

type Pos = nd_vec::Vec3<i32>;

fn part_a(input: &str) -> Answer {
    let world = World::parse(input);

    let mut open_faces = 0;

    for i in &world.points {
        open_faces += 6 - world.neighbors(i);
    }

    open_faces.into()
}

fn part_b(input: &str) -> Answer {
    let world = World::parse(input);

    let outside = world.flood_fill(Vector::default());
    let mut out = 0;
    for i in &world.points {
        for j in NEIGHBORS {
            let n = *i + j;
            if !world.points.contains(&n) && outside.contains(&n) {
                out += 1;
            }
        }
    }

    out.into()
}

struct World {
    points: HashSet<Pos>,
}

const NEIGHBORS: [Pos; 6] = [
    vector!(1, 0, 0),
    vector!(-1, 0, 0),
    vector!(0, 1, 0),
    vector!(0, -1, 0),
    vector!(0, 0, 1),
    vector!(0, 0, -1),
];

impl World {
    fn parse(raw: &str) -> Self {
        Self {
            points: HashSet::from_iter(parse(raw)),
        }
    }

    fn neighbors(&self, point: &Pos) -> usize {
        let mut out = 0;

        for i in NEIGHBORS {
            out += self.points.contains(&(*point + i)) as usize;
        }

        out
    }

    fn bounds(&self) -> (Pos, Pos) {
        let mut min = vector!(i32::MAX, i32::MAX, i32::MAX);
        let mut max = vector!(i32::MIN, i32::MIN, i32::MIN);

        for i in &self.points {
            min = min.min(i);
            max = max.max(i);
        }

        (min, max)
    }

    fn flood_fill(&self, start: Pos) -> HashSet<Pos> {
        let bounds = self.bounds();
        let mut steam = HashSet::new();
        let mut new = vec![start];

        while let Some(s) = new.pop() {
            steam.insert(s);
            for n in NEIGHBORS {
                let n = s + n;
                if n.x() > bounds.1.x() + 1
                    || n.x() < bounds.0.x() - 1
                    || n.y() > bounds.1.y() + 1
                    || n.y() < bounds.0.y() - 1
                    || n.z() > bounds.1.z() + 1
                    || n.z() < bounds.0.z() - 1
                    || self.points.contains(&n)
                    || steam.contains(&n)
                    || new.contains(&n)
                {
                    continue;
                }

                new.push(n);
            }
        }

        steam
    }
}

fn parse(raw: &str) -> Vec<Pos> {
    let mut out = Vec::new();

    for i in raw.lines() {
        let mut parts = i.split(',');
        out.push(vector!(
            parts.next().unwrap().parse().unwrap(),
            parts.next().unwrap().parse().unwrap(),
            parts.next().unwrap().parse().unwrap()
        ));
    }

    out
}
