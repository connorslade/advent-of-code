use std::collections::HashSet;

use common::{Answer, Solution};
use nd_vec::vector;

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

        let size = map.size.num_cast::<i32>().unwrap();
        let (sx, sy) = (size.x(), size.y());

        let mut pos = HashSet::new();
        pos.insert(map.find(Tile::Start).unwrap().num_cast::<i32>().unwrap());

        for i in 0..500 {
            println!("{:.1}% done", i as f32 / 500.0 * 100.0);

            let mut new_pos = HashSet::new();

            for p in pos {
                for dir in Direction::ALL {
                    let new_p = dir.advance(p);
                    if {
                        let mut pos = new_p;

                        if pos.x() < 0 {
                            pos = vector!(sx + pos.x() % sx, pos.y());
                        }

                        if pos.y() < 0 {
                            pos = vector!(pos.x(), sy + pos.y() % sy);
                        }

                        pos = vector!(pos.x() % sx, pos.y());
                        pos = vector!(pos.x(), pos.y() % sy);

                        let pos = pos.num_cast::<usize>().unwrap();
                        *map.get(pos).unwrap()
                    } != Tile::Wall
                    {
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
