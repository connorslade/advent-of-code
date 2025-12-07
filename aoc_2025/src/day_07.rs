use std::{collections::HashMap, convert::identity};

use aoc_lib::{direction::cardinal::Direction, matrix::Grid};
use common::{Answer, solution};
use nd_vec::{Vec2, vector};

solution!("Laboratories", 7);

fn part_a(input: &str) -> Answer {
    solve(input).1.into()
}

fn part_b(input: &str) -> Answer {
    let (beams, _) = solve(input);
    beams.iter().map(|x| x.1).sum::<u64>().into()
}

fn solve(input: &str) -> (HashMap<Vec2<usize>, u64>, u64) {
    let grid = Grid::parse(input, identity);
    let start = grid.find('S').unwrap();

    let mut out = 0;
    let mut beams = HashMap::new();
    beams.insert(start, 1);

    loop {
        let mut next_beams = HashMap::new();
        for (beam, count) in beams.iter() {
            let next = Direction::Down.advance(*beam);
            let Some(tile) = grid.get(next) else {
                continue;
            };

            if *tile == '^' {
                out += 1;
                *next_beams.entry(next - vector!(1, 0)).or_default() += *count;
                *next_beams.entry(next + vector!(1, 0)).or_default() += *count;
            } else {
                *next_beams.entry(next).or_default() += *count;
            }
        }

        if next_beams.is_empty() {
            break;
        }

        beams = next_beams;
    }

    (beams, out)
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
