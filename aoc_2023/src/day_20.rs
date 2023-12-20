use std::collections::{HashMap, VecDeque};

use common::{Answer, Solution};

const BASE: &str = "broadcaster";

pub struct Day20;

impl Solution for Day20 {
    fn name(&self) -> &'static str {
        "Pulse Propagation"
    }

    fn part_a(&self, input: &str) -> Answer {
        let connections = parse_input(input);

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

        let mut low_pulse = 0;
        let mut high_pulse = 0;

        for _ in 0..1000 {
            low_pulse += 1;
            let mut next = VecDeque::new();
            for i in connections[BASE].target.iter() {
                next.push_back((BASE, i, Pulse::Low));
            }

            while let Some((source, target, pulse)) = next.pop_front() {
                match pulse {
                    Pulse::Low => low_pulse += 1,
                    Pulse::High => high_pulse += 1,
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

        (low_pulse * high_pulse).into()
    }

    fn part_b(&self, input: &str) -> Answer {
        let connections = parse_input(input);

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

        let mut low_pulse = 0;
        let mut high_pulse = 0;

        const IMPORTANT_CONNECTIONS: &[&str] = &["kd", "zf", "vg", "gs"];
        let mut last_cycle = [0; 4];
        let mut cycle_length = [0; 4];

        for i in 0.. {
            low_pulse += 1;
            let mut next = VecDeque::new();
            for i in connections[BASE].target.iter() {
                next.push_back((BASE, i, Pulse::Low));
            }

            while let Some((source, target, pulse)) = next.pop_front() {
                if let Some(idx) = IMPORTANT_CONNECTIONS.iter().position(|s| s == &source) {
                    if target == &"rg" && pulse == Pulse::High {
                        let last = last_cycle[idx];
                        last_cycle[idx] = i;
                        cycle_length[idx] = i - last + 1;

                        if cycle_length.iter().all(|&i| i != 0) {
                            dbg!(cycle_length.iter().product::<u64>());
                            std::process::exit(0);
                        }
                    }
                }

                match pulse {
                    Pulse::Low => low_pulse += 1,
                    Pulse::High => high_pulse += 1,
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

        (low_pulse * high_pulse).into()
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
enum Pulse {
    Low,
    High,
}

#[derive(Debug)]
enum ConnectionType {
    Normal,
    /// Flip-flop modules (prefix %) are either on or off; they are initially off.
    /// If a flip-flop module receives a high pulse, it is ignored and nothing happens.
    /// However, if a flip-flop module receives a low pulse, it flips between on and off.
    /// If it was off, it turns on and sends a high pulse.
    /// If it was on, it turns off and sends a low pulse.
    FlipFlop,
    /// Conjunction modules (prefix &) remember the type of the most recent pulse received from each of their connected input modules;
    /// they initially default to remembering a low pulse for each input. When a pulse is received, the conjunction module first updates its memory for that input.
    /// Then, if it remembers high pulses for all inputs, it sends a low pulse; otherwise, it sends a high pulse.
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

    #[test]
    fn part_b() {
        assert_eq!(Day20.part_b(CASE), ().into());
    }
}
