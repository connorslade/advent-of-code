use std::collections::HashSet;

use common::{Answer, Solution};
use nd_vec::{vector, Vec2};

use crate::aoc_lib::{direction::Direction, matrix::Matrix};

pub struct Day21;

impl Solution for Day21 {
    fn name(&self) -> &'static str {
        "Step Counter"
    }

    fn part_a(&self, input: &str) -> Answer {
        let map = parse(input);
        let mut pos = HashSet::new();
        pos.insert(map.find(Tile::Start).unwrap().num_cast::<i32>().unwrap());

        for _ in 0..64 {
            let mut new_pos = HashSet::new();

            for p in pos {
                for dir in Direction::ALL {
                    let new_p = dir.advance(p);
                    if !map.contains(new_p)
                        || *map.get(new_p.num_cast().unwrap()).unwrap() != Tile::Wall
                    {
                        new_pos.insert(new_p);
                    }
                }
            }

            pos = new_pos;
        }

        pos.len().into()
    }

    fn part_b(&self, input: &str) -> Answer {
        let map = parse(input);
        let start = map.find(Tile::Start).unwrap().num_cast::<i32>().unwrap();
        let size = map.size.num_cast::<i32>().unwrap();

        let mut pos = HashSet::new();
        pos.insert(start);

        for i in 0..100 {
            let mut new_pos = HashSet::new();

            if i % size.x() == 65 {
                println!("({i}, {})", pos.len());
                // Pipe the first few values into wolfram alpha to get a formula then evaluate it with `26501365`
                // You may have to bump the number of iterations from 100 to get enough data points
                // Ex: curve fit (65, 3797), (196, 34009), (327, 94353)
            }

            for p in pos {
                for dir in Direction::ALL {
                    let new_p = dir.advance(p);
                    let mapped = map_pos(new_p, size);
                    if *map.get(mapped).unwrap() != Tile::Wall {
                        new_pos.insert(new_p);
                    }
                }
            }

            pos = new_pos;
        }

        // pos.len().into()

        Answer::Unimplemented
    }
}

fn map_pos(pos: Vec2<i32>, size: Vec2<i32>) -> Vec2<usize> {
    let mut mapped = pos;
    mapped = vector!((size.x() + mapped.x() % size.x()) % size.x(), mapped.y());
    mapped = vector!(mapped.x(), (size.y() + mapped.y() % size.y()) % size.y());
    mapped.num_cast().unwrap()
}

#[derive(Debug, PartialEq, Eq, Copy, Clone)]
enum Tile {
    Garden,
    Wall,
    Start,
}

fn parse(input: &str) -> Matrix<Tile> {
    Matrix::new_chars(input, |x| match x {
        '#' => Tile::Wall,
        '.' => Tile::Garden,
        'S' => Tile::Start,
        _ => panic!("Invalid input"),
    })
}

#[cfg(test)]
mod test {
    use common::Solution;
    use indoc::indoc;

    use super::Day21;

    const CASE: &str = indoc! {"
        ...........
        .....###.#.
        .###.##..#.
        ..#.#...#..
        ....#.#....
        .##..S####.
        .##..#...#.
        .......##..
        .##.#.####.
        .##..##.##.
        ...........
    "};

    #[test]
    fn part_a() {
        assert_eq!(Day21.part_a(CASE), 4056.into());
    }

    #[test]
    fn part_b() {
        assert_eq!(Day21.part_b(CASE), ().into());
    }
}
