use std::collections::HashSet;

use aoc_lib::{direction::cardinal::Direction, matrix::Matrix};
use common::{solution, Answer};
use nd_vec::Vec2;
use rayon::iter::{ParallelBridge, ParallelIterator};

solution!("Guard Gallivant", 6);

fn part_a(input: &str) -> Answer {
    Map::new(input).visited().len().into()
}

fn part_b(input: &str) -> Answer {
    let map = Map::new(input);

    map.visited()
        .into_iter()
        .par_bridge()
        .map(|x| map.loops(x) as usize)
        .sum::<usize>()
        .into()
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
        let start = map.find(Tile::Start).unwrap();

        Self { map, start }
    }

    fn visited(&self) -> HashSet<Vec2<usize>> {
        let mut seen = HashSet::new();
        let mut pos = self.start;
        let mut direction = Direction::Up;

        loop {
            seen.insert(pos);

            let Some(mut next) = direction.try_advance(pos) else {
                break;
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

        seen
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
