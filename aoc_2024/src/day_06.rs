use std::collections::HashSet;

use aoc_lib::{direction::cardinal::Direction, matrix::Matrix};
use common::{solution, Answer};
use nd_vec::{vector, Vec2};

solution!("Guard Gallivant", 6);

fn part_a(input: &str) -> Answer {
    Map::new(input).visited().into()
}

fn part_b(input: &str) -> Answer {
    let map = Map::new(input);

    let mut count = 0;
    for y in 0..map.map.size.y() {
        for x in 0..map.map.size.x() {
            let obs = vector!(x, y);
            count += map.loops(obs) as usize;
        }
    }

    count.into()
}

#[derive(Debug, PartialEq, Eq)]
enum Tile {
    Obstacle,
    Start,
    None,
}

struct Map {
    map: Matrix<Tile>,
    start: Vec2<usize>,
}

impl Map {
    fn new(input: &str) -> Self {
        let map = Matrix::new_chars(input, |x| match x {
            '#' => Tile::Obstacle,
            '^' => Tile::Start,
            _ => Tile::None,
        });

        let mut start = vector!(0, 0);
        'outer: for y in 0..map.size.y() {
            for x in 0..map.size.x() {
                let tmp = vector!(x, y);
                if *map.get(tmp).unwrap() == Tile::Start {
                    start = tmp;
                    break 'outer;
                }
            }
        }

        Self { map, start }
    }

    fn visited(&self) -> usize {
        let mut seen = HashSet::new();
        let mut pos = self.start;
        let mut direction = Direction::Up;

        loop {
            seen.insert(pos);

            let mut next = match direction.try_advance(pos) {
                Some(x) => x,
                None => break,
            };
            let Some(tile) = self.map.get(next) else {
                break;
            };
            if *tile == Tile::Obstacle {
                direction = direction.turn_right();
                next = match direction.try_advance(pos) {
                    Some(x) => x,
                    None => break,
                }
            }

            pos = next;
        }

        seen.len()
    }

    fn loops(&self, obstacle: Vec2<usize>) -> bool {
        let mut seen = HashSet::new();
        let mut pos = self.start;
        let mut direction = Direction::Up;

        loop {
            let Some(tile) = self.map.get(pos) else {
                break;
            };
            if obstacle == pos || *tile == Tile::Obstacle {
                pos = direction.opposite().advance(pos);
                direction = direction.turn_right();
            }

            if !seen.insert((pos, direction)) {
                return true;
            }

            pos = match direction.try_advance(pos) {
                Some(x) => x,
                None => break,
            };
        }

        false
    }
}

#[cfg(test)]
mod test {
    use indoc::indoc;

    const CASE: &str = indoc! {"
        ....#.....
        .........#
        ..........
        ..#.......
        .......#..
        ..........
        .#..^.....
        ........#.
        #.........
        ......#...
    "};

    #[test]
    fn part_a() {
        assert_eq!(super::part_a(CASE), 41.into());
    }

    #[test]
    fn part_b() {
        assert_eq!(super::part_b(CASE), 6.into());
    }
}
