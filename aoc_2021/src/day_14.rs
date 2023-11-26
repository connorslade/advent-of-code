use hashbrown::HashMap;

use common::Solution;

pub struct Day14;

impl Solution for Day14 {
    fn name(&self) -> &'static str {
        "Extended Polymerization"
    }

    fn part_a(&self, input: &str) -> String {
        process(input, 10).to_string()
    }

    // TODO: work with counts of units instead of the units themselves
    fn part_b(&self, input: &str) -> String {
        let mut _polymer = Polymer::parse(input);
        todo!()
    }
}

fn process(raw: &str, steps: usize) -> usize {
    let mut polymer = Polymer::parse(raw);
    (0..steps).for_each(|_| polymer.step());

    let (min, max) = polymer.min_max();
    max - min
}

#[derive(Debug)]
struct Polymer {
    units: Vec<char>,
    key: HashMap<[char; 2], char>,
}

impl Polymer {
    fn step(&mut self) {
        let mut next = Vec::new();
        for i in self.units.windows(2) {
            next.push(i[0]);

            if let Some(i) = self.key.get(i) {
                next.push(*i);
            }
        }

        next.push(*self.units.last().unwrap());
        self.units = next;
    }

    fn min_max(&self) -> (usize, usize) {
        let mut out = HashMap::new();
        self.units
            .iter()
            .for_each(|i| *out.entry(*i).or_insert(0) += 1);

        let mut out = out.into_iter().collect::<Vec<_>>();
        out.sort_by(|a, b| a.1.cmp(&b.1));
        (out[0].1, out[out.len() - 1].1)
    }

    fn parse(raw: &str) -> Self {
        let (start, key) = raw.split_once("\n\n").unwrap();
        let mut key_out = HashMap::new();

        for i in key.lines() {
            let (k, v) = i.split_once(" -> ").unwrap();
            let mut k = k.chars();
            key_out.insert(
                [k.next().unwrap(), k.next().unwrap()],
                v.chars().next().unwrap(),
            );
        }

        Self {
            units: start.chars().collect(),
            key: key_out,
        }
    }
}
