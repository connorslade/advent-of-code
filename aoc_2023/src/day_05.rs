use common::{Answer, Solution};

pub struct Day05;

impl Solution for Day05 {
    fn name(&self) -> &'static str {
        "If You Give A Seed A Fertilizer"
    }

    fn part_a(&self, input: &str) -> Answer {
        let seeds = parse(input);

        let mut min = u32::MAX;
        for mut seed in seeds.seeds {
            for map in &seeds.maps {
                seed = map.get(seed);
            }
            min = min.min(seed);
        }

        min.into()
    }

    fn part_b(&self, input: &str) -> Answer {
        let seeds = parse(input);

        let mut min = u32::MAX;
        for seed in seeds.seeds.chunks_exact(2) {
            for mut seed in seed[0]..seed[0] + seed[1] {
                for map in &seeds.maps {
                    seed = map.get(seed);
                }

                min = min.min(seed);
            }
        }

        min.into()
    }
}

#[derive(Debug)]
struct Map {
    ranges: Vec<Range>,
}

#[derive(Debug)]
struct Range {
    start: u32,
    end: u32,
    length: u32,
}

struct ParseResult {
    maps: Vec<Map>,
    seeds: Vec<u32>,
}

fn parse(input: &str) -> ParseResult {
    let mut maps = Vec::new();

    let mut sections = input.split("\n\n");

    let seeds = sections
        .next()
        .unwrap()
        .split_whitespace()
        .skip(1)
        .map(|s| s.parse().unwrap())
        .collect();

    for section in sections.into_iter().filter(|x| !x.is_empty()) {
        let lines = section.lines();
        let mut ranges = Vec::new();

        for line in lines.skip(1) {
            let mut parts = line.split_whitespace();

            let end = parts.next().unwrap().parse().unwrap();
            let start = parts.next().unwrap().parse().unwrap();
            let length = parts.next().unwrap().parse().unwrap();

            ranges.push(Range { start, end, length });
        }

        maps.push(Map { ranges });
    }

    ParseResult { maps, seeds }
}

impl Map {
    fn get(&self, value: u32) -> u32 {
        for range in &self.ranges {
            if range.start <= value && value <= range.start + range.length {
                let diff = value - range.start;
                return range.end + diff;
            }
        }

        value
    }
}

#[cfg(test)]
mod test {
    use common::Solution;
    use indoc::indoc;

    use super::Day05;

    const CASE: &str = indoc! {"
        seeds: 79 14 55 13

        seed-to-soil map:
        50 98 2
        52 50 48

        soil-to-fertilizer map:
        0 15 37
        37 52 2
        39 0 15

        fertilizer-to-water map:
        49 53 8
        0 11 42
        42 0 7
        57 7 4

        water-to-light map:
        88 18 7
        18 25 70

        light-to-temperature map:
        45 77 23
        81 45 19
        68 64 13

        temperature-to-humidity map:
        0 69 1
        1 0 69

        humidity-to-location map:
        60 56 37
        56 93 4

    "};

    #[test]
    fn part_a() {
        assert_eq!(Day05.part_a(CASE), 35.into());
    }

    #[test]
    fn part_b() {
        assert_eq!(Day05.part_b(CASE), 46.into());
    }
}
