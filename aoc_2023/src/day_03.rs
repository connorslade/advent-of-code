use std::collections::{HashMap, HashSet};

use nd_vec::{vector, Vec2};

use common::{Answer, Solution};

type Pos = Vec2<usize>;

pub struct Day03;

impl Solution for Day03 {
    fn name(&self) -> &'static str {
        "Gear Ratios"
    }

    fn part_a(&self, input: &str) -> Answer {
        parse(input)
            .0
            .iter()
            .filter(|x| x.part_number)
            .map(|x| x.value)
            .sum::<u32>()
            .into()
    }

    fn part_b(&self, input: &str) -> Answer {
        parse(input)
            .1
            .iter()
            .filter(|(_, vals)| vals.len() == 2)
            .map(|(_, vals)| vals[0] * vals[1])
            .sum::<u32>()
            .into()
    }
}

fn parse(input: &str) -> (Vec<Gear>, HashMap<Pos, Vec<u32>>) {
    let chars = input
        .lines()
        .map(|line| line.chars().collect::<Vec<_>>())
        .collect::<Vec<_>>();

    let mut symbols = HashSet::new();
    let mut ratios = HashMap::new();
    for (y, line) in chars.iter().enumerate() {
        for (x, c) in line.iter().enumerate() {
            if !"0123456789.".contains(*c) {
                symbols.insert(vector!(x, y));
            }
        }
    }

    let mut gears = Vec::new();
    for (y, line) in chars.iter().enumerate() {
        let mut pos = None;

        let mut check = |pos, x| {
            if let Some(pos) = pos {
                let [start, end] = [pos, x - 1];
                let value = line[start..=end]
                    .iter()
                    .collect::<String>()
                    .parse()
                    .unwrap();

                let mut part_number = false;
                for nx in (start as isize - 1)..=(end as isize + 1) {
                    for ny in (y as isize - 1)..=(y as isize + 1) {
                        if nx < 0 || ny < 0 {
                            continue;
                        }

                        let pos = vector!(nx as usize, ny as usize);
                        part_number |= symbols.contains(&pos);

                        if symbols.contains(&pos) {
                            let symbol = chars[ny as usize][nx as usize];
                            if symbol == '*' {
                                ratios.entry(pos).or_insert(Vec::new()).push(value);
                            }
                        }
                    }
                }

                gears.push(Gear {
                    value,
                    part_number,
                });
            }
        };

        let mut x = 0;
        while x < line.len() {
            if line[x].is_numeric() {
                if pos.is_none() {
                    pos = Some(x);
                }
            } else {
                check(pos, x);
                pos = None;
            }
            x += 1;
        }
        check(pos, x);
    }

    (gears, ratios)
}

#[derive(Debug)]
struct Gear {
    value: u32,
    part_number: bool,
}

#[cfg(test)]
mod test {
    use common::Solution;
    use indoc::indoc;

    use super::Day03;

    const CASE: &str = indoc! {"
        467..114..
        ...*......
        ..35..633.
        ......#...
        617*......
        .....+.58.
        ..592.....
        ......755.
        ...$.*....
        .664.598..
    "};

    #[test]
    fn part_a() {
        assert_eq!(Day03.part_a(CASE), 4361.into());
    }

    #[test]
    fn part_b() {
        assert_eq!(Day03.part_b(CASE), 467835.into());
    }
}
