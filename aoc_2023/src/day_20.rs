use std::collections::{HashMap, VecDeque};

use common::{Answer, Solution};

pub struct Day20;

impl Solution for Day20 {
    fn name(&self) -> &'static str {
        "Pulse Propagation"
    }

    fn part_a(&self, input: &str) -> Answer {
        let connections = parse_input(input);

        let mut low_pulse = 999;
        let mut high_pulse = 0;

        simulate(&connections, |i, _source, _target, pulse| {
            match pulse {
                Pulse::Low => low_pulse += 1,
                Pulse::High => high_pulse += 1,
            }

            i < 1000
        });

        (low_pulse * high_pulse).into()
    }

    fn part_b(&self, input: &str) -> Answer {
        let connections = parse_input(input);
        let important_connections = important_connections(&connections);

        let mut last_cycle = [0; 4];
        let mut cycle_length = [0; 4];

        simulate(&connections, |i, source, target, pulse| {
            if target == "rg" && pulse == Pulse::High {
                if let Some(idx) = important_connections.iter().position(|s| s == &source) {
                    let last = last_cycle[idx];
                    last_cycle[idx] = i;
                    cycle_length[idx] = (i - last + 1) as u64;

                    if cycle_length.iter().all(|&i| i != 0) {
                        return false;
                    }
                }
            }

            true
        });

        cycle_length.iter().product::<u64>().into()
    }
}

fn simulate(
    connections: &HashMap<&str, Connection<'_>>,
    mut hook: impl FnMut(u32, &str, &str, Pulse) -> bool,
) {
    let mut flop_memory = HashMap::new();
    let mut conjunction_memory = HashMap::new();

    for i in connections.values() {
        match i.connection_type {
            ConnectionType::FlipFlop => {
                flop_memory.insert(i.source, false);
            }
            ConnectionType::Conjunction => {
                let mut map = HashMap::new();
                // Add map of all items mapping to i
                for j in connections.values() {
                    if j.target.contains(&i.source) {
                        map.insert(j.source, Pulse::Low);
                    }
                }
                conjunction_memory.insert(i.source, map);
            }
            _ => continue,
        }
    }

    for i in 0.. {
        let mut next = VecDeque::new();
        const BASE: &str = "broadcaster";
        for i in connections[BASE].target.iter() {
            next.push_back((BASE, i, Pulse::Low));
        }

        while let Some((source, target, pulse)) = next.pop_front() {
            if !hook(i, source, target, pulse) {
                return;
            }

            let Some(connection) = connections.get(target) else {
                continue;
            };

            let pulse = match connection.connection_type {
                ConnectionType::Normal => continue,
                ConnectionType::FlipFlop => {
                    if pulse == Pulse::High {
                        continue;
                    }

                    let mem = flop_memory.get_mut(target).unwrap();
                    *mem ^= true;

                    if *mem {
                        Pulse::High
                    } else {
                        Pulse::Low
                    }
                }
                ConnectionType::Conjunction => {
                    let mem = conjunction_memory.get_mut(target).unwrap();
                    let this = mem.get_mut(source).unwrap();
                    *this = pulse;

                    if mem.values().all(|&l| l == Pulse::High) {
                        Pulse::Low
                    } else {
                        Pulse::High
                    }
                }
            };

            for i in connection.target.iter() {
                next.push_back((target, i, pulse));
            }
        }
    }
}

fn important_connections<'a>(connections: &'a HashMap<&'a str, Connection<'a>>) -> Vec<&'a str> {
    // Find node that connects to rx
    let drain = connections
        .values()
        .find(|i| i.target.contains(&"rx"))
        .unwrap();
    assert_eq!(drain.connection_type, ConnectionType::Conjunction);

    // Find all nodes that connect to drain
    connections
        .values()
        .filter(|i| i.target.contains(&drain.source))
        .map(|i| i.source)
        .collect()
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
enum Pulse {
    Low,
    High,
}

#[derive(Debug, PartialEq, Eq)]
enum ConnectionType {
    Normal,
    FlipFlop,
    Conjunction,
}

#[derive(Debug)]
struct Connection<'a> {
    connection_type: ConnectionType,
    source: &'a str,
    target: Vec<&'a str>,
}

fn parse_input(input: &str) -> HashMap<&str, Connection<'_>> {
    let mut out = HashMap::new();

    for line in input.lines() {
        let (source, target) = line.split_once(" -> ").unwrap();
        let connection_type = match source.chars().next() {
            Some('%') => ConnectionType::FlipFlop,
            Some('&') => ConnectionType::Conjunction,
            _ => ConnectionType::Normal,
        };

        let source = source.trim_start_matches(|c| c == '%' || c == '&');
        out.insert(
            source,
            Connection {
                connection_type,
                source,
                target: target.split(", ").collect(),
            },
        );
    }

    out
}

#[cfg(test)]
mod test {
    use common::Solution;
    use indoc::indoc;

    use super::Day20;

    const CASE: &str = indoc! {"
        broadcaster -> a, b, c
        %a -> b
        %b -> c
        %c -> inv
        &inv -> a
    "};

    const CASE_2: &str = indoc! {"
        broadcaster -> a
        %a -> inv, con
        &inv -> b
        %b -> con
        &con -> output
    "};

    #[test]
    fn part_a() {
        assert_eq!(Day20.part_a(CASE), 32000000.into());
    }

    #[test]
    fn part_a_2() {
        assert_eq!(Day20.part_a(CASE_2), 11687500.into());
    }
}
