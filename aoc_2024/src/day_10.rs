use std::collections::{HashSet, VecDeque};

use aoc_lib::{direction::cardinal::Direction, matrix::Grid};
use common::{solution, Answer};
use nd_vec::Vec2;

solution!("Hoof It", 10);

fn part_a(input: &str) -> Answer {
    solve(input, false).into()
}

fn part_b(input: &str) -> Answer {
    solve(input, true).into()
}

fn solve(input: &str, part_b: bool) -> usize {
    let map = Map::parse(input);
    map.trailheads()
        .map(|x| map.score(x, !part_b))
        .sum::<usize>()
}

struct Map {
    board: Grid<u32>,
}

impl Map {
    fn parse(input: &str) -> Self {
        let board = Grid::new(input, |x| x.to_digit(10).unwrap());
        Self { board }
    }

    // Find the coordinates of all 0s
    fn trailheads(&self) -> impl Iterator<Item = Vec2<usize>> + '_ {
        self.board
            .iter()
            .filter(|(_, &tile)| tile == 0)
            .map(|(pos, _)| pos)
    }

    // Simple BFS for pathfinding, where we don't avoid going to already
    // explored tiles if on part B.
    fn score(&self, pos: Vec2<usize>, no_repeats: bool) -> usize {
        let mut queue = VecDeque::new();
        let mut seen = HashSet::new();

        queue.push_back(pos);
        seen.insert(pos);

        let mut score = 0;
        while let Some(pos) = queue.pop_front() {
            let value = *self.board.get(pos).unwrap();
            score += (value == 9) as usize;

            queue.extend(
                Direction::ALL
                    .iter()
                    .filter_map(|&dir| dir.try_advance(pos))
                    .filter(|&next| {
                        self.board.contains(next)
                            && *self.board.get(next).unwrap() == value + 1
                            && (!no_repeats || seen.insert(next))
                    }),
            );
        }

        score
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
