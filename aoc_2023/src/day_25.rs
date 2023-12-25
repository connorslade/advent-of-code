use std::collections::HashMap;

use common::{Answer, Solution};
use rustworkx_core::{
    connectivity::stoer_wagner_min_cut,
    petgraph::{graph::UnGraph, Graph, Undirected},
};

pub struct Day25;

impl Solution for Day25 {
    fn name(&self) -> &'static str {
        "Snowverload"
    }

    fn part_a(&self, input: &str) -> Answer {
        let wires = parse(input);

        let total = wires.wire.node_count();
        let (len, side) = stoer_wagner_min_cut(&wires.wire, |_| Ok::<i32, ()>(1))
            .unwrap()
            .unwrap();

        assert_eq!(len, 3);
        ((total - side.len()) * side.len()).into()
    }

    fn part_b(&self, _input: &str) -> Answer {
        Answer::Unimplemented
    }
}

struct Wires<'a> {
    wire: Graph<&'a str, (), Undirected>,
}

fn parse(input: &str) -> Wires {
    let mut wire = UnGraph::new_undirected();

    let mut nodes = HashMap::new();

    for line in input.lines() {
        let mut parts = line.split(": ");
        let key = parts.next().unwrap();
        let values = parts.next().unwrap().split_whitespace();

        let node = *nodes.entry(key).or_insert_with(|| wire.add_node(key));
        for value in values {
            let value = *nodes.entry(value).or_insert_with(|| wire.add_node(value));
            wire.add_edge(node, value, ());
        }
    }
    Wires { wire }
}

#[cfg(test)]
mod test {
    use common::Solution;
    use indoc::indoc;

    use super::Day25;

    const CASE: &str = indoc! {"
        jqt: rhn xhk nvd
        rsh: frs pzl lsr
        xhk: hfx
        cmg: qnr nvd lhk bvb
        rhn: xhk bvb hfx
        bvb: xhk hfx
        pzl: lsr hfx nvd
        qnr: nvd
        ntq: jqt hfx bvb xhk
        nvd: lhk
        lsr: lhk
        rzs: qnr cmg lsr rsh
        frs: qnr lhk lsr
    "};

    #[test]
    fn part_a() {
        assert_eq!(Day25.part_a(CASE), 54.into());
    }
}
