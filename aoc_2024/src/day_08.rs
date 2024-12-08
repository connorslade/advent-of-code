use std::collections::{HashMap, HashSet};

use aoc_lib::matrix::Matrix;
use common::{solution, Answer};
use nd_vec::{vector, Vec2};

use itertools::Itertools;

solution!("Resonant Collinearity", 8);

#[derive(Clone)]
enum Tile {
    Emitter(char),
    Empty,
}

fn part_a(input: &str) -> Answer {
    let world = Matrix::new_chars(input, |x| match x {
        'a'..='z' | 'A'..='Z' | '0'..='9' => Tile::Emitter(x),
        _ => Tile::Empty,
    });

    let mut freqs = HashMap::<char, Vec<Vec2<i32>>>::new();

    // oh god just make a Matrix.iter function at this point...
    for x in 0..world.size.x() {
        for y in 0..world.size.y() {
            let pos = vector!(x, y);
            if let Tile::Emitter(chr) = world.get(pos).unwrap() {
                freqs
                    .entry(*chr)
                    .or_default()
                    .push(pos.try_cast::<i32>().unwrap());
            }
        }
    }

    // nodes are on the line between two of the same freq emitters when d1 = 2*d2
    // to find nodes, start by finding all lines through matching emitters

    let in_bounds = |pos: Vec2<i32>| {
        pos.x() >= 0
            && pos.y() >= 0
            && pos.x() < world.size.x() as i32
            && pos.y() < world.size.y() as i32
    };

    let mut out = HashSet::new();
    for (_freq, pos) in freqs {
        for (a, b) in pos.into_iter().tuple_combinations() {
            let diff_a = a + (a - b);
            let diff_b = b + (b - a);

            if in_bounds(diff_a) {
                out.insert(diff_a);
            }

            if in_bounds(diff_b) {
                out.insert(diff_b);
            }
        }
    }

    out.len().into()
}

fn part_b(input: &str) -> Answer {
    let world = Matrix::new_chars(input, |x| match x {
        'a'..='z' | 'A'..='Z' | '0'..='9' => Tile::Emitter(x),
        _ => Tile::Empty,
    });

    let mut freqs = HashMap::<char, Vec<Vec2<i32>>>::new();

    // oh god just make a Matrix.iter function at this point...
    for x in 0..world.size.x() {
        for y in 0..world.size.y() {
            let pos = vector!(x, y);
            if let Tile::Emitter(chr) = world.get(pos).unwrap() {
                freqs
                    .entry(*chr)
                    .or_default()
                    .push(pos.try_cast::<i32>().unwrap());
            }
        }
    }

    // nodes are on the line between two of the same freq emitters when d1 = 2*d2
    // to find nodes, start by finding all lines through matching emitters

    let in_bounds = |pos: Vec2<i32>| {
        pos.x() >= 0
            && pos.y() >= 0
            && pos.x() < world.size.x() as i32
            && pos.y() < world.size.y() as i32
    };

    let mut out = HashSet::new();
    for (_freq, pos) in freqs {
        for (a, b) in pos.into_iter().tuple_combinations() {
            let diff_a = a - b;
            let diff_b = b - a;

            let mut pos_a = a;
            let mut pos_b = b;

            while in_bounds(pos_a) {
                out.insert(pos_a);
                pos_a += diff_a;
            }

            while in_bounds(pos_b) {
                out.insert(pos_b);
                pos_b += diff_b;
            }
        }
    }

    out.len().into()
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
