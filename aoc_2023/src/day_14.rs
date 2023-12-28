use std::{collections::HashMap, convert::identity, hash::Hash};

use aoc_lib::matrix::Matrix;
use common::{Answer, Solution};
use nd_vec::{vector, Vec2};

type Pos = Vec2<isize>;

pub struct Day14;

impl Solution for Day14 {
    fn name(&self) -> &'static str {
        "Parabolic Reflector Dish"
    }

    fn part_a(&self, input: &str) -> Answer {
        let mut dish = parse(input);
        dish.tilt(vector!(0, -1));
        dish.score().into()
    }

    fn part_b(&self, input: &str) -> Answer {
        let mut dish = parse(input);

        const ITERS: usize = 1000000000;
        let mut seen = HashMap::new();
        for i in 0..ITERS {
            if let Some(prev) = seen.get(&dish) {
                if (ITERS - i) % (i - prev) == 0 {
                    return dish.score().into();
                }
            }

            seen.insert(dish.clone(), i);
            dish.spin();
        }

        dish.score().into()
    }
}

#[derive(Debug, Clone, PartialEq, Eq, Hash)]
struct Dish {
    tiles: Matrix<char>,
}

fn parse(input: &str) -> Dish {
    Dish {
        tiles: Matrix::new_chars(input, identity),
    }
}

impl Dish {
    fn tilt(&mut self, tilt: Pos) {
        let tiles = &mut self.tiles;
        loop {
            let mut moved = false;
            for y in 0..tiles.size.y() {
                for x in 0..tiles.size.x() {
                    let pos = vector!(x as isize, y as isize);
                    if tiles[pos] != 'O' {
                        continue;
                    }

                    let new_pos = vector!(x, y).num_cast().unwrap() + tilt;
                    if !tiles.contains(new_pos) || tiles[new_pos] != '.' {
                        continue;
                    }

                    let tile = tiles[pos];
                    tiles.set(pos, '.');
                    tiles.set(new_pos, tile);
                    moved = true;
                }
            }

            if !moved {
                break;
            }
        }
    }

    fn spin(&mut self) {
        for pos in [vector!(0, -1), vector!(-1, 0), vector!(0, 1), vector!(1, 0)] {
            self.tilt(pos);
        }
    }

    fn score(&self) -> usize {
        let tiles = &self.tiles;
        let mut acc = 0;

        for y in 0..tiles.size.y() {
            for x in 0..tiles.size.x() {
                if tiles[[x, y]] == 'O' {
                    acc += tiles.size.y() - y;
                }
            }
        }

        acc
    }
}

#[cfg(test)]
mod test {
    use common::Solution;
    use indoc::indoc;

    use super::Day14;

    const CASE: &str = indoc! {"
        O....#....
        O.OO#....#
        .....##...
        OO.#O....O
        .O.....O#.
        O.#..O.#.#
        ..O..#O..O
        .......O..
        #....###..
        #OO..#....
    "};

    #[test]
    fn part_a() {
        assert_eq!(Day14.part_a(CASE), 136.into());
    }

    #[test]
    fn part_b() {
        assert_eq!(Day14.part_b(CASE), 64.into());
    }
}
