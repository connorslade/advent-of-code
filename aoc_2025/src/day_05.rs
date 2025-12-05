use std::ops::RangeInclusive;

use common::{Answer, solution};

solution!("Cafeteria", 5);

fn part_a(input: &str) -> Answer {
    let (ranges, nums) = parse(input);
    nums.iter()
        .filter(|n| ranges.iter().any(|r| r.contains(n)))
        .count()
        .into()
}

fn part_b(input: &str) -> Answer {
    let (mut ranges, _) = parse(input);
    ranges.sort_by_key(|x| *x.start());

    let (mut out, mut max) = (0, 0);
    for range in ranges {
        let (start, end) = (*range.start().max(&max), range.end() + 1);
        out += end.saturating_sub(start);
        max = max.max(end);
    }

    out.into()
}

fn parse(input: &str) -> (Vec<RangeInclusive<u64>>, Vec<u64>) {
    let (ranges, nums) = input.split_once("\n\n").unwrap();

    let ranges = (ranges.lines())
        .map(|x| {
            let (first, last) = x.split_once('-').unwrap();
            first.parse().unwrap()..=last.parse().unwrap()
        })
        .collect();
    let nums = nums.lines().map(|x| x.parse().unwrap()).collect();

    (ranges, nums)
}

#[cfg(test)]
mod test {
    use indoc::indoc;

    const CASE: &str = indoc! {"
        3-5
        10-14
        16-20
        12-18

        1
        5
        8
        11
        17
        32
    "};

    #[test]
    fn part_a() {
        assert_eq!(super::part_a(CASE), 3.into());
    }

    #[test]
    fn part_b() {
        assert_eq!(super::part_b(CASE), 14.into());
    }
}
