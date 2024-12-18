use std::{
    collections::{HashMap, HashSet, VecDeque},
    fmt::{Debug, Write},
    usize,
};

use aoc_lib::{direction::cardinal::Direction, matrix::Grid};
use common::{solution, Answer};
use nd_vec::{vector, Vec2};

solution!("RAM Run", 18);

fn part_a(input: &str) -> Answer {
    let mut map = Map::parse(input, vector!(71, 71));
    map.fill_all();
    map.shortest_path().into()
}

fn part_b(input: &str) -> Answer {
    let mut map = Map::parse(input, vector!(71, 71));
    loop {
        let next = map.fill_next();
        if map.shortest_path() == usize::MAX {
            return format!("{},{}", next.x(), next.y()).into();
        }
    }

    unreachable!()
}

#[derive(Clone, PartialEq, Eq)]
enum Tile {
    Filled,
    Empty,
}

impl Debug for Tile {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Tile::Filled => f.write_char('#'),
            Tile::Empty => f.write_char('.'),
        }
    }
}

struct Map {
    board: Grid<Tile>,
    falling: Vec<Vec2<usize>>,
}

impl Map {
    fn parse(input: &str, size: Vec2<usize>) -> Self {
        let falling = input
            .lines()
            .map(|x| {
                let (a, b) = x.split_once(',').unwrap();
                vector!(a.parse::<usize>().unwrap(), b.parse::<usize>().unwrap())
            })
            .collect::<Vec<_>>();

        let board = Grid::parse(size, Tile::Empty);

        Self { falling, board }
    }

    fn fill_next(&mut self) -> Vec2<usize> {
        let pos = self.falling.remove(0);
        self.board.set(pos, Tile::Filled);
        pos
    }

    fn fill_all(&mut self) {
        for ins in &self.falling[0..1024] {
            self.board.set(*ins, Tile::Filled);
        }

        println!("{:?}", self.board);
    }

    fn shortest_path(&self) -> usize {
        let mut queue = VecDeque::new();
        let mut seen = HashSet::new();
        queue.push_back((vector!(0, 0), 0));

        while let Some((pos, depth)) = queue.pop_front() {
            if pos + vector!(1, 1) == self.board.size() {
                return depth;
            }

            if !seen.insert(pos) {
                continue;
            }

            for dir in Direction::ALL {
                let next = dir.wrapping_advance(pos);
                if self.board.get(next) == Some(&Tile::Empty) {
                    queue.push_back((next, depth + 1));
                }
            }
        }

        usize::MAX
    }
}

#[cfg(test)]
mod test {
    use indoc::indoc;

    const CASE: &str = indoc! {"
        5,4
        4,2
        4,5
        3,0
        2,1
        6,3
        2,4
        1,5
        0,6
        3,3
        2,6
        5,1
        1,2
        5,5
        2,5
        6,5
        1,4
        0,4
        6,4
        1,1
        6,1
        1,0
        0,5
        1,6
        2,0
    "};

    #[test]
    fn part_a() {
        assert_eq!(super::part_a(CASE), 22.into());
    }

    #[test]
    fn part_b() {
        assert_eq!(super::part_b(CASE), ().into());
    }
}
