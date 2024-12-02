use std::usize;

use common::{solution, Answer};
use itertools::Itertools;

solution!("Red-Nosed Reports", 2);

fn part_a(input: &str) -> Answer {
    let reports = parse(input);

    let mut safe = 0;
    for report in reports {
        if is_safe(report, usize::MAX) {
            safe += 1;
        }
    }

    safe.into()
}

fn part_b(input: &str) -> Answer {
    let reports = parse(input);

    let mut safe = 0;
    for report in reports {
        if dbg!(is_safe_b(&report, usize::MAX)) {
            safe += 1;
        }
    }

    safe.into()
}

fn is_safe(input: Vec<i32>, skip: usize) -> bool {
    let diffs = input
        .iter()
        .enumerate()
        .filter(|(idx, _)| *idx != skip)
        .map(|(_, &x)| x)
        .tuple_windows()
        .map(|(a, b)| a - b)
        .collect::<Vec<_>>();

    let count = diffs
        .iter()
        .filter(|&x| x.abs() > 3 || x.abs() < 1 || x.signum() != diffs[0].signum())
        .count();

    count == 0
}

fn is_safe_b(input: &[i32], skip: usize) -> bool {
    let diffs = input
        .iter()
        .enumerate()
        .filter(|(idx, _)| *idx != skip)
        .map(|(_, &x)| x)
        .tuple_windows()
        .map(|(a, b)| a - b)
        .collect::<Vec<_>>();

    let invalid = diffs
        .iter()
        .map(|&x| x.abs() > 3 || x.abs() < 1 || x.signum() != diffs[0].signum())
        .collect::<Vec<_>>();

    if invalid.iter().all(|x| *x == false) {
        return true;
    }

    if skip == usize::MAX {
        if let Some(first_invalid) = invalid.iter().position(|x| *x == true) {
            return is_safe_b(input, first_invalid)
                | is_safe_b(input, first_invalid + 1)
                | is_safe_b(input, first_invalid.saturating_sub(1));
        }
    }

    false
}

fn parse(input: &str) -> Vec<Vec<i32>> {
    input
        .lines()
        .map(|x| {
            x.split_whitespace()
                .map(|x| x.parse::<i32>().unwrap())
                .collect::<Vec<_>>()
        })
        .collect::<Vec<_>>()
}

#[cfg(test)]
mod test {
    use indoc::indoc;

    const CASE: &str = indoc! {"
        7 6 4 2 1
        1 2 7 8 9
        9 7 6 2 1
        1 3 2 4 5
        8 6 4 4 1
        1 3 6 7 9
    "};

    #[test]
    fn part_a() {
        assert_eq!(super::part_a(CASE), 2.into());
    }

    #[test]
    fn part_b() {
        assert_eq!(super::part_b(CASE), 4.into());
    }
}
