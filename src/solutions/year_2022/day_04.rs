use crate::{problem, Solution};

pub struct Day04;

impl Solution for Day04 {
    fn name(&self) -> &'static str {
        "Camp Cleanup"
    }

    fn part_a(&self) -> String {
        let raw = problem::load(2022, 4);
        let mut out = 0;

        for i in assignment_loop(raw) {
            let (p1, p2) = (&i[0], &i[1]);
            out +=
                ((p1[0] >= p2[0] && p1[1] <= p2[1]) || (p2[0] >= p1[0] && p2[1] <= p1[1])) as usize;
        }

        out.to_string()
    }

    fn part_b(&self) -> String {
        let raw = problem::load(2022, 4);
        let mut out = 0;

        for i in assignment_loop(raw) {
            let (p1, p2) = (&i[0], &i[1]);
            out += ((p1[0] >= p2[0] && p1[1] <= p2[1])
                || (p2[0] >= p1[0] && p2[0] <= p1[1])
                || (p1[1] >= p2[0] && p1[1] <= p2[1])
                || (p2[1] >= p1[0] && p2[1] <= p1[1])) as usize;
        }

        out.to_string()
    }
}

fn assignment_loop(raw: String) -> Vec<Vec<Vec<u32>>> {
    raw.trim()
        .lines()
        .map(|x| x.split(',').map(split_range).collect::<Vec<_>>())
        .collect()
}

fn split_range(range: &str) -> Vec<u32> {
    range
        .split('-')
        .map(|x| x.parse::<u32>().unwrap())
        .collect::<Vec<_>>()
}
