use std::collections::VecDeque;

use common::{solution, Answer};
use nd_vec::vector;

type Point = nd_vec::Vec2<usize>;

solution!("Hill Climbing Algorithm", (2022, 00));

fn part_a(input: &str) -> Answer {
    let map = parse(input);

    run_path(&map, |a, b| a <= b + 1, |c| c == map.end)
        .unwrap()
        .into()
}

fn part_b(input: &str) -> Answer {
    let mut map = parse(input);
    map.start = map.end;
    map.current = map.start;

    run_path(&map, |a, b| b <= a + 1, |c| map.data[c.y()][c.x()] == 0)
        .expect("No path found!?")
        .into()
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

        let current_height = map.data[current.y()][current.x()];
        let mut check_neighbor = |x: usize, y: usize| {
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
            queue.push_back((vector!(x, y), new_history));
        };

        let (cx, cy) = (current.x(), current.y());
        check_neighbor(cx + 1, cy);
        check_neighbor(cx, cy + 1);
        check_neighbor(cx.wrapping_sub(1), cy);
        check_neighbor(cx, cy.wrapping_sub(1));
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
    let mut start = vector!(0, 0);
    let mut end = vector!(0, 0);

    for i in raw.lines() {
        let mut row = Vec::new();

        for j in i.chars() {
            match j {
                'S' => {
                    row.push(0);
                    start = vector!(row.len() - 1, out.len());
                }
                'E' => {
                    row.push(25);
                    end = vector!(row.len() - 1, out.len());
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
