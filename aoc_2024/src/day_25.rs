use std::convert::identity;

use aoc_lib::matrix::Grid;
use common::{solution, Answer};
use nd_vec::vector;

solution!("Code Chronicle", 25);

fn part_a(input: &str) -> Answer {
    let mut keys = Vec::new();
    let mut locks = Vec::new();

    for item in input.split("\n\n") {
        let (item, lock) = Key::parse(item);
        if lock {
            locks.push(item);
        } else {
            keys.push(item);
        }
    }

    let mut out = 0;

    for key in keys {
        for lock in locks.iter() {
            if key.fits(&lock) {
                out += 1;
            }
        }
    }

    out.into()
}

fn part_b(input: &str) -> Answer {
    Answer::Unimplemented
}

#[derive(Debug)]
struct Key {
    heights: [u8; 5],
}

impl Key {
    fn parse(input: &str) -> (Self, bool) {
        let grid = Grid::parse(input, identity);

        let mut heights = [0; 5];
        let mut is_lock = true;

        for x in 0..5 {
            let mut height = 0;
            for y in 0..7 {
                if y == 0 && grid[vector!(x, y)] != '#' {
                    is_lock = false;
                }

                if grid[vector!(x, y)] == '#' {
                    height += 1;
                }
            }

            heights[x] = height - 1;
        }

        (Self { heights }, is_lock)
    }

    fn fits(&self, other: &Self) -> bool {
        for (a, b) in self.heights.iter().zip(other.heights.iter()) {
            if a + b > 5 {
                return false;
            }
        }

        true
    }
}

#[cfg(test)]
mod test {
    use indoc::indoc;

    const CASE: &str = indoc! {"
        #####
        .####
        .####
        .####
        .#.#.
        .#...
        .....

        #####
        ##.##
        .#.##
        ...##
        ...#.
        ...#.
        .....

        .....
        #....
        #....
        #...#
        #.#.#
        #.###
        #####

        .....
        .....
        #.#..
        ###..
        ###.#
        ###.#
        #####

        .....
        .....
        .....
        #....
        #.#..
        #.#.#
        #####
    "};

    #[test]
    fn part_a() {
        assert_eq!(super::part_a(CASE), 3.into());
    }

    #[test]
    fn part_b() {
        assert_eq!(super::part_b(CASE), ().into());
    }
}
