use std::iter::{once, repeat};

use common::{solution, Answer};
use itertools::Itertools;
use rayon::iter::{ParallelBridge, ParallelIterator};

solution!("Monkey Market", 22);

fn part_a(input: &str) -> Answer {
    let mut sum = 0;

    for num in input.lines() {
        let mut num = num.parse::<u64>().unwrap();
        (0..2000).for_each(|_| {
            next(&mut num);
        });
        sum += num;
    }

    sum.into()
}

fn part_b(input: &str) -> Answer {
    let mut buyers = Vec::new();
    for num in input.lines() {
        let mut num = num.parse::<u64>().unwrap();
        let seq = once(num)
            .chain((0..2000).map(|_| next(&mut num)))
            .collect::<Vec<_>>();
        buyers.push(seq);
    }

    let diffs = buyers
        .iter()
        .map(|buyer| {
            buyer
                .iter()
                .tuple_windows()
                .map(|(&a, &b)| (b % 10) as i8 - (a % 10) as i8)
                .collect::<Vec<_>>()
        })
        .collect::<Vec<_>>();

    let out = repeat(-9..=9)
        .take(4)
        .multi_cartesian_product()
        .map(|x| (x[0], x[1], x[2], x[3]))
        .par_bridge()
        .map(|(a, b, c, d)| {
            let mut sum = 0;

            for (diff, nums) in diffs.iter().zip(buyers.iter()) {
                if let Some(idx) = find_sequence(diff, (a, b, c, d)) {
                    sum += nums[idx + 4] % 10;
                }
            }

            sum
        })
        .max()
        .unwrap();

    out.into()
}

fn next(num: &mut u64) -> u64 {
    *num ^= *num * 64;
    *num %= 16777216;

    *num ^= *num / 32;
    *num %= 16777216;

    *num ^= *num * 2048;
    *num %= 16777216;

    *num
}

fn find_sequence(haystack: &[i8], (a, b, c, d): (i8, i8, i8, i8)) -> Option<usize> {
    haystack
        .iter()
        .tuple_windows()
        .position(|(&ax, &bx, &cx, &dx)| ax == a && bx == b && cx == c && dx == d)
}

#[cfg(test)]
mod test {
    use indoc::indoc;

    const CASE: &str = indoc! {"
        1
        10
        100
        2024
    "};

    #[test]
    fn part_a() {
        assert_eq!(super::part_a(CASE), 37327623.into());
    }

    #[test]
    fn part_b() {
        assert_eq!(super::part_b(CASE), 24.into());
    }
}
