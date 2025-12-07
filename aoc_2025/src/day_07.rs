use std::{
    collections::{HashMap, HashSet},
    convert::identity,
};

use aoc_lib::{direction::cardinal::Direction, matrix::Grid};
use common::{Answer, solution};
use nd_vec::vector;

solution!("Laboratories", 7);

fn part_a(input: &str) -> Answer {
    let grid = Grid::parse(input, identity);
    let start = grid.find('S').unwrap();

    let mut beams = HashSet::new();
    beams.insert(start);

    let mut out = 0;

    loop {
        let mut next_beams = HashSet::new();
        for beam in beams.iter() {
            let next = Direction::Down.advance(*beam);
            let Some(tile) = grid.get(next) else {
                continue;
            };

            if *tile == '^' {
                out += 1;
                next_beams.insert(next - vector!(1, 0));
                next_beams.insert(next + vector!(1, 0));
            } else {
                next_beams.insert(next);
            }
        }

        if next_beams.is_empty() {
            break;
        }

        beams = next_beams;
    }

    out.into()
}

fn part_b(input: &str) -> Answer {
    let grid = Grid::parse(input, identity);
    let start = grid.find('S').unwrap();

    let mut beams = HashMap::new();
    beams.insert(start, 1_u64);

    // let mut out = 0;

    loop {
        let mut next_beams = HashMap::new();
        for (beam, count) in beams.iter() {
            let next = Direction::Down.advance(*beam);
            let Some(tile) = grid.get(next) else {
                continue;
            };

            if *tile == '^' {
                *next_beams.entry(next - vector!(1, 0)).or_default() += *count;
                *next_beams.entry(next + vector!(1, 0)).or_default() += *count;
            } else {
                *next_beams.entry(next).or_default() += *count;
            }
        }

        println!("count : {}", next_beams.len());
        if next_beams.is_empty() {
            break;
        }

        beams = next_beams;
    }

    beams.iter().map(|x| x.1).sum::<u64>().into()
}

#[cfg(test)]
mod test {
    use indoc::indoc;

    const CASE: &str = indoc! {"
        .......S.......
        ...............
        .......^.......
        ...............
        ......^.^......
        ...............
        .....^.^.^.....
        ...............
        ....^.^...^....
        ...............
        ...^.^...^.^...
        ...............
        ..^...^.....^..
        ...............
        .^.^.^.^.^...^.
        ...............
    "};

    #[test]
    fn part_a() {
        assert_eq!(super::part_a(CASE), 21.into());
    }

    #[test]
    fn part_b() {
        assert_eq!(super::part_b(CASE), 40.into());
    }
}
