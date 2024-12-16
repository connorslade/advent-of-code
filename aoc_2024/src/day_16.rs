use std::{
    cmp::Ordering,
    collections::{BinaryHeap, HashSet},
};

use aoc_lib::{direction::cardinal::Direction, matrix::Matrix};
use common::{solution, Answer};
use nd_vec::Vec2;

solution!("Reindeer Maze", 16);

fn part_a(input: &str) -> Answer {
    let map = Maze::parse(input);
    let (_, shortest) = map.foreword();
    shortest.into()
}

fn part_b(input: &str) -> Answer {
    let map = Maze::parse(input);
    let (scores, _) = map.foreword();
    map.reverse(scores).into()
}

struct Maze {
    map: Matrix<Tile>,

    start: Vec2<usize>,
    end: Vec2<usize>,
}

#[derive(PartialEq, Eq)]
enum Tile {
    Empty,
    Wall,
    Start,
    End,
}

#[derive(PartialEq, Eq)]
struct Item {
    pos: Vec2<usize>,
    dir: Direction,
    score: u32,
}

impl Maze {
    fn parse(input: &str) -> Self {
        let map = Matrix::new_chars(input, |c| match c {
            '.' => Tile::Empty,
            '#' => Tile::Wall,
            'S' => Tile::Start,
            'E' => Tile::End,
            _ => panic!(),
        });

        let start = map.find(Tile::Start).unwrap();
        let end = map.find(Tile::End).unwrap();

        Self { map, start, end }
    }

    /// Use dijkstra's to find the shortest path, populating a costs grid with
    /// the minimum cost needed to reach that tile, which will be used for part B.
    fn foreword(&self) -> (Matrix<[u32; 4]>, u32) {
        let mut queue = BinaryHeap::new();
        let mut seen = HashSet::new();
        let mut costs = Matrix::new_default(self.map.size, [u32::MAX; 4]);

        queue.push(Item::new(self.start, Direction::Right, 0));

        while let Some(Item { pos, dir, score }) = queue.pop() {
            let min = &mut costs[pos];
            min[dir as usize] = min[dir as usize].min(score);

            if !seen.insert((pos, dir)) {
                continue;
            }

            if self.map[pos] == Tile::End {
                return (costs, score);
            }

            let next = dir.wrapping_advance(pos);
            if self.map.contains(next) && self.map[next] != Tile::Wall {
                queue.push(Item::new(next, dir, score + 1));
            }

            for dir in [dir.turn_left(), dir.turn_right()] {
                queue.push(Item::new(pos, dir, score + 1000));
            }
        }

        unreachable!("No path found")
    }

    /// Walks backwards from the end using a BFS to find all the tiles that are
    /// on any of the shortest path.
    fn reverse(&self, mut scores: Matrix<[u32; 4]>) -> u32 {
        let mut seen = HashSet::new();
        let mut seen_queue = vec![];

        let end_lowest = scores.get(self.end).unwrap();
        for dir in Direction::ALL {
            let min_cost = end_lowest[dir as usize];
            seen_queue.push(Item::new(self.end, dir, min_cost));
        }

        while let Some(item) = seen_queue.pop() {
            seen.insert(item.pos);
            if item.pos == self.start {
                continue;
            }

            let next = item.dir.opposite().wrapping_advance(item.pos);
            for next in [
                Item::new(next, item.dir, item.score - 1),
                Item::new(item.pos, item.dir.turn_left(), item.score - 1000),
                Item::new(item.pos, item.dir.turn_right(), item.score - 1000),
            ] {
                if self.map.contains(next.pos)
                    && self.map[next.pos] != Tile::Wall
                    && next.score == scores.get(next.pos).unwrap()[next.dir as usize]
                {
                    scores.get_mut(next.pos).unwrap()[next.dir as usize] = u32::MAX;
                    seen_queue.push(next);
                }
            }
        }

        seen.len() as u32
    }
}

impl Item {
    fn new(pos: Vec2<usize>, dir: Direction, score: u32) -> Self {
        Self { pos, dir, score }
    }
}

impl Ord for Item {
    fn cmp(&self, other: &Self) -> Ordering {
        other.score.cmp(&self.score)
    }
}

impl PartialOrd for Item {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        Some(self.cmp(other))
    }
}

#[cfg(test)]
mod test {
    use indoc::indoc;

    const CASE: &str = indoc! {"
        ###############
        #.......#....E#
        #.#.###.#.###.#
        #.....#.#...#.#
        #.###.#####.#.#
        #.#.#.......#.#
        #.#.#####.###.#
        #...........#.#
        ###.#.#####.#.#
        #...#.....#.#.#
        #.#.#.###.#.#.#
        #.....#...#.#.#
        #.###.#.#.#.#.#
        #S..#.....#...#
        ###############
    "};

    #[test]
    fn part_a() {
        assert_eq!(super::part_a(CASE), 7036.into());
    }

    #[test]
    fn part_b() {
        assert_eq!(super::part_b(CASE), 45.into());
    }
}
