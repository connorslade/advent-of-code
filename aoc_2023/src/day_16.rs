use std::collections::HashSet;

use aoc_lib::{direction::cardinal::Direction, matrix::Matrix};
use common::{solution, Answer};
use nd_vec::{vector, Vec2};

solution!("The Floor Will Be Lava", 16);

type Pos = Vec2<isize>;

fn part_a(input: &str) -> Answer {
    lazer(&parse(input), vector!(-1, 0), Direction::Right).into()
}

fn part_b(input: &str) -> Answer {
    let tiles = parse(input);
    let mut max = 0;

    let size = tiles.size.num_cast::<isize>().unwrap();
    for y in 0..size.y() {
        max = max.max(lazer(&tiles, vector!(-1, y), Direction::Right));
        max = max.max(lazer(&tiles, vector!(size.x(), y), Direction::Left));
    }

    for x in 0..size.x() {
        max = max.max(lazer(&tiles, vector!(x, -1), Direction::Down));
        max = max.max(lazer(&tiles, vector!(x, size.y()), Direction::Up));
    }

    max.into()
}

fn parse(input: &str) -> Matrix<Tile> {
    Matrix::new_chars(input, Tile::from_char)
}

#[derive(Debug, Clone, Copy, PartialEq)]
enum Tile {
    Empty,      // .
    Horizontal, // -
    Vertical,   // |
    SlantLeft,  // \
    SlantRight, // /
}

fn lazer(cavern: &Matrix<Tile>, start: Pos, direction: Direction) -> usize {
    fn _lazer(cavern: &Matrix<Tile>, visited: &mut HashSet<Pos>, mut pos: Pos, mut dir: Direction) {
        loop {
            pos = dir.advance(pos);
            if !cavern.contains(pos) {
                break;
            }

            let new = visited.insert(pos);
            let tile = cavern[pos];

            if tile == Tile::Empty || tile.matching_dir(dir) {
                continue;
            }

            match tile {
                Tile::SlantLeft => {
                    dir = match dir {
                        Direction::Up => Direction::Left,
                        Direction::Down => Direction::Right,
                        Direction::Left => Direction::Up,
                        Direction::Right => Direction::Down,
                    }
                }
                Tile::SlantRight => {
                    dir = match dir {
                        Direction::Up => Direction::Right,
                        Direction::Down => Direction::Left,
                        Direction::Left => Direction::Down,
                        Direction::Right => Direction::Up,
                    }
                }
                Tile::Horizontal if new => {
                    _lazer(cavern, visited, pos, Direction::Left);
                    _lazer(cavern, visited, pos, Direction::Right);
                    break;
                }
                Tile::Vertical if new => {
                    _lazer(cavern, visited, pos, Direction::Up);
                    _lazer(cavern, visited, pos, Direction::Down);
                    break;
                }
                _ => break,
            };
        }
    }

    let mut visited = HashSet::new();
    _lazer(cavern, &mut visited, start, direction);
    visited.len()
}

impl Tile {
    fn from_char(c: char) -> Self {
        match c {
            '.' => Self::Empty,
            '-' => Self::Horizontal,
            '|' => Self::Vertical,
            '/' => Self::SlantRight,
            '\\' => Self::SlantLeft,
            _ => panic!("Invalid tile: {}", c),
        }
    }

    fn matching_dir(&self, direction: Direction) -> bool {
        matches!(
            (self, direction),
            (Self::Horizontal, Direction::Left | Direction::Right)
                | (Self::Vertical, Direction::Up | Direction::Down)
        )
    }
}

#[cfg(test)]
mod test {
    use indoc::indoc;

    const CASE: &str = indoc! {r"
        .|...\....
        |.-.\.....
        .....|-...
        ........|.
        ..........
        .........\
        ..../.\\..
        .-.-/..|..
        .|....-|.\
        ..//.|....
    "};

    #[test]
    fn part_a() {
        assert_eq!(super::part_a(CASE), 46.into());
    }

    #[test]
    fn part_b() {
        assert_eq!(super::part_b(CASE), 51.into());
    }
}
