use std::collections::{HashSet, VecDeque};

use aoc_lib::{direction::cardinal::Direction, matrix::Matrix};
use common::{solution, Answer};
use nd_vec::Vec2;

solution!("Hoof It", 10);

fn part_a(input: &str) -> Answer {
    let map = Map::parse(input);
    map.trailheads()
        .into_iter()
        .map(|x| map.score(x))
        .sum::<usize>()
        .into()
}

fn part_b(input: &str) -> Answer {
    let map = Map::parse(input);
    map.trailheads()
        .into_iter()
        .map(|x| map.rating(x))
        .sum::<usize>()
        .into()
}

struct Map {
    board: Matrix<u32>,
}

impl Map {
    fn parse(input: &str) -> Self {
        let board = Matrix::new_chars(input, |x| x.to_digit(10).unwrap());
        Self { board }
    }

    fn trailheads(&self) -> Vec<Vec2<usize>> {
        self.board
            .iter()
            .filter(|(_, &tile)| tile == 0)
            .map(|(pos, _)| pos)
            .collect()
    }

    fn score(&self, pos: Vec2<usize>) -> usize {
        let mut queue = VecDeque::new();
        let mut seen = HashSet::new();

        queue.push_back(pos);
        seen.insert(pos);

        let mut score = 0;
        while let Some(pos) = queue.pop_front() {
            let value = *self.board.get(pos).unwrap();
            if value == 9 {
                score += 1;
            }

            for dir in Direction::ALL {
                if let Some(next) = dir.try_advance(pos) {
                    if self.board.contains(next)
                        && *self.board.get(next).unwrap() == value + 1
                        && seen.insert(next)
                    {
                        queue.push_back(next);
                    }
                }
            }
        }

        score
    }

    fn rating(&self, pos: Vec2<usize>) -> usize {
        fn inner(board: &Matrix<u32>, pos: Vec2<usize>, mut seen: HashSet<Vec2<usize>>) -> usize {
            let value = *board.get(pos).unwrap();
            if value == 9 {
                return 1;
            }

            let mut sum = 0;
            for dir in Direction::ALL {
                if let Some(next) = dir.try_advance(pos) {
                    if board.contains(next)
                        && *board.get(next).unwrap() == value + 1
                        && seen.insert(next)
                    {
                        sum += inner(board, next, seen.clone());
                    }
                }
            }

            sum
        }

        inner(&self.board, pos, HashSet::new())
    }
}

#[cfg(test)]
mod test {
    use indoc::indoc;

    const CASE: &str = indoc! {"
        89010123
        78121874
        87430965
        96549874
        45678903
        32019012
        01329801
        10456732
    "};

    #[test]
    fn part_a() {
        assert_eq!(super::part_a(CASE), 36.into());
    }

    #[test]
    fn part_b() {
        assert_eq!(super::part_b(CASE), 81.into());
    }
}
