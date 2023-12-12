use std::collections::HashMap;

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

#[derive(Debug, Clone)]
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
        let mut field = field.chars().collect::<Vec<_>>();
        field.push('.');
        out.push(Spring { field, springs });
    }

    out
}

impl Spring {
    fn arrangements(&self) -> usize {
        fn count(
            memo: &mut HashMap<(usize, usize, usize), usize>,
            spring: &Spring,
            pos: usize,
            sequence_idx: usize,
            sequences: usize,
        ) -> usize {
            if let Some(&ret) = memo.get(&(pos, sequence_idx, sequences)) {
                return ret;
            }

            let ret = if pos == spring.field.len() {
                (sequences == spring.springs.len()) as usize
            } else if spring.field[pos] == '#' {
                count(memo, spring, pos + 1, sequence_idx + 1, sequences)
            } else if spring.field[pos] == '.' || sequences == spring.springs.len() {
                if sequences < spring.springs.len() && sequence_idx == spring.springs[sequences] {
                    count(memo, spring, pos + 1, 0, sequences + 1)
                } else if sequence_idx == 0 {
                    count(memo, spring, pos + 1, 0, sequences)
                } else {
                    0
                }
            } else {
                let hash_count = count(memo, spring, pos + 1, sequence_idx + 1, sequences);
                let mut dot_count = 0;
                if sequence_idx == spring.springs[sequences] {
                    dot_count = count(memo, spring, pos + 1, 0, sequences + 1);
                } else if sequence_idx == 0 {
                    dot_count = count(memo, spring, pos + 1, 0, sequences);
                }
                hash_count + dot_count
            };

            memo.insert((pos, sequence_idx, sequences), ret);
            ret
        }

        count(&mut HashMap::new(), &self, 0, 0, 0)
    }

    fn expand(&self) -> Self {
        let mut new_field = self.field.clone();
        *new_field.last_mut().unwrap() = '?';

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

    use super::Day12;

    const CASE: &str = indoc! {"
        ???.### 1,1,3
        .??..??...?##. 1,1,3
        ?#?#?#?#?#?#?#? 1,3,1,6
        ????.#...#... 4,1,1
        ????.######..#####. 1,6,5
        ?###???????? 3,2,1
    "};

    #[test]
    fn part_a() {
        assert_eq!(Day12.part_a(CASE), 21.into());
    }

    #[test]
    fn part_b() {
        assert_eq!(Day12.part_b(CASE), 525152.into());
    }
}
