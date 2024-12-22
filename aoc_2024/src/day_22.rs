use std::iter::repeat;

use common::{solution, Answer};
use itertools::Itertools;
use rayon::iter::{ParallelBridge, ParallelIterator};

solution!("Monkey Market", 22);

fn part_a(input: &str) -> Answer {
    let mut sum = 0;

    for num in input.lines() {
        let mut num = num.parse::<u64>().unwrap();

        for _ in 0..2000 {
            num ^= num * 64;
            num %= 16777216;

            num ^= num / 32;
            num %= 16777216;

            num ^= num * 2048;
            num %= 16777216;
        }

        sum += num;
    }

    sum.into()
}

fn part_b(input: &str) -> Answer {
    let mut buyers = Vec::new();
    let mut diffs = Vec::new();

    for num in input.lines() {
        let mut num = num.parse::<u64>().unwrap();
        let mut seq = vec![num];

        for _ in 0..2000 {
            num ^= num * 64;
            num %= 16777216;

            num ^= num / 32;
            num %= 16777216;

            num ^= num * 2048;
            num %= 16777216;

            seq.push(num);
        }

        buyers.push(seq);
    }

    for buyer in buyers.iter() {
        let mut diff = Vec::new();
        for (&a, &b) in buyer.iter().tuple_windows() {
            diff.push((b % 10) as i8 - (a % 10) as i8);
        }
        diffs.push(diff);
    }

    let out = repeat(-9..=9)
        .take(4)
        .multi_cartesian_product()
        .par_bridge()
        .map(|x| {
            let (a, b, c, d) = (x[0], x[1], x[2], x[3]);
            let mut sum = 0;

            for (diff, nums) in diffs.iter().zip(buyers.iter()) {
                let idx = diff
                    .iter()
                    .tuple_windows()
                    .position(|(&ax, &bx, &cx, &dx)| ax == a && bx == b && cx == c && dx == d);

                if let Some(idx) = idx {
                    sum += nums[idx + 4] % 10;
                }
            }

            sum
        })
        .max()
        .unwrap();

    out.into()
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
