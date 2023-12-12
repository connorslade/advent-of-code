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
            .map(|s| {
                let mut s = s.clone();
                s.field.push('.');
                s.arrangements_b()
            })
            .sum::<usize>()
            .into()
    }

    fn part_b(&self, input: &str) -> Answer {
        parse(input)
            .par_iter()
            .map(|s| s.expand().arrangements_b())
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
        let field = field.chars().collect::<Vec<_>>();
        out.push(Spring { field, springs });
    }

    out
}

impl Spring {
    fn arrangements_b(&self) -> usize {
        fn count(
            memo: &mut HashMap<(usize, usize, usize), usize>,
            field: &[char],
            counts: &[usize],
            pos: usize,
            current_count: usize,
            count_pos: usize,
        ) -> usize {
            if let Some(&ret) = memo.get(&(pos, current_count, count_pos)) {
                return ret;
            }

            let mut ret = 0;

            if pos == field.len() {
                ret = if count_pos == counts.len() { 1 } else { 0 };
            } else if field[pos] == '#' {
                ret = count(memo, field, counts, pos + 1, current_count + 1, count_pos);
            } else if field[pos] == '.' || count_pos == counts.len() {
                if count_pos < counts.len() && current_count == counts[count_pos] {
                    ret = count(memo, field, counts, pos + 1, 0, count_pos + 1);
                } else if current_count == 0 {
                    ret = count(memo, field, counts, pos + 1, 0, count_pos);
                }
            } else {
                let hash_count = count(memo, field, counts, pos + 1, current_count + 1, count_pos);
                let mut dot_count = 0;
                if current_count == counts[count_pos] {
                    dot_count = count(memo, field, counts, pos + 1, 0, count_pos + 1);
                } else if current_count == 0 {
                    dot_count = count(memo, field, counts, pos + 1, 0, count_pos);
                }
                ret = hash_count + dot_count;
            }

            memo.insert((pos, current_count, count_pos), ret);
            ret
        }

        let mut memo = HashMap::new();
        count(&mut memo, &self.field, &self.springs, 0, 0, 0)
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
