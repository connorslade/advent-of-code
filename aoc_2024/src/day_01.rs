use common::{solution, Answer};

solution!("Historian Hysteria", 1);

fn part_a(input: &str) -> Answer {
    let mut list_a = Vec::new();
    let mut list_b = Vec::new();

    for x in input.lines() {
        let mut parts = x.split_whitespace();
        let a = parts.next().unwrap().parse::<i32>().unwrap();
        let b = parts.next().unwrap().parse::<i32>().unwrap();

        list_a.push(a);
        list_b.push(b);
    }

    list_a.sort();
    list_b.sort();

    let mut diff = 0;

    for (a, b) in list_a.iter().zip(list_b.iter()) {
        diff += (a - b).abs();
    }

    diff.into()
}

fn part_b(input: &str) -> Answer {
    let mut list_a = Vec::new();
    let mut list_b = Vec::new();

    for x in input.lines() {
        let mut parts = x.split_whitespace();
        let a = parts.next().unwrap().parse::<i32>().unwrap();
        let b = parts.next().unwrap().parse::<i32>().unwrap();

        list_a.push(a);
        list_b.push(b);
    }

    let mut out = 0;

    for x in list_a {
        let mut count = 0;
        for &y in list_b.iter() {
            count += (y == x) as u32;
        }

        out += x * count as i32;
    }

    out.into()
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
