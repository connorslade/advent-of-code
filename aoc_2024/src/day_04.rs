use std::convert::identity;

use aoc_lib::{direction::ordinal::Direction, matrix::Grid};
use common::{solution, Answer};
use nd_vec::vector;

solution!("Ceres Search", 4);

fn part_a(input: &str) -> Answer {
    let matrix = Grid::new(input, identity);
    let mut count = 0;

    for y in 0..matrix.size.y() {
        for x in 0..matrix.size.x() {
            let start = vector!(x, y);
            if *matrix.get(start).unwrap() != 'X' {
                continue;
            }

            'outer: for dir in Direction::ALL {
                let mut pos = start;
                for expected in ['M', 'A', 'S'] {
                    let next = dir.try_advance(pos);
                    let Some(next) = next else { continue 'outer };
                    pos = next;

                    if Some(&expected) != matrix.get(pos) {
                        continue 'outer;
                    };
                }

                count += 1;
            }
        }
    }

    count.into()
}

/// The directions to advance from the middle 'A' for each MAS instance.
const MAS_DIRECTIONS: [[Direction; 2]; 2] = [
    [Direction::NorthEast, Direction::SouthWest],
    [Direction::SouthEast, Direction::NorthWest],
];

fn part_b(input: &str) -> Answer {
    let matrix = Grid::new(input, identity);
    let mut count = 0;

    for y in 0..matrix.size.y() {
        'outer: for x in 0..matrix.size.x() {
            let start = vector!(x, y);
            if *matrix.get(start).unwrap() != 'A' {
                continue;
            }

            for mas in MAS_DIRECTIONS {
                let (mut m, mut s) = (false, false);
                for dir in mas {
                    let Some(&chr) = dir.try_advance(start).and_then(|x| matrix.get(x)) else {
                        continue 'outer;
                    };

                    m |= chr == 'M';
                    s |= chr == 'S';
                }

                if !(m && s) {
                    continue 'outer;
                }
            }

            count += 1;
        }
    }

    count.into()
}

#[cfg(test)]
mod test {
    use indoc::indoc;

    const CASE: &str = indoc! {"
        MMMSXXMASM
        MSAMXMSMSA
        AMXSXMAAMM
        MSAMASMSMX
        XMASAMXAMM
        XXAMMXXAMA
        SMSMSASXSS
        SAXAMASAAA
        MAMMMXMMMM
        MXMXAXMASX
    "};

    #[test]
    fn part_a() {
        assert_eq!(super::part_a(CASE), 18.into());
    }

    #[test]
    fn part_b() {
        assert_eq!(super::part_b(CASE), 9.into());
    }
}
