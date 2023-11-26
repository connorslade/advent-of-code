use common::{Answer, Solution};

use hashbrown::HashMap;

pub struct Day05;

impl Solution for Day05 {
    fn name(&self) -> &'static str {
        "Hydrothermal Venture"
    }

    fn part_a(&self, input: &str) -> Answer {
        run(input, false).into()
    }

    fn part_b(&self, input: &str) -> Answer {
        run(input, true).into()
    }
}

/// dig -> Weather to include Diagonal Lines
fn run(input: &str, dig: bool) -> u32 {
    let data = Segment::parse_inp(input, dig).unwrap();
    let mut all_loc = HashMap::new();

    for x in data {
        let dump = x.dump();
        for y in dump {
            if all_loc.contains_key(&y) {
                *all_loc.get_mut(&y).unwrap() += 1;
                continue;
            }
            all_loc.insert(y, 1);
        }
    }

    all_loc.iter().filter(|x| *x.1 >= 2).count() as u32
}

#[derive(Debug, PartialEq, Eq, Clone, Copy)]
struct Segment {
    x1: u32,
    y1: u32,

    x2: u32,
    y2: u32,
}

impl Segment {
    fn parse_inp(inp: &str, dig: bool) -> Option<Vec<Segment>> {
        let mut out = Vec::new();

        for line in inp.lines() {
            let mut parts = line.split(" -> ");
            let mut one = parts.next()?.split(',');
            let mut two = parts.next()?.split(',');

            let x1 = one.next()?.parse().ok()?;
            let y1 = one.next()?.parse().ok()?;
            let x2 = two.next()?.parse().ok()?;
            let y2 = two.next()?.parse().ok()?;

            if x1 != x2 && y1 != y2 && !dig {
                continue;
            }

            out.push(Segment { x1, y1, x2, y2 })
        }

        Some(out)
    }

    fn dump(&self) -> Vec<(u32, u32)> {
        let mut out = vec![(self.x1, self.y1)];
        let mut x = self.x1;
        let mut y = self.y1;

        while x != self.x2 || y != self.y2 {
            x -= (x > self.x2) as u32;
            x += (x < self.x2) as u32;
            y -= (y > self.y2) as u32;
            y += (y < self.y2) as u32;
            out.push((x, y));
        }

        out
    }
}
