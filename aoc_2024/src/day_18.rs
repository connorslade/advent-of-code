use std::collections::{HashSet, VecDeque};

use aoc_lib::{direction::cardinal::Direction, matrix::Grid};
use common::{solution, Answer};
use nd_vec::{vector, Vec2};

solution!("RAM Run", 18);

fn part_a(input: &str) -> Answer {
    let mut map = Map::parse(input, vector!(71, 71));
    map.fill_to(1023);
    map.shortest_path().into()
}

fn part_b(input: &str) -> Answer {
    let mut map = Map::parse(input, vector!(71, 71));

    let (mut start, mut end) = (0, map.falling.len() - 1);

    while start <= end {
        let mid = (start + end) / 2;
        let next = map.fill_to(mid);

        let works = map.shortest_path() != usize::MAX;

        if !works && {
            map.fill_to(mid - 1);
            map.shortest_path() != usize::MAX
        } {
            return format!("{},{}", next.x(), next.y()).into();
        }

        if works {
            start = mid + 1;
        } else {
            end = mid - 1;
        }
    }

    panic!("No solution found")
}

struct Map {
    board: Grid<bool>,
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

        let board = Grid::new(size, false);

        Self { falling, board }
    }

    fn fill_to(&mut self, end: usize) -> Vec2<usize> {
        self.board.fill(false);
        for ins in &self.falling[0..=end] {
            self.board.set(*ins, true);
        }

        self.falling[end]
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
                if self.board.get(next) == Some(&false) {
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
    use nd_vec::vector;

    use super::Map;

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
        let mut map = Map::parse(CASE, vector!(7, 7));
        map.fill_to(11);
        assert_eq!(map.shortest_path(), 22);
    }
}
