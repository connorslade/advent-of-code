use std::{collections::HashSet, convert::identity};

use aoc_lib::{direction::cardinal::Direction, matrix::Matrix};
use common::{solution, Answer};
use nd_vec::Vec2;

solution!("Garden Groups", 12);

fn part_a(input: &str) -> Answer {
    let mut garden = Garden::parse(input);

    let mut sum = 0;

    for pos in garden.matrix.clone().iter().map(|(pos, _)| pos) {
        let (area, perimeter) = garden.flood(pos);
        sum += area * perimeter;
    }

    sum.into()
}

fn part_b(input: &str) -> Answer {
    let mut garden = Garden::parse(input);

    let mut sum = 0;

    for pos in garden.matrix.clone().iter().map(|(pos, _)| pos) {
        let plant = *garden.matrix.get(pos).unwrap();
        let (area, perimeter) = garden.flood_b(pos);
        if perimeter.is_empty() {
            continue;
        }

        let mut corners = 0;

        for &point in area.iter() {
            for (a, b) in [
                (Direction::Up, Direction::Right),
                (Direction::Right, Direction::Down),
                (Direction::Down, Direction::Left),
                (Direction::Left, Direction::Up),
            ] {
                // if a and b are both not in area +1
                if !area.contains(&a.advance(point)) && !area.contains(&b.advance(point)) {
                    corners += 1;
                }
            }

            for (a, b) in [
                (Direction::Up, Direction::Right),
                (Direction::Right, Direction::Down),
                (Direction::Down, Direction::Left),
                (Direction::Left, Direction::Up),
            ] {
                let e = a.as_vector::<i32>() + b.as_vector();
                if area.contains(&a.advance(point))
                    && area.contains(&b.advance(point))
                    && !area.contains(&(point + e))
                {
                    corners += 1;
                }
            }
        }

        println!("{} * {corners} [{plant}]", area.len());
        sum += area.len() * corners;
    }

    sum.into()
}

struct Garden {
    matrix: Matrix<char>,

    seen: HashSet<Vec2<usize>>,
}

impl Garden {
    fn parse(input: &str) -> Self {
        let matrix = Matrix::new_chars(input, identity);
        Self {
            matrix,
            seen: HashSet::new(),
        }
    }

    // -> (area, perimeter)
    fn flood(&mut self, start: Vec2<usize>) -> (u32, u32) {
        let (mut area, mut perimeter) = (1, 0);
        let plant = self.matrix.get(start).unwrap();

        let mut queue = Vec::new();

        if !self.seen.insert(start) {
            return (0, 0);
        }
        queue.push(start);

        while let Some(pos) = queue.pop() {
            for dir in Direction::ALL.into_iter() {
                let Some(next) = dir.try_advance(pos) else {
                    perimeter += 1;
                    continue;
                };
                if !self.matrix.contains(next) {
                    perimeter += 1;
                    continue;
                }

                if self.matrix.get(next).unwrap() == plant {
                    if self.seen.insert(next) {
                        area += 1;
                        queue.push(next);
                    }
                } else {
                    perimeter += 1
                }
            }
        }

        (area, perimeter)
    }

    fn flood_b(&mut self, start: Vec2<usize>) -> (HashSet<Vec2<i32>>, HashSet<Vec2<i32>>) {
        let (mut area, mut perimeter) = (HashSet::new(), HashSet::new());
        area.insert(start.try_cast::<i32>().unwrap());
        let plant = self.matrix.get(start).unwrap();

        let mut queue = Vec::new();

        if !self.seen.insert(start) {
            return (HashSet::new(), HashSet::new());
        }
        queue.push(start);

        while let Some(pos) = queue.pop() {
            for dir in Direction::ALL.into_iter() {
                let Some(next) = dir.try_advance(pos) else {
                    perimeter.insert(dir.advance(pos.try_cast::<i32>().unwrap()));
                    continue;
                };
                if !self.matrix.contains(next) {
                    perimeter.insert(next.try_cast::<i32>().unwrap());
                    continue;
                }

                if self.matrix.get(next).unwrap() == plant {
                    if self.seen.insert(next) {
                        area.insert(next.try_cast::<i32>().unwrap());
                        queue.push(next);
                    }
                } else {
                    perimeter.insert(next.try_cast::<i32>().unwrap());
                }
            }
        }

        (area, perimeter)
    }
}

#[cfg(test)]
mod test {
    use indoc::indoc;

    const CASE: &str = indoc! {"
        RRRRIICCFF
        RRRRIICCCF
        VVRRRCCFFF
        VVRCCCJFFF
        VVVVCJJCFE
        VVIVCCJJEE
        VVIIICJJEE
        MIIIIIJJEE
        MIIISIJEEE
        MMMISSJEEE
    "};

    #[test]
    fn part_a() {
        assert_eq!(super::part_a(CASE), 1930.into());
    }

    #[test]
    fn part_b() {
        assert_eq!(super::part_b(CASE), 1206.into());
    }
}
