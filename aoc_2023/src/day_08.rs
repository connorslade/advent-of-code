use std::collections::HashMap;

use aoc_lib::math::lcm;
use common::{solution, Answer};

solution!("Haunted Wasteland", 8);

/// Just start at `AAA` and follow the instructions until you reach `ZZZ`.
fn part_a(input: &str) -> Answer {
    let map = parse(input);

    let mut i = 0;
    let mut pos = "AAA";
    loop {
        pos = map.get(pos, i);
        i += 1;

        if pos == "ZZZ" {
            break;
        }
    }

    i.into()
}

/// Get the cycle length for each starting position, this is the number of positions you need to get from `AAA` to `ZZZ`.
/// Calculate the least common multiple of all cycle lengths to get the number of steps needed to get from `AAA` to `ZZZ` for all starting positions.
fn part_b(input: &str) -> Answer {
    let map = parse(input);

    let mut pos = Vec::new();
    for &id in map.nodes.keys() {
        if id.ends_with('A') {
            pos.push(id);
        }
    }

    let mut cycles = Vec::new();
    for mut pos in pos {
        let mut cycle_len = 0_u64;
        let mut i = 0;
        loop {
            pos = map.get(pos, i);
            i += 1;

            cycle_len += 1;
            if pos.ends_with('Z') {
                cycles.push(cycle_len);
                break;
            }
        }
    }

    cycles.into_iter().reduce(lcm).unwrap().into()
}

#[derive(Debug)]
struct Map<'a> {
    // Char array of 'L's and 'R's
    instructions: &'a [u8],
    // Node => (Left, Right)
    nodes: HashMap<&'a str, (&'a str, &'a str)>,
}

impl<'a> Map<'a> {
    fn get(&self, pos: &'a str, i: usize) -> &'a str {
        let (left, right) = self.nodes.get(pos).unwrap();
        match self.instructions[i % self.instructions.len()] as char {
            'L' => left,
            'R' => right,
            _ => unreachable!(),
        }
    }
}

fn parse(input: &str) -> Map {
    let (instructions, node_list) = input.split_once("\n\n").unwrap();

    let mut nodes = HashMap::new();
    for node in node_list.lines() {
        let (id, children) = node.split_once(" = ").unwrap();
        let children = children
            .trim_start_matches('(')
            .trim_end_matches(')')
            .split_once(", ")
            .unwrap();
        nodes.insert(id, children);
    }

    Map {
        instructions: instructions.as_bytes(),
        nodes,
    }
}

#[cfg(test)]
mod test {
    use indoc::indoc;

    const CASE_A: &str = indoc! {"
        LLR

        AAA = (BBB, BBB)
        BBB = (AAA, ZZZ)
        ZZZ = (ZZZ, ZZZ)
    "};

    const CASE_B: &str = indoc! {"
        LR

        11A = (11B, XXX)
        11B = (XXX, 11Z)
        11Z = (11B, XXX)
        22A = (22B, XXX)
        22B = (22C, 22C)
        22C = (22Z, 22Z)
        22Z = (22B, 22B)
        XXX = (XXX, XXX)
    "};

    #[test]
    fn part_a() {
        assert_eq!(super::part_a(CASE_A), 6.into());
    }

    #[test]
    fn part_b() {
        assert_eq!(super::part_b(CASE_B), 6.into());
    }
}
