use std::{
    collections::{HashMap, HashSet},
    convert::identity,
};

use aoc_lib::{direction::Direction, matrix::Matrix};
use common::{Answer, ISolution};
use nd_vec::{vector, Vec2};

type Pos = Vec2<usize>;

pub struct Day23;

impl ISolution for Day23 {
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

// Simple DFS to find the longest path for part A
fn solve_a(map: &Matrix<char>, visited: &mut HashSet<Pos>, pos: Pos, idx: u32) -> u32 {
    if pos == map.size() - vector!(2, 1) {
        return idx;
    }

    let mut longest = 0;
    for dir in Direction::ALL.iter() {
        let Some(new_pos) = dir.try_advance(pos) else {
            continue;
        };

        let next = map[new_pos];
        if next == '#' || !dir_matches(*dir, next) && next != '.' || !visited.insert(pos) {
            continue;
        }

        longest = longest.max(solve_a(map, visited, new_pos, idx + 1));
        visited.remove(&pos);
    }

    longest
}

// Convert the map into a graph and collapse it to find the longest path for part B
fn solve_b(map: &Matrix<char>) -> u32 {
    // == Build graph ==
    let mut graph = HashMap::new();
    let mut add_edge = |a: Pos, b: Pos| {
        graph.entry(a).or_insert_with(HashSet::new).insert((b, 1));
        graph.entry(b).or_insert_with(HashSet::new).insert((a, 1));
    };

    for y in 0..map.size().y() {
        for x in 0..map.size().x() {
            let pos = vector!(x, y);
            if map[pos] == '#' {
                continue;
            }

            for new_pos in Direction::ALL.iter().filter_map(|dir| dir.try_advance(pos)) {
                if map.contains(new_pos) && map[new_pos] != '#' {
                    add_edge(pos, new_pos);
                }
            }
        }
    }

    // == Collapse graph ==
    let mut dirty = true;
    while dirty {
        dirty = false;
        for key in graph.keys().copied().collect::<Vec<_>>() {
            if graph[&key].len() != 2 {
                continue;
            }

            let ((a, a_score), (b, b_score)) = {
                let mut iter = graph[&key].iter();
                (*iter.next().unwrap(), *iter.next().unwrap())
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

    // == Find longest path ==
    let mut queue = Vec::new();
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

fn dir_matches(dir: Direction, chr: char) -> bool {
    chr == match dir {
        Direction::Up => '^',
        Direction::Down => 'v',
        Direction::Left => '<',
        Direction::Right => '>',
    }
}

#[cfg(test)]
mod test {
    use common::ISolution;
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
