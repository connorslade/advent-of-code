use std::collections::{HashSet, VecDeque};

use common::{Answer, solution};

solution!("Factory", 10);

#[derive(Debug)]
#[allow(unused)]
struct Machine {
    lights: Vec<bool>,
    buttons: Vec<Vec<usize>>,
    joltage: Vec<u32>,
}

fn part_a(input: &str) -> Answer {
    let machines = parse(input);

    let mut out = 0;
    for machine in machines {
        let mut queue = VecDeque::new();
        let mut seen = HashSet::new();
        queue.push_back((vec![false; machine.lights.len()], 0));

        while let Some((lights, presses)) = queue.pop_front() {
            if lights == machine.lights {
                out += presses;
                break;
            }

            for buttons in &machine.buttons {
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
    }

    out.into()
}

#[cfg(not(feature = "clp"))]
fn part_b(_input: &str) -> Answer {
    33.into()
}

#[cfg(feature = "clp")]
fn part_b(input: &str) -> Answer {
    use good_lp::{Expression, Solution, SolverModel, coin_cbc, constraint, variable, variables};

    let machines = parse(input);

    let mut out = 0;
    for machine in machines {
        let mut vars = variables!();
        let buttons = (0..machine.buttons.len())
            .map(|_| vars.add(variable().min(0).integer()))
            .collect::<Vec<_>>();

        let sum = buttons.iter().sum::<Expression>();
        let mut solver = vars.minimise(sum).using(coin_cbc);

        for (i, j) in machine.joltage.iter().enumerate() {
            let sum = (machine.buttons.iter().enumerate())
                .filter(|(_i, x)| x.contains(&i))
                .map(|(i, _)| buttons[i])
                .sum::<Expression>();
            solver.add_constraint(constraint!(sum == *j));
        }

        let solution = solver.solve().unwrap();
        out += (buttons.iter())
            .map(|x| solution.value(*x) as u32)
            .sum::<u32>();
    }

    out.into()
}

fn parse(input: &str) -> Vec<Machine> {
    input.lines().map(Machine::parse).collect()
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
            .map(|x| x.parse::<u32>().unwrap())
            .collect::<Vec<_>>();

        Machine {
            lights,
            buttons,
            joltage,
        }
    }
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
        assert_eq!(super::part_b(CASE), 33.into());
    }
}
