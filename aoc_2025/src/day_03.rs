use common::{Answer, solution};

solution!("Lobby", 3);

fn part_a(input: &str) -> Answer {
    solve(input, 2).into()
}

fn part_b(input: &str) -> Answer {
    solve(input, 12).into()
}

fn solve(input: &str, n: usize) -> u64 {
    let mut out = 0;
    for bank in input.trim().lines() {
        let digits = bank
            .chars()
            .map(|x| x.to_digit(10).unwrap() as u8)
            .collect::<Vec<_>>();
        out += joltage(&digits, n);
    }

    out
}

fn joltage(digits: &[u8], n: usize) -> u64 {
    let mut num = 0;
    let mut last_used = 0;

    for bat in 0..n {
        let size = digits.len() - (n - 1 - bat) - last_used;
        let slice = &digits[last_used..(last_used + size).min(digits.len())];

        for i in (0..=9).rev() {
            if let Some(idx) = slice.iter().position(|x| *x == i) {
                last_used += idx + 1;
                num = num * 10 + i as u64;
                break;
            }
        }
    }

    num
}

#[cfg(test)]
mod test {
    use indoc::indoc;

    const CASE: &str = indoc! {"
        987654321111111
        811111111111119
        234234234234278
        818181911112111
    "};

    #[test]
    fn part_a() {
        assert_eq!(super::part_a(CASE), 357.into());
    }

    #[test]
    fn part_b() {
        assert_eq!(super::part_b(CASE), 3121910778619_u64.into());
    }
}
