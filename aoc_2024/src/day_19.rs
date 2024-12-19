use std::collections::HashMap;

use common::{solution, Answer};

solution!("Linen Layout", 19);

fn part_a(input: &str) -> Answer {
    let problem = Problem::parse(input);
    let mut sum = 0;

    for i in 0..problem.needed.len() {
        if problem.possible(i) {
            sum += 1;
        }
    }

    sum.into()
}

fn part_b(input: &str) -> Answer {
    let problem = Problem::parse(input);
    let mut sum = 0;

    for i in 0..problem.needed.len() {
        sum += problem.ways(i);
    }

    sum.into()
}

struct Problem {
    sources: Vec<String>,
    needed: Vec<String>,
}

impl Problem {
    fn parse(input: &str) -> Self {
        let (sources, needed) = input.split_once("\n\n").unwrap();
        let sources = sources.split(", ").map(|x| x.to_owned()).collect();
        let needed = needed.lines().map(|x| x.to_owned()).collect();

        Self { sources, needed }
    }

    fn possible(&self, design: usize) -> bool {
        fn _inner<'a>(
            memo: &mut HashMap<&'a str, bool>,
            expected: &'a str,
            sources: &[String],
        ) -> bool {
            if let Some(&cache) = memo.get(expected) {
                return cache;
            }

            if expected.len() == 0 {
                memo.insert(expected, true);
                return true;
            }

            for source in sources {
                if expected.len() >= source.len()
                    && expected.starts_with(source)
                    && _inner(memo, &expected[source.len()..], &sources)
                {
                    memo.insert(expected, true);
                    return true;
                }
            }

            memo.insert(expected, false);
            false
        }

        _inner(&mut HashMap::new(), &self.needed[design], &self.sources)
    }

    fn ways(&self, design: usize) -> u64 {
        fn _inner<'a>(
            memo: &mut HashMap<&'a str, u64>,
            expected: &'a str,
            sources: &[String],
        ) -> u64 {
            if let Some(&cache) = memo.get(expected) {
                return cache;
            }

            if expected.len() == 0 {
                return 1;
            }

            let mut ways = 0;
            for source in sources {
                if expected.len() >= source.len() && expected.starts_with(source) {
                    ways += _inner(memo, &expected[source.len()..], &sources);
                }
            }

            memo.insert(expected, ways);
            ways
        }

        _inner(&mut HashMap::new(), &self.needed[design], &self.sources)
    }
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
