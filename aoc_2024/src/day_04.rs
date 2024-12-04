use aoc_lib::matrix::Matrix;
use common::{solution, Answer};
use nd_vec::{vector, Vector};

solution!("Ceres Search", 4);

fn part_a(input: &str) -> Answer {
    let matrix = Matrix::new_chars(input, |x| x);
    let mut count = 0;

    for y in 0..matrix.size.y() {
        for x in 0..matrix.size.x() {
            let start = vector!(x, y);
            for dir in Direction::ALL {
                let mut pos = start.clone();
                let mut word = String::new();
                for _ in 0..4 {
                    if let Some(&chr) = matrix.get(pos) {
                        word.push(chr);
                    } else {
                        break;
                    }

                    let Some(next) = dir.try_advance(pos) else {
                        break;
                    };
                    pos = next;
                }

                if word == "XMAS" {
                    count += 1;
                }
            }
        }
    }

    count.into()
}

fn part_b(input: &str) -> Answer {
    let matrix = Matrix::new_chars(input, |x| x);
    let mut count = 0;

    for y in 0..matrix.size.y() {
        'outer: for x in 0..matrix.size.x() {
            let start = vector!(x, y);
            if *matrix.get(start).unwrap() != 'A' {
                continue;
            }

            let directions = [
                [Direction::NorthEast, Direction::SouthWest],
                [Direction::SouthEast, Direction::NorthWest],
                // should have a 'M' and a 'S'
            ];

            for mas in directions {
                let (mut m, mut s) = (false, false);
                for dir in mas {
                    let Some(pos) = dir.try_advance(start) else {
                        continue 'outer;
                    };
                    let Some(&chr) = matrix.get(pos) else {
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

#[derive(Debug)]
enum Direction {
    North,
    NorthEast,
    East,
    SouthEast,
    South,
    SouthWest,
    West,
    NorthWest,
}

impl Direction {
    pub const ALL: [Direction; 8] = [
        Direction::North,
        Direction::NorthEast,
        Direction::East,
        Direction::SouthEast,
        Direction::South,
        Direction::SouthWest,
        Direction::West,
        Direction::NorthWest,
    ];

    pub fn try_advance(&self, pos: Vector<usize, 2>) -> Option<Vector<usize, 2>> {
        Some(match self {
            Self::North => vector!(pos.x(), pos.y() + 1),
            Self::NorthEast => vector!(pos.x() + 1, pos.y() + 1),
            Self::East => vector!(pos.x() + 1, pos.y()),
            Self::SouthEast if pos.y() > 0 => vector!(pos.x() + 1, pos.y() - 1),
            Self::South if pos.y() > 0 => vector!(pos.x(), pos.y() - 1),
            Self::SouthWest if pos.x() > 0 && pos.y() > 0 => vector!(pos.x() - 1, pos.y() - 1),
            Self::West if pos.x() > 0 => vector!(pos.x() - 1, pos.y()),
            Self::NorthWest if pos.x() > 0 => vector!(pos.x() - 1, pos.y() + 1),
            _ => return None,
        })
    }
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
