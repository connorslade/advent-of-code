use std::{collections::HashSet, convert::identity};

use common::{solution, Answer};
use nd_vec::{vector, Vec3};

solution!("Sand Slabs", 22);

fn part_a(input: &str) -> Answer {
    solve(parse(input), false, |x| (x == 0) as u32).into()
}

fn part_b(input: &str) -> Answer {
    solve(parse(input), true, identity).into()
}

fn solve(mut map: Vec<Box>, exhaustive: bool, count: fn(u32) -> u32) -> u32 {
    // Shift all boxes down as far as possible
    while shift_down(&mut map, false) != 0 {}

    // For each box, remove it and shift all other boxes down as far as possible
    let mut out = 0;
    for i in 0..map.len() {
        let mut map_clone = map.clone();
        map_clone.remove(i);
        out += count(shift_down(&mut map_clone, exhaustive));
    }

    out
}

fn shift_down(map: &mut [Box], exhaustive: bool) -> u32 {
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
                            .take(i)
                            .rev()
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
            let a = s.split(',').collect::<Vec<_>>();
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

    out.sort_unstable_by_key(|b| b.a.z());
    out
}

#[cfg(test)]
mod test {
    use indoc::indoc;

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
        assert_eq!(super::part_a(CASE), 5.into());
    }

    #[test]
    fn part_b() {
        assert_eq!(super::part_b(CASE), 7.into());
    }
}
