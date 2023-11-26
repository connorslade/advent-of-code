use std::collections::VecDeque;

use crate::aoc_lib;
use common::{Answer, Solution};

type Point = aoc_lib::Point<usize>;

pub struct Day12;

impl Solution for Day12 {
    fn name(&self) -> &'static str {
        "Hill Climbing Algorithm"
    }

    fn part_a(&self, input: &str) -> Answer {
        let map = parse(input);

        run_path(&map, |a, b| a <= b + 1, |c| c == map.end)
            .unwrap()
            .into()
    }

    fn part_b(&self, input: &str) -> Answer {
        let mut map = parse(input);
        map.start = map.end;
        map.current = map.start;

        run_path(&map, |a, b| b <= a + 1, |c| map.data[c.y][c.x] == 0)
            .expect("No path found!?")
            .into()
    }
}

fn run_path(
    map: &HeightMap,
    allow: fn(u8, u8) -> bool,
    solve: impl Fn(Point) -> bool,
) -> Option<usize> {
    let mut visited = vec![vec![false; map.data[0].len()]; map.data.len()];
    let mut queue = VecDeque::new();
    queue.push_back((map.current, Vec::new()));

    while !queue.is_empty() {
        let (current, history) = queue.pop_front().unwrap();
        if solve(current) {
            return Some(history.len());
        }

        let current_height = map.data[current.y][current.x];
        let mut check_neighbour = |x: usize, y: usize| {
            if x >= map.data[0].len()
                || y >= map.data.len()
                || visited[y][x]
                || !allow(map.data[y][x], current_height)
            {
                return;
            }

            visited[y][x] = true;
            let mut new_history = history.clone();
            new_history.push(current);
            queue.push_back((Point::new(x, y), new_history));
        };

        check_neighbour(current.x + 1, current.y);
        check_neighbour(current.x, current.y + 1);
        check_neighbour(current.x.wrapping_sub(1), current.y);
        check_neighbour(current.x, current.y.wrapping_sub(1));
    }

    None
}

#[derive(Debug, Clone)]
struct HeightMap {
    data: Vec<Vec<u8>>,
    current: Point,

    start: Point,
    end: Point,
}

fn parse(raw: &str) -> HeightMap {
    let mut out = Vec::new();
    let mut start = Point::new(0, 0);
    let mut end = Point::new(0, 0);

    for i in raw.lines() {
        let mut row = Vec::new();

        for j in i.chars() {
            match j {
                'S' => {
                    row.push(0);
                    start = Point::new(row.len() - 1, out.len());
                }
                'E' => {
                    row.push(25);
                    end = Point::new(row.len() - 1, out.len());
                }
                _ => row.push(j as u8 - 97),
            }
        }

        out.push(row);
    }

    HeightMap {
        data: out,
        current: start,
        start,
        end,
    }
}
