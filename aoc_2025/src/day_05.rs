use std::{mem, ops::RangeInclusive};

use common::{Answer, solution};

solution!("Cafeteria", 5);

type Range = RangeInclusive<u64>;

fn part_a(input: &str) -> Answer {
    let (ranges, nums) = parse(input);
    let count = nums.iter().filter(|x| ranges.contains(x)).count();
    count.into()
}

fn part_b(input: &str) -> Answer {
    let (ranges, _) = parse(input);
    ranges.count().into()
}

fn parse(input: &str) -> (Ranges, Vec<u64>) {
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

#[derive(Default, Debug)]
struct Ranges {
    inner: Vec<Range>,
}

impl Ranges {
    fn add(&mut self, mut new: RangeInclusive<u64>) {
        fn range_combine(a: &Range, b: &Range) -> Option<Range> {
            (a.start() <= b.start() && a.end() >= b.start()
                || b.start() <= a.start() && b.end() >= a.start())
            .then(|| (*a.start().min(b.start()))..=(*a.end().max(b.end())))
        }

        let mut i = 0;
        while i < self.inner.len() {
            if let Some(combination) = range_combine(&self.inner[i], &new) {
                self.inner.remove(mem::take(&mut i));
                new = combination;
                continue;
            }

            i += 1;
        }

        self.inner.push(new);
    }

    fn contains(&self, val: &u64) -> bool {
        self.inner.iter().any(|x| x.contains(val))
    }

    fn count(&self) -> u64 {
        self.inner.iter().map(|x| x.end() - x.start() + 1).sum()
    }
}

impl FromIterator<Range> for Ranges {
    fn from_iter<T: IntoIterator<Item = Range>>(iter: T) -> Self {
        let mut out = Ranges::default();
        iter.into_iter().for_each(|item| out.add(item));
        out
    }
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
