use common::{Answer, ISolution};
use hashbrown::{HashMap, HashSet};
use petgraph::graph::{NodeIndex, UnGraph};

pub struct Day12;

impl ISolution for Day12 {
    fn name(&self) -> &'static str {
        "Passage Pathing"
    }

    fn part_a(&self, input: &str) -> Answer {
        let graph = parse(input);
        paths_a(&graph, graph.start, HashSet::new()).into()
    }

    fn part_b(&self, _input: &str) -> Answer {
        Answer::Unimplemented
    }
}

fn paths_a(graph: &ParseResult, at: NodeIndex, mut visited: HashSet<NodeIndex>) -> usize {
    if at == graph.end {
        return 1;
    }

    if !visited.insert(at) && graph.graph[at].cave_type != Type::Big {
        return 0;
    }

    graph
        .graph
        .neighbors(at)
        .map(|child| paths_a(graph, child, visited.clone()))
        .sum()
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
enum Type {
    Small,
    Big,
    Root,
}

struct Node<'a> {
    name: &'a str,
    cave_type: Type,
}

struct ParseResult<'a> {
    graph: UnGraph<Node<'a>, ()>,
    start: NodeIndex,
    end: NodeIndex,
}

fn parse(input: &str) -> ParseResult {
    let mut graph = UnGraph::new_undirected();
    let mut nodes = HashMap::new();

    fn make_node(name: &str) -> Node {
        Node {
            name,
            cave_type: if name == "start" || name == "end" {
                Type::Root
            } else if name.chars().next().unwrap().is_uppercase() {
                Type::Big
            } else {
                Type::Small
            },
        }
    }

    fn get_node<'a>(
        nodes: &mut HashMap<&'a str, NodeIndex>,
        graph: &mut UnGraph<Node<'a>, ()>,
        name: &'a str,
    ) -> NodeIndex {
        *nodes
            .entry(name)
            .or_insert_with(|| graph.add_node(make_node(name)))
    }

    for line in input.lines() {
        let (from, to) = line.split_once('-').unwrap();
        let from = get_node(&mut nodes, &mut graph, from);
        let to = get_node(&mut nodes, &mut graph, to);
        graph.add_edge(from, to, ());
    }

    fn find<'a>(graph: &'a UnGraph<Node<'a>, ()>, name: &str) -> NodeIndex {
        graph
            .node_indices()
            .find(|i| graph[*i].name == name)
            .unwrap()
    }

    ParseResult {
        start: find(&graph, "start"),
        end: find(&graph, "end"),
        graph,
    }
}

#[cfg(test)]
mod test {
    use common::ISolution;
    use indoc::indoc;

    use super::Day12;

    const CASE: &str = indoc! {"
        start-A
        start-b
        A-c
        A-b
        b-d
        A-end
        b-end
    "};

    const CASE_2: &str = indoc! {"
        dc-end
        HN-start
        start-kj
        dc-start
        dc-HN
        LN-dc
        HN-end
        kj-sa
        kj-HN
        kj-dc
    "};

    #[test]
    fn part_a() {
        assert_eq!(Day12.part_a(CASE), 10.into());
        assert_eq!(Day12.part_a(CASE_2), 19.into());
    }

    #[test]
    fn part_b() {
        assert_eq!(Day12.part_b(CASE), ().into()); // 36
    }
}
