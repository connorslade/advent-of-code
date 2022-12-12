use std::collections::VecDeque;

use crate::{problem, Solution};

pub struct Day12;

impl Solution for Day12 {
    fn name(&self) -> &'static str {
        "Hill Climbing Algorithm"
    }

    fn part_a(&self) -> String {
        let raw = problem::load(2022, 12);
        let map = parse(&raw);
        run_path(&map, |a, b| a > b + 1).to_string()
    }

    fn part_b(&self) -> String {
        let raw = problem::load(2022, 12);
        let map = parse(&raw);
        let mut min = usize::MAX;

        for x in 0..map.data.len() {
            for y in 0..map.data[x].len() {
                if map.data[x][y] != 0 {
                    continue;
                }

                let mut new_map = map.clone();
                new_map.start = Point::new(y, x);
                new_map.current = new_map.start;
                let distance = run_path(&new_map, |a, b| a > b + 1);
                min = min.min(distance);
            }
        }

        min.to_string()
    }
}

fn run_path(map: &HeightMap, disallow: fn(u8, u8) -> bool) -> usize {
    let mut visited = vec![vec![false; map.data[0].len()]; map.data.len()];
    let mut queue = VecDeque::new();
    queue.push_back((map.current, Vec::new()));
    // dbg!(map.current);

    while !queue.is_empty() {
        let (current, history) = queue.pop_front().unwrap();
        // println!("({}, {})", current.x, current.y);
        if current == map.end {
            return history.len();
        }

        let current_height = map.data[current.y][current.x];

        let mut check_neighbour = |x: usize, y: usize| {
            if x >= map.data[0].len() || y >= map.data.len() {
                return;
            }

            let height = map.data[y][x];
            if disallow(height, current_height) {
                return;
            }

            if visited[y][x] {
                return;
            }

            visited[y][x] = true;
            let mut new_history = history.clone();
            new_history.push(current);
            queue.push_back((Point::new(x, y), new_history));
        };

        check_neighbour(current.x + 1, current.y);
        check_neighbour(current.x, current.y + 1);
        check_neighbour(current.x - 1, current.y);
        check_neighbour(current.x, current.y - 1);
    }

    // unreachable!()
    usize::MAX
}

#[derive(Debug, Clone)]
struct HeightMap {
    data: Vec<Vec<u8>>,
    current: Point,

    start: Point,
    end: Point,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
struct Point {
    x: usize,
    y: usize,
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

impl Point {
    fn new(x: usize, y: usize) -> Self {
        Self { x, y }
    }
}
