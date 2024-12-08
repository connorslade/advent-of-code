use std::collections::{HashMap, HashSet};

use aoc_lib::matrix::Matrix;
use common::{solution, Answer};
use nd_vec::Vec2;

use itertools::Itertools;

solution!("Resonant Collinearity", 8);

fn part_a(input: &str) -> Answer {
    // Parse the input into a hash table that maps antenna frequencies to positions.
    let map = AntennaMap::parse(input);

    let mut out = HashSet::new();
    for (_freq, pos) in map.freqs {
        // Because a line is defined by two points, we find all point
        // combinations with two points. To get the two antinode points, we add
        // the difference between both points to the first point and subtract
        // the difference from the second.
        for (a, b) in pos.into_iter().tuple_combinations() {
            out.extend(
                [a + (a - b), b + (b - a)]
                    .into_iter()
                    .filter(|&x| map.world.contains(x)),
            );
        }
    }

    out.len().into()
}

fn part_b(input: &str) -> Answer {
    let map = AntennaMap::parse(input);

    let mut out = HashSet::new();
    for (_freq, pos) in map.freqs {
        for (a, b) in pos.into_iter().tuple_combinations() {
            // For part be we just need to keep adding / subtracting the
            // diffract between the two points to the starting position until
            // the result is out of bounds.
            for (mut start, delta) in [(a, a - b), (b, b - a)] {
                while map.world.contains(start) {
                    out.insert(start);
                    start += delta;
                }
            }
        }
    }

    out.len().into()
}

struct AntennaMap {
    world: Matrix<Tile>,
    freqs: HashMap<char, Vec<Vec2<i32>>>,
}

enum Tile {
    Emitter(char),
    Empty,
}

impl AntennaMap {
    fn parse(input: &str) -> Self {
        let world = Matrix::new_chars(input, |x| match x {
            'a'..='z' | 'A'..='Z' | '0'..='9' => Tile::Emitter(x),
            _ => Tile::Empty,
        });

        let mut freqs = HashMap::<char, Vec<Vec2<i32>>>::new();
        for (pos, tile) in world.iter() {
            if let Tile::Emitter(chr) = tile {
                freqs
                    .entry(*chr)
                    .or_default()
                    .push(pos.try_cast::<i32>().unwrap());
            }
        }

        Self { world, freqs }
    }
}

#[cfg(test)]
mod test {
    use indoc::indoc;

    const CASE: &str = indoc! {"
        ............
        ........0...
        .....0......
        .......0....
        ....0.......
        ......A.....
        ............
        ............
        ........A...
        .........A..
        ............
        ............
    "};

    #[test]
    fn part_a() {
        assert_eq!(super::part_a(CASE), 14.into());
    }

    #[test]
    fn part_b() {
        assert_eq!(super::part_b(CASE), 34.into());
    }
}
