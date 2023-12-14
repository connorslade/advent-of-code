use std::{collections::HashMap, hash::Hash};

use nd_vec::{vector, Vec2};

use common::{Answer, Solution};

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
    tiles: Vec<Vec<char>>,
}

fn parse(input: &str) -> Dish {
    let times = input.lines().map(|line| line.chars().collect()).collect();
    Dish { tiles: times }
}

impl Dish {
    fn tilt(&mut self, pos: Pos) {
        loop {
            let mut moved = false;
            for y in 0..self.tiles.len() {
                for x in 0..self.tiles[y].len() {
                    if self.tiles[y][x] != 'O' {
                        continue;
                    }

                    let new_pos = vector!(x, y).num_cast().unwrap() + pos;
                    if new_pos.x() < 0
                        || new_pos.x() >= self.tiles[y].len() as isize
                        || new_pos.y() < 0
                        || new_pos.y() >= self.tiles.len() as isize
                    {
                        continue;
                    }

                    let [nx, ny] = {
                        let a = new_pos.num_cast::<usize>().unwrap();
                        [a.x(), a.y()]
                    };
                    if self.tiles[ny][nx] != '.' {
                        continue;
                    }

                    let tile = self.tiles[y][x];
                    self.tiles[y][x] = '.';
                    self.tiles[ny][nx] = tile;
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
        let mut acc = 0;

        for y in 0..self.tiles.len() {
            for x in 0..self.tiles[y].len() {
                if self.tiles[y][x] == 'O' {
                    acc += self.tiles.len() - y;
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
