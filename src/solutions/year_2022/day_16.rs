use crate::{problem, Solution};

use hashbrown::HashMap;
use petgraph::Graph;

pub struct Day16;

impl Solution for Day16 {
    fn name(&self) -> &'static str {
        "Proboscidea Volcanium"
    }

    fn part_a(&self) -> String {
        let raw = problem::load(2022, 16);
        let graph = parse(&raw);

        todo!()
    }

    fn part_b(&self) -> String {
        let raw = problem::load(2022, 16);
        todo!()
    }
}

fn parse(raw: &str) -> Graph<bool, u8> {
    let mut graph = Graph::new();
    // maps (valve) -> (weight, to)
    let mut working: HashMap<_, (u8, Vec<&str>)> = HashMap::new();
    let mut nodes = HashMap::new();
    for i in raw.lines() {
        let parts = i.split_whitespace().collect::<Vec<_>>();
        let name = parts[1];
        let flow = parts[4]
            .trim_start_matches("rate=")
            .trim_end_matches(";")
            .parse::<u8>()
            .unwrap();
        let to = parts[9..]
            .iter()
            .map(|x| x.trim_end_matches(','))
            .collect::<Vec<_>>();

        let node = graph.add_node(false);
        nodes.insert(name.to_string(), node);
        working.insert(node, (flow, to));
    }

    for (node, (flow, to)) in working {
        for i in to {
            let to = nodes[i];
            graph.add_edge(node, to, flow);
        }
    }

    graph
}
