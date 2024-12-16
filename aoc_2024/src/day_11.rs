use std::collections::HashMap;

use common::{solution, Answer};

solution!("Plutonian Pebbles", 11);

fn part_a(input: &str) -> Answer {
    solve(parse(input), 25).into()
}

fn part_b(input: &str) -> Answer {
    solve(parse(input), 75).into()
}

fn solve(nums: Vec<u64>, iters: usize) -> u64 {
    // Store the counts of each stone type
    let mut counts = HashMap::<u64, u64>::new();
    nums.into_iter()
        .for_each(|x| *counts.entry(x).or_default() += 1);

    // For each iteration, create a new count map by applying the rules for each
    // stone to get a new key and adding the previous count to it.
    for _ in 0..iters {
        let mut next = HashMap::new();
        for (stone, count) in counts {
            if stone == 0 {
                *next.entry(1).or_default() += count;
            } else if let Some((a, b)) = split_digits(stone) {
                *next.entry(a).or_default() += count;
                *next.entry(b).or_default() += count;
            } else {
                *next.entry(stone * 2024).or_default() += count;
            }
        }
        counts = next;
    }

    counts.values().sum::<u64>()
}

fn parse(input: &str) -> Vec<u64> {
    input
        .split_ascii_whitespace()
        .map(|x| x.parse().unwrap())
        .collect()
}

/// Given an integer, this function will return None if it has an odd number of
/// base 10 digits, otherwise the first half and second half of the digits will
/// be returned separately.
fn split_digits(num: u64) -> Option<(u64, u64)> {
    let digits = num.ilog10() + 1;
    let pow = u64::pow(10, digits / 2);
    (digits & 1 == 0).then(|| (num / pow, num % pow))
}

#[cfg(test)]
mod test {
    use indoc::indoc;

    const CASE: &str = indoc! {"
        125 17
    "};

    #[test]
    fn part_a() {
        assert_eq!(super::part_a(CASE), 55312.into());
    }

    #[test]
    fn part_b() {
        assert_eq!(super::part_b(CASE), 65601038650482_u64.into());
    }
}
