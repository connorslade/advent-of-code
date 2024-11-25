use std::collections::VecDeque;

use common::{solution, Answer};

use hashbrown::HashMap;

solution!("Proboscidea Volcanium", 16);

fn part_a(input: &str) -> Answer {
    let parse = parse(input);
    solve(&mut HashMap::new(), &parse, 0, parse.start, 30).into()
}

fn part_b(input: &str) -> Answer {
    let parse = parse(input);

    let mut max = 0;
    let set = (1 << parse.indices.len() as u64) - 1;

    let mut cache = HashMap::new();
    for i in 0..((set + 1) / 2) {
        let a = solve(&mut cache, &parse, i, parse.start, 26);
        let b = solve(&mut cache, &parse, set ^ i, parse.start, 26);
        max = max.max(a + b);
    }

    max.into()
}

fn solve(
    cache: &mut HashMap<(usize, u32, u64), u32>,
    parse: &ParseResult,
    open: u64,
    pos: usize,
    time: u32,
) -> u32 {
    if let Some(&result) = cache.get(&(pos, time, open)) {
        return result;
    }

    let mut result = 0;
    let neighbors = parse.distances.get(&pos).unwrap();

    for (neighbor, distance) in neighbors {
        let index = parse.indices.get(neighbor).unwrap();
        if open & (1 << index) != 0 {
            continue;
        }

        let new_time = time.saturating_sub(distance + 1);
        if new_time == 0 {
            continue;
        }

        let flow = parse.graph.get(neighbor).unwrap().0;
        result = result.max(
            solve(cache, parse, open | 1 << index, *neighbor, new_time) + flow as u32 * new_time,
        );
    }

    cache.insert((pos, time, open), result);
    result
}

struct ParseResult {
    graph: HashMap<usize, (u16, Box<[usize]>)>,
    distances: HashMap<usize, Vec<(usize, u32)>>,
    indices: HashMap<usize, usize>,
    start: usize,
}

fn parse(input: &str) -> ParseResult {
    let mut graph = HashMap::new();
    let mut name_map = HashMap::new();

    for line in input.lines() {
        let parts = line.split_whitespace().collect::<Vec<_>>();
        let name = parts[1];
        let flow = parts[4]
            .trim_start_matches("rate=")
            .trim_end_matches(';')
            .parse::<u16>()
            .unwrap();
        let to = parts[9..]
            .iter()
            .map(|x| x.trim_end_matches(','))
            .collect::<Vec<_>>();
        let index = graph.len();
        name_map.insert(name, index);
        graph.insert(index, (flow, to));
    }

    let mut out: HashMap<usize, (u16, Box<[usize]>)> = HashMap::new();

    for (index, (flow, to)) in graph {
        let to = to
            .iter()
            .map(|x| *name_map.get(x).unwrap())
            .collect::<Vec<_>>();
        out.insert(index, (flow, to.into()));
    }

    let start = *name_map.get("AA").unwrap();
    let mut distances = HashMap::new();

    for (valve, (flow, _)) in out.iter() {
        if *flow == 0 && *valve != start {
            continue;
        }

        let mut queue = VecDeque::new();
        let mut visited = vec![false; out.len()];
        queue.push_back((0, *valve));
        visited[*valve] = true;

        while let Some((distance, node)) = queue.pop_front() {
            for neighbor in out.get(&node).unwrap().1.iter() {
                if visited[*neighbor] {
                    continue;
                }
                visited[*neighbor] = true;

                if out.get(neighbor).unwrap().0 != 0 {
                    distances
                        .entry(*valve)
                        .or_insert(Vec::new())
                        .push((*neighbor, distance as u32 + 1));
                }

                queue.push_back((distance + 1, *neighbor));
            }
        }
    }

    let mut indices = HashMap::new();

    for (index, (flow, _)) in out.iter() {
        if *flow == 0 {
            continue;
        }

        indices.insert(*index, indices.len());
    }

    ParseResult {
        graph: out,
        distances,
        indices,
        start,
    }
}

#[cfg(test)]
mod test {
    use indoc::indoc;

    const CASE: &str = indoc! {"
        Valve AA has flow rate=0; tunnels lead to valves DD, II, BB
        Valve BB has flow rate=13; tunnels lead to valves CC, AA
        Valve CC has flow rate=2; tunnels lead to valves DD, BB
        Valve DD has flow rate=20; tunnels lead to valves CC, AA, EE
        Valve EE has flow rate=3; tunnels lead to valves FF, DD
        Valve FF has flow rate=0; tunnels lead to valves EE, GG
        Valve GG has flow rate=0; tunnels lead to valves FF, HH
        Valve HH has flow rate=22; tunnel leads to valve GG
        Valve II has flow rate=0; tunnels lead to valves AA, JJ
        Valve JJ has flow rate=21; tunnel leads to valve II
    "};

    #[test]
    fn part_a() {
        assert_eq!(super::part_a(CASE), 1651.into());
    }

    #[test]
    fn part_b() {
        assert_eq!(super::part_b(CASE), 1707.into());
    }
}
