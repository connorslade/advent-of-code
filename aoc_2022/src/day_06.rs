use hashbrown::HashSet;

use common::Solution;

pub struct Day06;

impl Solution for Day06 {
    fn name(&self) -> &'static str {
        "Tuning Trouble"
    }

    fn part_a(&self, input: &str) -> String {
        process(input, 4).to_string()
    }

    fn part_b(&self, input: &str) -> String {
        process(input, 14).to_string()
    }
}

fn process(input: &str, size: usize) -> usize {
    let mut chars = HashSet::new();
    'o: for i in input.chars().enumerate().collect::<Vec<_>>().windows(size) {
        for j in i {
            if !chars.insert(j.1) {
                chars.clear();
                continue 'o;
            }
        }

        return i[size - 1].0 + 1;
    }

    unreachable!()
}
