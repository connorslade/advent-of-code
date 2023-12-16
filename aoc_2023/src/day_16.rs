use std::collections::HashSet;

use nd_vec::{vector, Vec2};

use common::{Answer, Solution};

type Pos = Vec2<isize>;

pub struct Day16;

impl Solution for Day16 {
    fn name(&self) -> &'static str {
        "The Floor Will Be Lava"
    }

    fn part_a(&self, input: &str) -> Answer {
        let tiles = parse(input);
        tiles.lazer(vector!(-1, 0), Direction::Right).into()
    }

    fn part_b(&self, input: &str) -> Answer {
        let tiles = parse(input);
        let mut max = 0;

        for y in 0..tiles.tiles.len() {
            max = max.max(tiles.lazer(vector!(-1, y as isize), Direction::Right));
            max = max.max(tiles.lazer(
                vector!(tiles.tiles[0].len() as isize, y as isize),
                Direction::Left,
            ));
        }

        for x in 0..tiles.tiles[0].len() {
            max = max.max(tiles.lazer(vector!(x as isize, -1), Direction::Down));
            max = max.max(tiles.lazer(
                vector!(x as isize, tiles.tiles.len() as isize),
                Direction::Up,
            ));
        }

        max.into()
    }
}

#[derive(Debug)]
struct Cavern {
    tiles: Vec<Vec<Tile>>,
}

#[derive(Debug, Clone, Copy)]
enum Direction {
    Up,
    Down,
    Left,
    Right,
}

#[derive(Debug, Clone, Copy, PartialEq)]
enum Tile {
    Empty,      // .
    Horizontal, // -
    Vertical,   // |
    SlantLeft,  // \
    SlantRight, // /
}

fn parse(input: &str) -> Cavern {
    let mut tiles = Vec::new();

    for line in input.trim().lines() {
        let mut row = Vec::new();
        for c in line.chars() {
            row.push(Tile::from_char(c));
        }
        tiles.push(row);
    }

    Cavern { tiles }
}

impl Cavern {
    fn lazer(&self, start: Pos, direction: Direction) -> usize {
        fn _lazer(cavern: &Cavern, visited: &mut HashSet<Pos>, mut pos: Pos, mut dir: Direction) {
            visited.insert(pos);

            loop {
                if let Some(i) = dir.advance(pos) {
                    pos = i;
                } else {
                    break;
                }

                if pos.x() >= cavern.tiles[0].len() as isize
                    || pos.y() >= cavern.tiles.len() as isize
                {
                    break;
                }

                let new = visited.insert(pos);
                let tile = cavern.tiles[pos.y() as usize][pos.x() as usize];

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
        _lazer(&self, &mut visited, start, direction);

        visited.len() - 1
    }
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
        match self {
            Self::Horizontal => matches!(direction, Direction::Left | Direction::Right),
            Self::Vertical => matches!(direction, Direction::Up | Direction::Down),
            _ => false,
        }
    }
}

impl Direction {
    fn advance(&self, pos: Pos) -> Option<Pos> {
        Some(match self {
            Self::Up if pos.y() > 0 => pos - vector!(0, 1),
            Self::Down => pos + vector!(0, 1),
            Self::Left if pos.x() > 0 => pos - vector!(1, 0),
            Self::Right => pos + vector!(1, 0),
            _ => return None,
        })
    }
}

#[cfg(test)]
mod test {
    use common::Solution;
    use indoc::indoc;

    use super::Day16;

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
        assert_eq!(Day16.part_a(CASE), 46.into());
    }

    #[test]
    fn part_b() {
        assert_eq!(Day16.part_b(CASE), 51.into());
    }
}
