use std::collections::{BinaryHeap, HashMap, HashSet};

use aoc_lib::{direction::cardinal::Direction, matrix::Matrix};
use common::{solution, Answer};
use nd_vec::{vector, Vec2};

solution!("Reindeer Maze", 16);

#[derive(PartialEq, Eq)]
enum Tile {
    Empty,
    Wall,
    Start,
    End,
}

fn part_a(input: &str) -> Answer {
    let map = Matrix::new_chars(input, |c| match c {
        '.' => Tile::Empty,
        '#' => Tile::Wall,
        'S' => Tile::Start,
        'E' => Tile::End,
        _ => panic!(),
    });
    let start = map.find(Tile::Start).unwrap();

    // find minimum score needed to path from S to E
    // - Move foreword (+1)
    // - rotate left or right (+1000)

    #[derive(PartialEq, Eq)]
    struct Item {
        pos: Vec2<usize>,
        dir: Direction,
        score: u32,
    }

    impl Ord for Item {
        fn cmp(&self, other: &Self) -> std::cmp::Ordering {
            other.score.cmp(&self.score)
        }
    }

    impl PartialOrd for Item {
        fn partial_cmp(&self, other: &Self) -> Option<std::cmp::Ordering> {
            Some(other.score.cmp(&self.score))
        }
    }

    // todo: try bin heap
    let mut queue = BinaryHeap::new();
    let mut seen = HashSet::new();

    queue.push(Item {
        pos: start,
        dir: Direction::Right,
        score: 0,
    });

    while let Some(Item { pos, dir, score }) = queue.pop() {
        if !seen.insert((pos, dir)) {
            continue;
        }

        if map[pos] == Tile::End {
            return score.into();
        }

        // Move foreword
        let next = dir.wrapping_advance(pos);
        if map.contains(next) && map[next] != Tile::Wall {
            queue.push(Item {
                pos: next,
                dir,
                score: score + 1,
            });
        }

        queue.push(Item {
            pos,
            dir: dir.turn_left(),
            score: score + 1000,
        });

        queue.push(Item {
            pos,
            dir: dir.turn_right(),
            score: score + 1000,
        });
    }

    unreachable!()
}

fn part_b(input: &str) -> Answer {
    let map = Matrix::new_chars(input, |c| match c {
        '.' => Tile::Empty,
        '#' => Tile::Wall,
        'S' => Tile::Start,
        'E' => Tile::End,
        _ => panic!(),
    });
    let start = map.find(Tile::Start).unwrap();

    // find minimum score needed to path from S to E
    // - Move foreword (+1)
    // - rotate left or right (+1000)

    #[derive(PartialEq, Eq)]
    struct Item {
        pos: Vec2<usize>,
        dir: Direction,
        score: u32,

        path: Vec<Vec2<usize>>,
    }

    impl Ord for Item {
        fn cmp(&self, other: &Self) -> std::cmp::Ordering {
            other.score.cmp(&self.score)
        }
    }

    impl PartialOrd for Item {
        fn partial_cmp(&self, other: &Self) -> Option<std::cmp::Ordering> {
            Some(other.score.cmp(&self.score))
        }
    }

    // todo: try bin heap
    let mut queue = BinaryHeap::new();
    let mut seen = HashMap::new();

    let mut on_best = HashSet::<Vec2<usize>>::new();

    queue.push(Item {
        pos: start,
        dir: Direction::Right,
        score: 0,

        path: vec![start],
    });

    let mut best = None;

    while let Some(Item {
        pos,
        dir,
        score,
        path,
    }) = queue.pop()
    {
        if let Some(&prev) = seen.get(&(pos, dir)) {
            if score > prev {
                continue;
            }
        } else {
            seen.insert((pos, dir), score);
        }

        if map[pos] == Tile::End {
            if let Some(real_best) = best {
                dbg!(score, real_best);
                if score == real_best {
                    on_best.extend(path.iter());
                }
            } else {
                best = Some(score);
                on_best.extend(path.iter());
            }

            continue;
        }

        // Move foreword
        let next = dir.wrapping_advance(pos);
        let mut next_path = path.clone();
        next_path.push(next);
        if map.contains(next) && map[next] != Tile::Wall {
            queue.push(Item {
                pos: next,
                dir,
                score: score + 1,
                path: next_path,
            });
        }

        queue.push(Item {
            pos,
            dir: dir.turn_left(),
            score: score + 1000,
            path: path.clone(),
        });

        queue.push(Item {
            pos,
            dir: dir.turn_right(),
            score: score + 1000,
            path,
        });
    }

    for y in 0..map.size.y() {
        for x in 0..map.size.x() {
            let pos = vector!(x, y);
            print!(
                "{}",
                match map[pos] {
                    Tile::Empty if on_best.contains(&pos) => '@',
                    Tile::Empty => '.',
                    Tile::Wall => '#',
                    Tile::Start => 'S',
                    Tile::End => 'E',
                }
            );
        }
        println!();
    }

    on_best.len().into()
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
