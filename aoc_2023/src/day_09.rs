use common::{solution, Answer};

solution!("Mirage Maintenance", 9);

fn part_a(input: &str) -> Answer {
    parse(input)
        .iter()
        .map(Sequence::predict)
        .sum::<i64>()
        .into()
}

fn part_b(input: &str) -> Answer {
    parse(input)
        .into_iter()
        .map(|x| x.reverse().predict())
        .sum::<i64>()
        .into()
}

struct Sequence {
    values: Vec<i64>,
}

fn parse(input: &str) -> Vec<Sequence> {
    let mut out = Vec::new();

    for line in input.lines() {
        let values = line
            .split_whitespace()
            .map(|v| v.parse().unwrap())
            .collect();
        out.push(Sequence { values });
    }

    out
}

impl Sequence {
    fn derive(&self) -> Vec<Vec<i64>> {
        let mut derived = vec![self.values.clone()];

        while !derived.last().unwrap().iter().all(|&x| x == 0) {
            let last = derived.last().unwrap();
            let mut next = Vec::new();

            for i in 1..last.len() {
                next.push(last[i] - last[i - 1]);
            }

            derived.push(next);
        }

        derived
    }

    fn reverse(mut self) -> Self {
        self.values.reverse();
        self
    }

    fn predict(&self) -> i64 {
        self.derive().iter().filter_map(|v| v.last()).sum()
    }
}

#[cfg(test)]
mod test {
    use indoc::indoc;

    const CASE: &str = indoc! {"
        0 3 6 9 12 15
        1 3 6 10 15 21
        10 13 16 21 30 45
    "};

    #[test]
    fn part_a() {
        assert_eq!(super::part_a(CASE), 114.into());
    }

    #[test]
    fn part_b() {
        assert_eq!(super::part_b(CASE), 2.into());
    }
}
