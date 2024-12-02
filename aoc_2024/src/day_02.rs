use common::{solution, Answer};
use itertools::Itertools;

solution!("Red-Nosed Reports", 2);

fn part_a(input: &str) -> Answer {
    let reports = parse(input);
    reports.iter().filter(|x| is_safe(x)).count().into()
}

fn part_b(input: &str) -> Answer {
    let reports = parse(input);
    reports.iter().filter(|x| is_safe_b(x, None)).count().into()
}

fn is_safe(input: &[i32]) -> bool {
    let sig = (input[0] - input[1]).signum();
    input
        .iter()
        .tuple_windows()
        .map(|(a, b)| a - b)
        .all(|x| (1..=3).contains(&x.abs()) && x.signum() == sig)
}

fn is_safe_b(input: &[i32], skip: Option<usize>) -> bool {
    let vals = input
        .iter()
        .enumerate()
        .filter(|(idx, _)| skip.is_none() || Some(*idx) != skip)
        .map(|(_, &x)| x);
    let mut diffs = vals.tuple_windows().map(|(a, b)| a - b).peekable();

    let sig = diffs.peek().unwrap().signum();
    let first_invalid = diffs.position(|x| !(1..=3).contains(&x.abs()) || x.signum() != sig);

    match first_invalid {
        Some(x) if skip.is_none() => {
            is_safe_b(input, Some(x + 1))
                || is_safe_b(input, Some(x.saturating_sub(1)))
                || is_safe_b(input, Some(x))
        }
        None => true,
        _ => false,
    }
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
