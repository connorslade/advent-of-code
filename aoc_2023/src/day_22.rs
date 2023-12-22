use std::collections::HashSet;

use common::{Answer, Solution};
use nd_vec::{vector, Vec3};

pub struct Day22;

// This could be (and will be in the future) updated to make use of the fact that elements can be sorted by their z value
impl Solution for Day22 {
    fn name(&self) -> &'static str {
        "Sand Slabs"
    }

    fn part_a(&self, input: &str) -> Answer {
        let mut map = parse(input);
        while shift_down(&mut map, false) != 0 {}

        let mut count = 0;
        for i in 0..map.len() {
            let mut map_clone = map.clone();
            map_clone.remove(i);
            if shift_down(&mut map_clone, false) == 0 {
                count += 1;
            }
        }

        count.into()
    }

    fn part_b(&self, input: &str) -> Answer {
        let mut map = parse(input);
        while shift_down(&mut map, false) != 0 {}

        let mut count = 0;
        for i in 0..map.len() {
            let mut map_clone = map.clone();
            map_clone.remove(i);
            count += shift_down(&mut map_clone, true);
        }

        count.into()
    }
}

fn shift_down(map: &mut Vec<Box>, exhaustive: bool) -> u32 {
    let mut moved = HashSet::new();
    let mut dirty = true;
    while dirty {
        dirty = false;
        'outer: for i in 0..map.len() {
            // If no other box below this one, move it down
            let item = map[i];
            for x in item.a.x()..=item.b.x() {
                for y in item.a.y()..=item.b.y() {
                    if item.a.z() == 1
                        || map
                            .iter()
                            .any(|b| b.contains(vector!(x, y, item.a.z() - 1)))
                    {
                        continue 'outer;
                    }
                }
            }

            map[i].a -= vector!(0, 0, 1);
            map[i].b -= vector!(0, 0, 1);
            moved.insert(i);
            dirty = exhaustive;
        }
    }
    moved.len() as u32
}

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
struct Box {
    a: Vec3<u32>,
    b: Vec3<u32>,
}

impl Box {
    fn contains(&self, p: Vec3<u32>) -> bool {
        p.x() >= self.a.x()
            && p.x() <= self.b.x()
            && p.y() >= self.a.y()
            && p.y() <= self.b.y()
            && p.z() >= self.a.z()
            && p.z() <= self.b.z()
    }
}

fn parse(input: &str) -> Vec<Box> {
    let mut out = Vec::new();
    for line in input.lines() {
        let parse = |s: &str| {
            let a = s.split(",").collect::<Vec<_>>();
            vector!(
                a[0].parse().unwrap(),
                a[1].parse().unwrap(),
                a[2].parse().unwrap()
            )
        };
        let (a, b) = line.split_once('~').unwrap();
        let (a, b) = (parse(a), parse(b));
        out.push(Box {
            a: a.min(&b),
            b: a.max(&b),
        });
    }
    out
}

#[cfg(test)]
mod test {
    use common::Solution;
    use indoc::indoc;

    use super::Day22;

    const CASE: &str = indoc! {"
        1,0,1~1,2,1
        0,0,2~2,0,2
        0,2,3~2,2,3
        0,0,4~0,2,4
        2,0,5~2,2,5
        0,1,6~2,1,6
        1,1,8~1,1,9
    "};

    #[test]
    fn part_a() {
        assert_eq!(Day22.part_a(CASE), 5.into());
    }

    #[test]
    fn part_b() {
        assert_eq!(Day22.part_b(CASE), 7.into());
    }
}
