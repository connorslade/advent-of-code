use common::{Answer, Solution};
use rayon::iter::{IntoParallelRefIterator, ParallelIterator};

pub struct Day12;

impl Solution for Day12 {
    fn name(&self) -> &'static str {
        "Hot Springs"
    }

    fn part_a(&self, input: &str) -> Answer {
        parse(input)
            .iter()
            .map(|s| s.arrangements())
            .sum::<usize>()
            .into()
    }

    fn part_b(&self, input: &str) -> Answer {
        parse(input)
            .par_iter()
            .map(|s| s.expand().arrangements())
            .sum::<usize>()
            .into()
    }
}

#[derive(Debug)]
struct Spring {
    field: Vec<char>,
    springs: Vec<usize>,
}

fn parse(input: &str) -> Vec<Spring> {
    let mut out = Vec::new();

    for line in input.lines() {
        let (field, springs) = line.split_once(' ').unwrap();
        let springs = springs
            .split(',')
            .map(|s| s.parse().unwrap())
            .collect::<Vec<_>>();

        out.push(Spring {
            field: field.chars().collect(),
            springs,
        });
    }

    out
}

impl Spring {
    // In this example, the number of possible arrangements for each row is:

    // ???.### 1,1,3 - 1 arrangement
    // .??..??...?##. 1,1,3 - 4 arrangements
    // ?#?#?#?#?#?#?#? 1,3,1,6 - 1 arrangement
    // ????.#...#... 4,1,1 - 1 arrangement
    // ????.######..#####. 1,6,5 - 4 arrangements
    // ?###???????? 3,2,1 - 10 arrangements
    fn arrangements(&self) -> usize {
        let unknown = self.field.iter().filter(|&&c| c == '?').count();
        let mut count = 0;
        let mut valid = 0;

        while count < (2 << unknown - 1) {
            count += 1;

            // Create new field using the bits of count to determine which unknowns are set
            let mut field = self.field.clone();
            let mut i = 0;
            for c in field.iter_mut() {
                if *c == '?' {
                    *c = if count & (1 << i) != 0 { '#' } else { '.' };
                    i += 1;
                }
            }

            // Find each spring and how long it is
            let mut springs = Vec::new();
            let mut spring = 0;
            for c in field.iter() {
                if *c == '#' {
                    spring += 1;
                } else if spring > 0 {
                    springs.push(spring);
                    spring = 0;
                }
            }

            if spring > 0 {
                springs.push(spring);
            }

            if springs == self.springs {
                valid += 1;
            }
        }

        valid
    }

    fn expand(&self) -> Self {
        let mut new_field = self.field.clone();
        new_field.push('?');

        Self {
            field: new_field.repeat(5),
            springs: self.springs.repeat(5),
        }
    }
}

#[cfg(test)]
mod test {
    use common::Solution;
    use indoc::indoc;

    use super::{Day12, Spring};

    const CASE: &str = indoc! {"
        ???.### 1,1,3
        .??..??...?##. 1,1,3
        ?#?#?#?#?#?#?#? 1,3,1,6
        ????.#...#... 4,1,1
        ????.######..#####. 1,6,5
        ?###???????? 3,2,1
    "};

    #[test]
    fn test() {
        let sprint = Spring {
            field: ".??..??...?##.".chars().collect(),
            springs: vec![1, 1, 3],
        };
        dbg!(sprint.arrangements());
    }

    #[test]
    fn part_a() {
        assert_eq!(Day12.part_a(CASE), 21.into());
    }

    #[test]
    fn part_b() {
        assert_eq!(Day12.part_b(CASE), 525152.into());
    }
}
