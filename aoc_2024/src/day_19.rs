use std::collections::HashMap;

use common::{solution, Answer};

solution!("Linen Layout", 19);

fn part_a(input: &str) -> Answer {
    let problem = Onsen::parse(input);
    problem.possible().into()
}

fn part_b(input: &str) -> Answer {
    let problem = Onsen::parse(input);
    problem.ways().into()
}

struct Onsen<'a> {
    segments: Vec<&'a str>,
    towels: Vec<&'a str>,
}

impl<'a> Onsen<'a> {
    fn parse(input: &'a str) -> Self {
        let (sources, needed) = input.split_once("\n\n").unwrap();
        let segments = sources.split(", ").collect();
        let towels = needed.lines().collect();

        Self { segments, towels }
    }

    /// Returns the number of possible towel designs by counting all the towels
    /// that can be made a non-zero number of ways.
    fn possible(&self) -> usize {
        self.towels
            .iter()
            .filter(|x| count_ways(&mut HashMap::new(), x, &self.segments) != 0)
            .count()
    }

    /// Here we just sum up the number of ways each towel can be made.
    fn ways(&self) -> u64 {
        self.towels
            .iter()
            .map(|x| count_ways(&mut HashMap::new(), x, &self.segments))
            .sum()
    }
}

fn count_ways<'a>(memo: &mut HashMap<&'a str, u64>, expected: &'a str, sources: &[&'a str]) -> u64 {
    if let Some(&cache) = memo.get(expected) {
        return cache;
    }

    // If there is no more towel to find designs for, we have found one way to
    // make the towel.
    if expected.is_empty() {
        return 1;
    }

    // Otherwise, we will sum up the number of ways the towel can be made from
    // adding each of the available segments to the current towel, but only the
    // ones that match the current pattern.
    let mut ways = 0;
    for source in sources {
        if expected.len() >= source.len() && expected.starts_with(source) {
            ways += count_ways(memo, &expected[source.len()..], sources);
        }
    }

    // Memoization!!! This is what allows us to avoid evaluating huge segments
    // of the tree and get good performance.
    memo.insert(expected, ways);
    ways
}

#[cfg(test)]
mod test {
    use indoc::indoc;

    const CASE: &str = indoc! {"
        r, wr, b, g, bwu, rb, gb, br

        brwrr
        bggr
        gbbr
        rrbgbr
        ubwu
        bwurrg
        brgr
        bbrgwb
    "};

    #[test]
    fn part_a() {
        assert_eq!(super::part_a(CASE), 6.into());
    }

    #[test]
    fn part_b() {
        assert_eq!(super::part_b(CASE), 16.into());
    }
}
