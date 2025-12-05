use std::ops::RangeInclusive;

use common::{Answer, solution};

solution!("Cafeteria", 5);

fn part_a(input: &str) -> Answer {
    let (ranges_raw, nums_raw) = input.split_once("\n\n").unwrap();
    let mut ranges = Vec::new();
    for range in ranges_raw.lines() {
        let (first, last) = range.split_once('-').unwrap();
        ranges.push(first.parse::<u64>().unwrap()..=last.parse::<u64>().unwrap());
    }

    let mut out = 0;
    for num in nums_raw.lines() {
        let num = num.parse::<u64>().unwrap();
        if ranges.iter().any(|x| x.contains(&num)) {
            out += 1;
            continue;
        }
    }

    out.into()
}

#[derive(Default, Debug)]
struct Ranges {
    inner: Vec<RangeInclusive<u64>>,
}

fn range_intersects(a: &RangeInclusive<u64>, b: &RangeInclusive<u64>) -> bool {
    a.start() <= b.start() && a.end() >= b.start() || b.start() <= a.start() && b.end() >= a.start()
}

impl Ranges {
    fn add(&mut self, new: RangeInclusive<u64>) {
        for i in 0..self.inner.len() {
            let range = &self.inner[i];
            if range_intersects(&range, &new) {
                let start = *range.start().min(new.start());
                let end = *range.end().max(new.end());
                let new = start..=end;

                self.inner.remove(i);
                self.add(new);
                return;
            }
        }

        self.inner.push(new);
    }

    fn count(&self) -> u64 {
        let mut out = 0;
        for range in &self.inner {
            out += range.end() - range.start() + 1;
        }

        out
    }
}

fn part_b(input: &str) -> Answer {
    let (ranges_raw, _s) = input.split_once("\n\n").unwrap();
    let mut ranges = Ranges::default();

    for range in ranges_raw.lines() {
        let (first, last) = range.split_once('-').unwrap();
        let range = first.parse::<u64>().unwrap()..=last.parse::<u64>().unwrap();
        ranges.add(range);
    }

    dbg!(&ranges);

    ranges.count().into()
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

    // const CASE: &str = indoc! {"
    //     3-5
    //     2-7
    //     8-10
    //     2-9

    // "};

    #[test]
    fn part_a() {
        assert_eq!(super::part_a(CASE), 3.into());
    }

    #[test]
    fn part_b() {
        assert_eq!(super::part_b(CASE), 14.into());
    }
}

// not: 381374757320727
// not: 346809913640090
