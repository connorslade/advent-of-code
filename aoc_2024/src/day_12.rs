use std::{collections::HashSet, convert::identity};

use aoc_lib::{direction::cardinal::Direction, matrix::Grid};
use common::{solution, Answer};
use itertools::Itertools;
use nd_vec::{vector, Vec2};

solution!("Garden Groups", 12);

fn part_a(input: &str) -> Answer {
    let mut garden = Garden::parse(input);
    garden
        .points()
        .filter_map(|x| garden.flood(x))
        .map(|(area, perimeter)| area.len() * perimeter)
        .sum::<usize>()
        .into()
}

fn part_b(input: &str) -> Answer {
    let mut garden = Garden::parse(input);

    let mut sum = 0;
    for (area, _) in garden.points().filter_map(|x| garden.flood(x)) {
        let mut corners = 0;

        for &point in area.iter() {
            // Count convex corners by checking to see that the wall is not in
            // any cardinal direction and a direction orthogonal to that
            for a in Direction::ALL {
                corners += (!area.contains(&a.wrapping_advance(point))
                    && !area.contains(&a.turn_right().wrapping_advance(point)))
                    as u32;
            }

            // Count the concave angles by looking for when both the orthogonal
            // directions are in the area, but not the diagonal between them.
            for a in Direction::ALL {
                let b = a.turn_right();
                corners += (area.contains(&a.wrapping_advance(point))
                    && area.contains(&b.wrapping_advance(point))
                    && !area.contains(&b.wrapping_advance(a.wrapping_advance(point))))
                    as u32;
            }
        }

        sum += area.len() as u32 * corners;
    }

    sum.into()
}

struct Garden {
    matrix: Grid<char>,

    seen: HashSet<Vec2<usize>>,
}

impl Garden {
    fn parse(input: &str) -> Self {
        let matrix = Grid::new(input, identity);
        Self {
            matrix,
            seen: HashSet::new(),
        }
    }

    fn points(&self) -> impl Iterator<Item = Vec2<usize>> {
        let size = self.matrix.size;
        (0..size.x())
            .cartesian_product(0..size.y())
            .map(|(x, y)| vector!(x, y))
    }

    fn flood(&mut self, start: Vec2<usize>) -> Option<(HashSet<Vec2<usize>>, usize)> {
        if !self.seen.insert(start) {
            return None;
        }

        let mut area = HashSet::new();
        let mut perimeter = 0;

        let mut queue = Vec::new();
        let plant = self.matrix.get(start).unwrap();

        area.insert(start);
        queue.push(start);

        while let Some(pos) = queue.pop() {
            for next in Direction::ALL.map(|x| x.wrapping_advance(pos)) {
                if !self.matrix.contains(next) {
                    perimeter += 1;
                    continue;
                }

                if self.matrix.get(next).unwrap() == plant {
                    if self.seen.insert(next) {
                        area.insert(next);
                        queue.push(next);
                    }
                } else {
                    perimeter += 1;
                }
            }
        }

        Some((area, perimeter))
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
