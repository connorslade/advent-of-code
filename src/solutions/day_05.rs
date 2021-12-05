use crate::common::{self, Solution};

use std::collections::HashMap;

pub struct Day05 {}

impl Solution for Day05 {
    fn name(&self) -> String {
        "Hydrothermal Venture".to_owned()
    }

    fn part_a(&self) -> String {
        run(false)
    }

    fn part_b(&self) -> String {
        run(true)
    }
}

fn run(dig: bool) -> String {
    let data = Segment::parse_inp(common::load("05"), dig).unwrap();
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

    all_loc.iter().filter(|x| x.1 >= &2).count().to_string()
}

#[derive(Debug, PartialEq, Eq, Clone, Copy)]
struct Segment {
    x1: u32,
    y1: u32,

    x2: u32,
    y2: u32,
}

impl Segment {
    fn parse_inp(inp: String, dig: bool) -> Option<Vec<Segment>> {
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
        let mut x = self.x1;
        let mut y = self.y1;

        let mut out = vec![(x, y)];

        while x != self.x2 || y != self.y2 {
            if x > self.x2 {
                x -= 1;
            }

            if x < self.x2 {
                x += 1;
            }

            if y > self.y2 {
                y -= 1;
            }

            if y < self.y2 {
                y += 1;
            }

            out.push((x, y));
        }

        out
    }
}
