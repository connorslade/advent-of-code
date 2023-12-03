use std::collections::HashMap;

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
            .gears
            .iter()
            .filter(|x| x.part_number)
            .map(|x| x.value)
            .sum::<u32>()
            .into()
    }

    fn part_b(&self, input: &str) -> Answer {
        parse(input)
            .ratios
            .iter()
            .filter(|(_, vals)| vals.len() == 2)
            .map(|(_, vals)| vals[0] * vals[1])
            .sum::<u32>()
            .into()
    }
}

struct ParseResult {
    gears: Vec<Gear>,
    ratios: HashMap<Pos, Vec<u32>>,
}

fn parse(input: &str) -> ParseResult {
    let mut symbols = HashMap::new();
    for (y, line) in input.lines().enumerate() {
        for (x, c) in line.char_indices() {
            if !c.is_ascii_digit() && c != '.' {
                symbols.insert(vector!(x, y), c);
            }
        }
    }

    let mut gears = Vec::new();
    let mut ratios = HashMap::new();
    for (y, line) in input.lines().enumerate() {
        for m in regex!(r"\d+").find_iter(line) {
            let value = m.as_str().parse().unwrap();

            let mut part_number = false;
            for nx in m.start().saturating_sub(1)..=m.end() {
                for ny in y.saturating_sub(1)..=y + 1 {
                    let pos = vector!(nx, ny);
                    let symbol = symbols.get(&pos);
                    part_number |= symbol.is_some();
                    
                    if symbol == Some(&'*') {
                        ratios.entry(pos).or_insert(Vec::new()).push(value);
                    }
                }
            }

            gears.push(Gear { value, part_number });
        }
    }

    ParseResult { gears, ratios }
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
