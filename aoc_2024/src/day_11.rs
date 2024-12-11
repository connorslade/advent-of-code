use std::collections::HashMap;

use common::{solution, Answer};

solution!("Plutonian Pebbles", 11);

fn part_a(input: &str) -> Answer {
    let mut nums = parse(input);

    // 0->1
    // even -> (left digits)(right digits)
    // else -> *= 2024
    for _ in 0..25 {
        let mut i = 0;
        while i < nums.len() {
            let num = nums[i];
            if num == 0 {
                nums[i] = 1;
            } else if even_digits(num) {
                let (a, b) = split_digits(num);
                nums[i] = a;
                i += 1;
                nums.insert(i, b);
            } else {
                nums[i] *= 2024;
            }
            i += 1;
        }
    }

    nums.len().into()
}

fn part_b(input: &str) -> Answer {
    let nums = parse(input);

    // 0->1
    // even -> (left digits)(right digits)
    // else -> *= 2024
    let mut counts = HashMap::<u64, u64>::new();
    for num in nums {
        *counts.entry(num).or_default() += 1;
    }
    println!("{counts:?}");

    for _ in 0..75 {
        let mut next = HashMap::new();
        for (k, v) in counts {
            if k == 0 {
                *next.entry(1).or_default() += v;
            } else if even_digits(k) {
                let (a, b) = split_digits(k);
                *next.entry(a).or_default() += v;
                *next.entry(b).or_default() += v;
            } else {
                *next.entry(k * 2024).or_default() += v;
            }
        }
        println!("{next:?}");
        counts = next;
    }

    counts.values().sum::<u64>().into()
}

fn even_digits(mut num: u64) -> bool {
    let mut digits = 0;

    while num > 0 {
        num /= 10;
        digits += 1;
    }

    digits % 2 == 0
}

fn split_digits(num: u64) -> (u64, u64) {
    let mut working = num;
    let mut digits = 0;

    while working > 0 {
        working /= 10;
        digits += 1;
    }

    let pow = 10_u64.pow(digits / 2);
    (num / pow, num - (num / pow) * pow)
}

fn parse(input: &str) -> Vec<u64> {
    input
        .split_ascii_whitespace()
        .map(|x| x.parse().unwrap())
        .collect()
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
        assert_eq!(super::part_b(CASE), 55312.into());
    }
}
