use std::{
    cell::RefCell,
    collections::{hash_map::Keys, HashMap, HashSet},
    fs,
    hash::Hash,
};

use common::{solution, Answer};
use itertools::Itertools;

solution!("Crossed Wires", 24);

fn part_a(input: &str) -> Answer {
    let problem = Problem::parse(input);

    let keys = problem.inner_keys();
    let keys = keys.filter(|x| x.starts_with('z')).sorted().rev();

    let mut out = 0;
    for key in keys {
        let val = problem.get(key);
        out <<= 1;
        out |= val as u64;
    }

    out.into()
}

fn part_b(input: &str) -> Answer {
    let problem = Problem::parse(input);
    let all_keys = problem.keys();
    let keys = problem.inner_keys();

    // input
    let mut bits;
    let x = {
        let all_keys = all_keys.clone();
        let x = all_keys
            .iter()
            .filter(|x| x.starts_with('x'))
            .sorted()
            .rev();
        let mut out = 0;
        bits = x.len();
        for key in x {
            let val = problem.get(key);
            out <<= 1;
            out |= val as u64;
        }
        out
    };
    let y = {
        let x = all_keys
            .iter()
            .filter(|x| x.starts_with('y'))
            .sorted()
            .rev();
        let mut out = 0;
        bits = bits.max(x.len());
        for key in x {
            let val = problem.get(key);
            out <<= 1;
            out |= val as u64;
        }
        out
    };

    let correct = x + y;

    let keys = keys.clone().filter(|x| x.starts_with('z')).sorted().rev();
    let mut to_swap = HashSet::new();

    let mut bit = dbg!(bits);
    let mut out = 0;
    for key in keys.clone() {
        let val = problem.get(key);
        let correct_bit = (correct >> bit) & 1 != 0;

        if val != correct_bit {
            // println!("wrong output: {key}");
            to_swap.insert(key);
        }

        out <<= 1;
        out |= val as u64;

        bit = bit.saturating_sub(1);
    }

    if out == correct {
        println!("CORRECT!");
    }

    let mut vis = String::new();
    vis.push_str("digraph G {\n");

    for (key, (a, op, b)) in problem.connections.iter() {
        vis.push_str(&format!("{a} -> {key}\n"));
        vis.push_str(&format!("{b} -> {key}\n"));
        vis.push_str(&format!("{key} [label=\"{key} {op:?}\"]\n"));

        if key.starts_with("z") {
            vis.push_str(&format!("{key} -> out{key}\n"));
            vis.push_str(&format!(
                "out{key} [label=\"{key}\",style=filled,fillcolor=\"{}\"]\n",
                if key.starts_with('z') && *op != Operation::Xor {
                    "green"
                } else {
                    if to_swap.contains(key) {
                        "red"
                    } else {
                        "blue"
                    }
                }
            ));
        }
    }

    vis.push_str("}");
    fs::write("out.txt", vis).unwrap();

    todo!()
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
enum Operation {
    And,
    Or,
    Xor,
}

#[derive(Clone)]
struct Problem<'a> {
    // output ->
    connections: HashMap<&'a str, (&'a str, Operation, &'a str)>,
    values: RefCell<HashMap<&'a str, bool>>,
}

impl<'a> Problem<'a> {
    fn parse(input: &'a str) -> Self {
        let mut connections = HashMap::new();
        let mut values = HashMap::new();

        let (raw_values, raw_connections) = input.split_once("\n\n").unwrap();

        for value in raw_values.lines() {
            let (key, val) = value.split_once(": ").unwrap();
            let num = val == "1";
            values.insert(key, num);
        }

        for connection in raw_connections.lines() {
            let parts = connection.split_ascii_whitespace().collect::<Vec<_>>();
            let (a, op, b, out) = (parts[0], parts[1], parts[2], parts[4]);
            let op = match op {
                "AND" => Operation::And,
                "OR" => Operation::Or,
                "XOR" => Operation::Xor,
                _ => panic!(),
            };

            if connections.insert(out, (a, op, b)).is_some() {
                dbg!(a, op, b, out);
                panic!("uhh");
            }
        }

        Self {
            connections,
            values: RefCell::new(values),
        }
    }

    fn keys(&self) -> HashSet<&str> {
        let mut out = HashSet::new();
        out.extend(self.connections.keys());
        out.extend(self.values.borrow().keys());
        out
    }

    fn inner_keys(&self) -> Keys<'_, &str, (&str, Operation, &str)> {
        self.connections.keys()
    }

    fn find_children(&self, key: &'a str) -> Vec<&'a str> {
        if !self.connections.contains_key(key) {
            return vec![];
        }

        let mut out = Vec::new();

        let (a, _, b) = self.connections[key];
        out.push(key);
        out.extend(self.find_children(a));
        out.extend(self.find_children(b));

        out
    }

    fn get(&self, key: &'a str) -> bool {
        if let Some(&val) = self.values.borrow().get(key) {
            return val;
        }

        let (a, op, b) = &self.connections[key];
        let a = self.get(a);
        let b = self.get(b);

        let out = match op {
            Operation::And => a && b,
            Operation::Or => a || b,
            Operation::Xor => a ^ b,
        };

        self.values.borrow_mut().insert(key, out);
        out
    }

    fn swap(&mut self, a: &'a str, b: &'a str) {
        // a -> smth
        // b -> smth

        let a_source = self.connections[a];
        let b_source = self.connections[b];

        self.connections.insert(b, a_source);
        self.connections.insert(a, b_source);
    }
}

#[cfg(test)]
mod test {
    use indoc::indoc;

    const CASE: &str = indoc! {"
        x00: 1
        x01: 0
        x02: 1
        x03: 1
        x04: 0
        y00: 1
        y01: 1
        y02: 1
        y03: 1
        y04: 1

        ntg XOR fgs -> mjb
        y02 OR x01 -> tnw
        kwq OR kpj -> z05
        x00 OR x03 -> fst
        tgd XOR rvg -> z01
        vdt OR tnw -> bfw
        bfw AND frj -> z10
        ffh OR nrd -> bqk
        y00 AND y03 -> djm
        y03 OR y00 -> psh
        bqk OR frj -> z08
        tnw OR fst -> frj
        gnj AND tgd -> z11
        bfw XOR mjb -> z00
        x03 OR x00 -> vdt
        gnj AND wpb -> z02
        x04 AND y00 -> kjc
        djm OR pbm -> qhw
        nrd AND vdt -> hwm
        kjc AND fst -> rvg
        y04 OR y02 -> fgs
        y01 AND x02 -> pbm
        ntg OR kjc -> kwq
        psh XOR fgs -> tgd
        qhw XOR tgd -> z09
        pbm OR djm -> kpj
        x03 XOR y03 -> ffh
        x00 XOR y04 -> ntg
        bfw OR bqk -> z06
        nrd XOR fgs -> wpb
        frj XOR qhw -> z04
        bqk OR frj -> z07
        y03 OR x01 -> nrd
        hwm AND bqk -> z03
        tgd XOR rvg -> z12
        tnw OR pbm -> gnj
    "};

    #[test]
    fn part_a() {
        assert_eq!(super::part_a(CASE), 2024.into());
    }
}
