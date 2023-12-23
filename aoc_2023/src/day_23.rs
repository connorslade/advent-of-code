use std::{
    collections::{HashMap, HashSet},
    convert::identity,
};

use common::{Answer, Solution};
use nd_vec::{vector, Vec2};

use crate::aoc_lib::{direction::Direction, matrix::Matrix};

type Pos = Vec2<usize>;

pub struct Day23;

impl Solution for Day23 {
    fn name(&self) -> &'static str {
        "A Long Walk"
    }

    fn part_a(&self, input: &str) -> Answer {
        solve_a(&parse(input), &mut HashSet::new(), vector!(1, 0), 0).into()
    }

    fn part_b(&self, input: &str) -> Answer {
        solve_b(&parse(input)).into()
    }
}

fn solve_a(map: &Matrix<char>, visited: &mut HashSet<Pos>, pos: Pos, idx: u32) -> u32 {
    if pos == map.size() - vector!(2, 1) {
        return idx;
    }

    let mut longest = 0;
    for dir in Direction::ALL.iter() {
        let Some(new_pos) = dir.try_advance(pos) else {
            continue;
        };

        if map[new_pos] == '#' {
            continue;
        }

        let next = map[new_pos];
        if !(match dir {
            Direction::Up => next == '^',
            Direction::Down => next == 'v',
            Direction::Left => next == '<',
            Direction::Right => next == '>',
        } || next == '.')
        {
            continue;
        }

        if !visited.insert(pos) {
            continue;
        }

        longest = longest.max(solve_a(map, visited, new_pos, idx + 1));
        visited.remove(&pos);
    }

    longest
}

fn solve_b(map: &Matrix<char>) -> u32 {
    // Build graph
    let mut graph = HashMap::new();
    for y in 0..map.size().y() {
        for x in 0..map.size().x() {
            let pos = vector!(x, y);
            let c = map[pos];
            if !b".>v".contains(&(c as u8)) {
                continue;
            }

            for dir in Direction::ALL {
                if let Some(new_pos) = dir.try_advance(pos) {
                    if new_pos.x() < map.size.x()
                        && new_pos.y() < map.size.y()
                        && b".>v".contains(&(map[new_pos] as u8))
                    {
                        graph
                            .entry(pos)
                            .or_insert_with(HashSet::new)
                            .insert((new_pos, 1));
                        graph
                            .entry(new_pos)
                            .or_insert_with(HashSet::new)
                            .insert((pos, 1));
                    }
                }
            }
        }
    }

    // Collapse graph
    let mut dirty = true;
    while dirty {
        dirty = false;
        for key in graph.keys().copied().collect::<Vec<_>>() {
            if graph[&key].len() != 2 {
                continue;
            }

            let ((a, a_score), (b, b_score)) = {
                let mut iter = graph[&key].iter();
                let a = iter.next().unwrap();
                let b = iter.next().unwrap();
                (*a, *b)
            };

            let a_graph = graph.get_mut(&a).unwrap();
            a_graph.retain(|(pos, _)| *pos != key);
            a_graph.insert((b, a_score + b_score));

            let b_graph = graph.get_mut(&b).unwrap();
            b_graph.retain(|(pos, _)| *pos != key);
            b_graph.insert((a, a_score + b_score));

            graph.remove(&key);
            dirty = true;
        }
    }

    // Find longest path

    let mut queue = Vec::<(Pos, Option<u32>)>::new();
    let mut visited = HashSet::new();
    let mut max = 0;

    queue.push((vector!(1, 0), Some(0)));
    while let Some((pos, distance)) = queue.pop() {
        let Some(distance) = distance else {
            visited.remove(&pos);
            continue;
        };

        if pos == map.size() - vector!(2, 1) {
            max = max.max(distance);
            continue;
        }

        if !visited.insert(pos) {
            continue;
        }

        queue.push((pos, None));
        for (pos, dist) in &graph[&pos] {
            queue.push((*pos, Some(distance + dist)));
        }
    }

    max
}

fn parse(input: &str) -> Matrix<char> {
    Matrix::new_chars(input, identity)
}

#[cfg(test)]
mod test {
    use common::Solution;
    use indoc::indoc;

    use super::Day23;

    const CASE: &str = indoc! {"
        #.#####################
        #.......#########...###
        #######.#########.#.###
        ###.....#.>.>.###.#.###
        ###v#####.#v#.###.#.###
        ###.>...#.#.#.....#...#
        ###v###.#.#.#########.#
        ###...#.#.#.......#...#
        #####.#.#.#######.#.###
        #.....#.#.#.......#...#
        #.#####.#.#.#########v#
        #.#...#...#...###...>.#
        #.#.#v#######v###.###v#
        #...#.>.#...>.>.#.###.#
        #####v#.#.###v#.#.###.#
        #.....#...#...#.#.#...#
        #.#########.###.#.#.###
        #...###...#...#...#.###
        ###.###.#.###v#####v###
        #...#...#.#.>.>.#.>.###
        #.###.###.#.###.#.#v###
        #.....###...###...#...#
        #####################.#
    "};

    #[test]
    fn part_a() {
        assert_eq!(Day23.part_a(CASE), 94.into());
    }

    #[test]
    fn part_b() {
        assert_eq!(Day23.part_b(CASE), 154.into());
    }
}
