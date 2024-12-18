use std::collections::BinaryHeap;

use hashbrown::HashMap;
use nd_vec::{vector, Vector};

use aoc_lib::{direction::cardinal::Direction, matrix::Grid};
use common::{solution, Answer};

solution!("Chiton", 15);

type Point = Vector<usize, 2>;

fn part_a(input: &str) -> Answer {
    let matrix = Grid::parse(input, |chr| chr.to_digit(10).unwrap() as u8);
    solve(matrix.size, |pos| matrix.get(pos).copied()).into()
}

fn part_b(input: &str) -> Answer {
    let matrix = Grid::parse(input, |chr| chr.to_digit(10).unwrap() as u8);
    solve(matrix.size * 5, |pos| {
        let (cx, cy) = (pos.x() / matrix.size.x(), pos.y() / matrix.size.y());
        if cx > 4 || cy > 4 {
            return None;
        };

        let pos = vector!(pos.x() % matrix.size.x(), pos.y() % matrix.size.y());
        matrix
            .get(pos)
            .map(|x| (x + cx as u8 + cy as u8 - 1) % 9 + 1)
    })
    .into()
}

fn solve(size: Point, get: impl Fn(Point) -> Option<u8>) -> u32 {
    let mut seen = HashMap::new();
    let mut queue = BinaryHeap::new();

    seen.insert(vector!(0, 0), 0);
    queue.push(QueueItem {
        pos: vector!(0, 0),
        distance: 0,
    });

    while let Some(item) = queue.pop() {
        if item.pos == size - vector!(1, 1) {
            return item.distance;
        }

        for dir in Direction::ALL {
            let Some((pos, cost)) = dir.try_advance(item.pos).and_then(|x| Some((x, get(x)?)))
            else {
                continue;
            };

            let dist = seen.entry(pos).or_insert(u32::MAX);
            let next_dist = item.distance + cost as u32;

            if next_dist < *dist {
                *dist = next_dist;
                queue.push(QueueItem {
                    pos,
                    distance: next_dist,
                });
            }
        }
    }

    unreachable!()
}

#[derive(PartialEq, Eq)]
struct QueueItem {
    pos: Point,
    distance: u32,
}

impl Ord for QueueItem {
    fn cmp(&self, other: &Self) -> std::cmp::Ordering {
        other.distance.cmp(&self.distance)
    }
}

impl PartialOrd for QueueItem {
    fn partial_cmp(&self, other: &Self) -> Option<std::cmp::Ordering> {
        Some(self.cmp(other))
    }
}

#[cfg(test)]
mod test {
    use indoc::indoc;

    const CASE: &str = indoc! {"
        1163751742
        1381373672
        2136511328
        3694931569
        7463417111
        1319128137
        1359912421
        3125421639
        1293138521
        2311944581
    "};

    #[test]
    fn part_a() {
        assert_eq!(super::part_a(CASE), 40.into());
    }

    #[test]
    fn part_b() {
        assert_eq!(super::part_b(CASE), 315.into());
    }
}
