use common::{solution, Answer};
use hashbrown::{HashMap, HashSet};
use petgraph::graph::{NodeIndex, UnGraph};

solution!("Passage Pathing", 12);

fn part_a(input: &str) -> Answer {
    let graph = parse(input);

    fn paths(graph: &ParseResult, at: NodeIndex, mut visited: HashSet<NodeIndex>) -> usize {
        if at == graph.end {
            return 1;
        }

        if !visited.insert(at) && graph.graph[at].cave_type != Type::Big {
            return 0;
        }

        graph
            .graph
            .neighbors(at)
            .map(|child| paths(graph, child, visited.clone()))
            .sum()
    }

    paths(&graph, graph.start, HashSet::new()).into()
}

fn part_b(input: &str) -> Answer {
    let graph = parse(input);

    fn paths(
        graph: &ParseResult,
        at: NodeIndex,
        mut visited: HashSet<NodeIndex>,
        mut small_twice: bool,
    ) -> usize {
        if at == graph.end {
            return 1;
        }

        let cave = graph.graph[at].cave_type;
        if !visited.insert(at) && cave != Type::Big {
            if !small_twice && cave == Type::Small {
                small_twice = true;
            } else {
                return 0;
            }
        }

        graph
            .graph
            .neighbors(at)
            .map(|child| paths(graph, child, visited.clone(), small_twice))
            .sum()
    }

    paths(&graph, graph.start, HashSet::new(), false).into()
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
    use indoc::indoc;

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
        assert_eq!(super::part_a(CASE), 10.into());
        assert_eq!(super::part_a(CASE_2), 19.into());
    }

    #[test]
    fn part_b() {
        assert_eq!(super::part_b(CASE), 36.into());
        assert_eq!(super::part_b(CASE_2), 103.into());
    }
}
