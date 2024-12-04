use std::collections::HashSet;

use aoc_lib::{direction::cardinal::Direction, matrix::Matrix};
use common::{solution, Answer};
use nd_vec::{vector, Vec2};

use polynomial::Polynomial;

solution!("Step Counter", 21);

fn part_a(input: &str) -> Answer {
    let map = parse(input);
    let mut pos = HashSet::new();
    pos.insert(map.find(Tile::Start).unwrap().num_cast::<i32>().unwrap());

    for _ in 0..64 {
        let mut new_pos = HashSet::new();

        for p in pos {
            for dir in Direction::ALL {
                let new_p = dir.advance(p);
                if !map.contains(new_p)
                    || *map.get(new_p.num_cast().unwrap()).unwrap() != Tile::Wall
                {
                    new_pos.insert(new_p);
                }
            }
        }

        pos = new_pos;
    }

    pos.len().into()
}

fn part_b(input: &str) -> Answer {
    let map = parse(input);
    let start = map.find(Tile::Start).unwrap().num_cast::<i32>().unwrap();
    let size = map.size.num_cast::<i32>().unwrap();

    let mut pos = HashSet::new();
    pos.insert(start);

    let mut points = Vec::new();
    for i in 0.. {
        let mut new_pos = HashSet::new();

        if i % size.x() == 65 {
            points.push((i, pos.len()));
            if points.len() >= 3 {
                break;
            }
        }

        for p in pos {
            for dir in Direction::ALL {
                let new_p = dir.advance(p);
                let mapped = map_pos(new_p, size);
                if *map.get(mapped).unwrap() != Tile::Wall {
                    new_pos.insert(new_p);
                }
            }
        }

        pos = new_pos;
    }

    let x = points.iter().map(|x| x.0 as f64).collect::<Vec<_>>();
    let y = points.iter().map(|x| x.1 as f64).collect::<Vec<_>>();
    let poly = Polynomial::lagrange(&x, &y).unwrap();

    poly.eval(26501365.0).round().into()
}

fn map_pos(pos: Vec2<i32>, size: Vec2<i32>) -> Vec2<usize> {
    let mut mapped = pos;
    mapped = vector!((size.x() + mapped.x() % size.x()) % size.x(), mapped.y());
    mapped = vector!(mapped.x(), (size.y() + mapped.y() % size.y()) % size.y());
    mapped.num_cast().unwrap()
}

#[derive(Debug, PartialEq, Eq, Copy, Clone)]
enum Tile {
    Garden,
    Wall,
    Start,
}

fn parse(input: &str) -> Matrix<Tile> {
    Matrix::new_chars(input, |x| match x {
        '#' => Tile::Wall,
        '.' => Tile::Garden,
        'S' => Tile::Start,
        _ => panic!("Invalid input"),
    })
}

#[cfg(test)]
mod test {
    use indoc::indoc;

    const CASE: &str = indoc! {"
        ...........
        .....###.#.
        .###.##..#.
        ..#.#...#..
        ....#.#....
        .##..S####.
        .##..#...#.
        .......##..
        .##.#.####.
        .##..##.##.
        ...........
    "};

    #[test]
    fn part_a() {
        assert_eq!(super::part_a(CASE), 4056.into());
    }
}
