use common::{Answer, Solution};

pub struct Day04;

impl Solution for Day04 {
    fn name(&self) -> &'static str {
        "Camp Cleanup"
    }

    fn part_a(&self, input: &str) -> Answer {
        let mut out = 0;

        for (p1, p2) in assignment_loop(input) {
            out += ((p1.0 >= p2.0 && p1.1 <= p2.1) || (p2.0 >= p1.0 && p2.1 <= p1.1)) as usize;
        }

        out.into()
    }

    fn part_b(&self, input: &str) -> Answer {
        let mut out = 0;

        for (p1, p2) in assignment_loop(input) {
            out += (p1.0.max(p2.0) <= p1.1.min(p2.1)) as usize;
        }

        out.into()
    }
}

fn assignment_loop(raw: &str) -> Vec<((u32, u32), (u32, u32))> {
    raw.trim()
        .lines()
        .map(|x| x.split_once(',').unwrap())
        .map(|(a, b)| (split_range(a), split_range(b)))
        .collect()
}

fn split_range(range: &str) -> (u32, u32) {
    let mut range = range.split('-').map(|x| x.parse::<u32>().unwrap());
    (range.next().unwrap(), range.next().unwrap())
}
