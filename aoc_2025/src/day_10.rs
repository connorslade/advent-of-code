use std::{
    collections::{HashMap, HashSet, VecDeque},
    u64,
};

use common::{Answer, solution};
use itertools::Itertools;

solution!("Factory", 10);

#[derive(Debug)]
struct Machine {
    lights: Vec<bool>,
    buttons: Vec<Vec<usize>>,
    joltage: Vec<u64>,
}

impl Machine {
    fn parse(line: &str) -> Self {
        let (lights, line) = line.split_once(' ').unwrap();
        let lights = lights[1..lights.len() - 1]
            .chars()
            .map(|x| x == '#')
            .collect::<Vec<_>>();

        let (buttons, joltage) = line.rsplit_once(' ').unwrap();
        let buttons = buttons
            .split_whitespace()
            .map(|x| {
                x[1..x.len() - 1]
                    .split(',')
                    .map(|x| x.parse::<usize>().unwrap())
                    .collect::<Vec<_>>()
            })
            .collect::<Vec<_>>();

        let joltage = joltage[1..joltage.len() - 1]
            .split(',')
            .map(|x| x.parse::<u64>().unwrap())
            .collect::<Vec<_>>();

        Machine {
            lights,
            buttons,
            joltage,
        }
    }

    fn min_presses_a(&self) -> u64 {
        let mut queue = VecDeque::new();
        let mut seen = HashSet::new();
        queue.push_back((vec![false; self.lights.len()], 0));

        while let Some((lights, presses)) = queue.pop_front() {
            dbg!(queue.len());
            if lights == self.lights {
                return presses;
            }

            for buttons in &self.buttons {
                let mut next = lights.clone();
                for button in buttons {
                    next[*button] ^= true;
                }

                if !seen.contains(&next) {
                    seen.insert(next.clone());
                    queue.push_back((next, presses + 1));
                }
            }
        }

        0
    }

    fn inner(&self, seen: &mut HashMap<Vec<u64>, u64>, joltage: Vec<u64>, depth: u64) -> u64 {
        if let Some(seen) = seen.get(&joltage) {
            return *seen;
        }

        if *joltage == self.joltage {
            return 1;
        }

        if joltage
            .iter()
            .zip(self.joltage.iter())
            .any(|(a, b)| *a > *b)
        {
            return u64::MAX - 1;
        }

        let mut min = u64::MAX - 1;

        for buttons in &self.buttons {
            let mut next = joltage.clone();
            for button in buttons {
                next[*button] += 1;
            }

            min = min.min(self.inner(seen, next, depth + 1) + 1);
        }

        seen.insert(joltage.to_owned(), min);
        min
    }

    fn min_presses_b(&self) -> u64 {
        let mut queue = VecDeque::new();
        let mut seen = HashSet::new();
        queue.push_back(vec![0; self.buttons.len()]);

        while let Some(buttons) = queue.pop_front() {
            let mut joltage = vec![0; self.joltage.len()];
            let mut presses = 0;

            for (count, buttons) in buttons.iter().zip(self.buttons.iter()) {
                presses += count;
                for button in buttons.iter() {
                    joltage[*button] += count;
                }
            }

            if joltage == self.joltage {
                return presses;
            }

            for i in 0..self.buttons.len() {
                let mut next = buttons.clone();
                next[i] += 1;

                if !seen.contains(&next) {
                    seen.insert(next.clone());
                    queue.push_back(next);
                }
            }
        }

        0
        // self.inner(&mut HashMap::new(), vec![0; self.joltage.len()], 0) - 1
    }
}

fn parse(input: &str) -> Vec<Machine> {
    input.lines().map(|x| Machine::parse(x)).collect()
}

fn part_a(input: &str) -> Answer {
    let machines = parse(input);

    let mut sum = 0;
    for machine in machines {
        sum += dbg!(machine.min_presses_a());
    }

    sum.into()
}

// currently generating code to paste into mathematica :eyes:
fn part_b(input: &str) -> Answer {
    let machines = parse(input);

    let chars = b"abcdefghijklmnopqrstuvwxyz";
    for machine in machines {
        let mut constraints = Vec::new();
        for (i, j) in machine.joltage.iter().enumerate() {
            let pos = machine
                .buttons
                .iter()
                .enumerate()
                .filter(|(_i, x)| x.contains(&i))
                .map(|(i, _)| chars[i] as char)
                .join("+");
            constraints.push(format!("{j}=={pos}"));
        }

        for i in 0..machine.buttons.len() {
            constraints.push(format!(
                "{c}>=0,{c}\\[Element] Integers",
                c = chars[i] as char
            ));
        }

        let sum = (0..machine.buttons.len())
            .map(|i| (chars[i] as char).to_string())
            .join("+");
        let vars = (0..machine.buttons.len())
            .map(|i| (chars[i] as char).to_string())
            .join(",");

        println!(
            "Total[#[[2]] & /@ LinearOptimization[{sum}, {{{}}}, {{{vars}}}]] +",
            constraints.join(",")
        );
    }

    ().into()
}

#[cfg(test)]
mod test {
    use indoc::indoc;

    const CASE: &str = indoc! {"
        [.##.] (3) (1,3) (2) (2,3) (0,2) (0,1) {3,5,4,7}
        [...#.] (0,2,3,4) (2,3) (0,4) (0,1,2) (1,2,3,4) {7,5,12,7,2}
        [.###.#] (0,1,2,3,4) (0,3,4) (0,1,2,4,5) (1,2) {10,11,11,5,10,5}
    "};

    #[test]
    fn part_a() {
        assert_eq!(super::part_a(CASE), 7.into());
    }

    #[test]
    fn part_b() {
        assert_eq!(super::part_b(CASE), ().into());
    }
}
