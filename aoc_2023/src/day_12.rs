use std::collections::HashMap;

use common::{solution, Answer};

solution!("Hot Springs", (2023, 04));

fn part_a(input: &str) -> Answer {
    parse(input)
        .iter()
        .map(|s| s.arrangements())
        .sum::<usize>()
        .into()
}

fn part_b(input: &str) -> Answer {
    parse(input)
        .iter()
        .map(|s| s.expand().arrangements())
        .sum::<usize>()
        .into()
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
            block: usize,
            sequences: usize,
        ) -> usize {
            if let Some(&res) = memo.get(&(pos, block, sequences)) {
                return res;
            }

            let mut res = 0;
            if pos == spring.field.len() {
                res = (sequences == spring.springs.len()) as usize;
            } else if spring.field[pos] == '#' {
                res = count(memo, spring, pos + 1, block + 1, sequences)
            } else if spring.field[pos] == '.' || sequences == spring.springs.len() {
                if sequences < spring.springs.len() && block == spring.springs[sequences] {
                    res = count(memo, spring, pos + 1, 0, sequences + 1)
                } else if block == 0 {
                    res = count(memo, spring, pos + 1, 0, sequences)
                }
            } else {
                res += count(memo, spring, pos + 1, block + 1, sequences);
                if block == spring.springs[sequences] {
                    res += count(memo, spring, pos + 1, 0, sequences + 1)
                } else if block == 0 {
                    res += count(memo, spring, pos + 1, 0, sequences)
                }
            }

            memo.insert((pos, block, sequences), res);
            res
        }

        count(&mut HashMap::new(), self, 0, 0, 0)
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
    use indoc::indoc;

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
        assert_eq!(super::part_a(CASE), 21.into());
    }

    #[test]
    fn part_b() {
        assert_eq!(super::part_b(CASE), 525152.into());
    }
}
