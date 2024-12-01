use common::{solution, Answer};

solution!("Historian Hysteria", 1);

fn part_a(input: &str) -> Answer {
    let (mut list_a, mut list_b) = parse(input);
    list_a.sort();
    list_b.sort();

    list_a
        .into_iter()
        .zip(list_b)
        .map(|(a, b)| a.abs_diff(b))
        .sum::<u32>()
        .into()
}

fn part_b(input: &str) -> Answer {
    let (list_a, list_b) = parse(input);

    list_a
        .into_iter()
        .map(|x| {
            let count = list_b.iter().filter(|&&y| y == x).count();
            x * count as u32
        })
        .sum::<u32>()
        .into()
}

fn parse(input: &str) -> (Vec<u32>, Vec<u32>) {
    let (mut a, mut b) = (Vec::new(), Vec::new());

    for x in input.lines() {
        let mut parts = x.split_whitespace();
        a.push(parts.next().unwrap().parse::<u32>().unwrap());
        b.push(parts.next().unwrap().parse::<u32>().unwrap());
    }

    (a, b)
}

#[cfg(test)]
mod test {
    use indoc::indoc;

    const CASE: &str = indoc! {"
        3   4
        4   3
        2   5
        1   3
        3   9
        3   3
    "};

    #[test]
    fn part_a() {
        assert_eq!(super::part_a(CASE), 11.into());
    }

    #[test]
    fn part_b() {
        assert_eq!(super::part_b(CASE), 31.into());
    }
}
