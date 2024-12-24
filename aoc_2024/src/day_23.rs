use std::collections::{HashMap, HashSet};

use common::{solution, Answer};
use itertools::Itertools;

solution!("LAN Party", 23);

fn part_a(input: &str) -> Answer {
    let graph = parse(input);

    let mut triplets = HashSet::new();
    for key in graph.keys() {
        let neighbors = &graph[key];

        for x in neighbors {
            for y in neighbors.iter().skip(1) {
                if graph[x].contains(y) {
                    let mut sorted = vec![key, x, y];
                    sorted.sort();
                    triplets.insert((sorted[0], sorted[1], sorted[2]));
                }
            }
        }
    }

    triplets
        .iter()
        .filter(|(a, b, c)| a.starts_with('t') || b.starts_with('t') || c.starts_with('t'))
        .count()
        .into()
}

fn part_b(input: &str) -> Answer {
    let graph = parse(input);

    let mut cliques = Vec::new();
    bron_kerbosch(
        &graph,
        &mut HashSet::new(),
        &mut graph.keys().cloned().collect(),
        &mut HashSet::new(),
        &mut cliques,
    );

    let max = cliques.iter().max_by_key(|x| x.len()).unwrap();
    max.iter().sorted().join(",").into()
}

fn parse(input: &str) -> HashMap<&str, HashSet<&str>> {
    let mut out: HashMap<&str, HashSet<&str>> = HashMap::new();

    for line in input.lines() {
        let (a, b) = line.split_once('-').unwrap();
        out.entry(a).or_default().insert(b);
        out.entry(b).or_default().insert(a);
    }

    out
}

fn bron_kerbosch<'a>(
    graph: &HashMap<&'a str, HashSet<&'a str>>,
    r: &mut HashSet<&'a str>,
    p: &mut HashSet<&'a str>,
    x: &mut HashSet<&'a str>,
    cliques: &mut Vec<HashSet<&'a str>>,
) {
    if p.is_empty() && x.is_empty() {
        cliques.push(r.clone());
        return;
    }

    let pivot = p.iter().chain(x.iter()).next();

    for &v in p.difference(&pivot.map_or_else(HashSet::new, |p| {
        graph.get(p).cloned().unwrap_or(HashSet::new())
    })) {
        let mut r = r.clone();
        r.insert(v);

        let mut p = p
            .intersection(graph.get(&v).unwrap_or(&HashSet::new()))
            .cloned()
            .collect();
        let mut x = x
            .intersection(graph.get(&v).unwrap_or(&HashSet::new()))
            .cloned()
            .collect();

        bron_kerbosch(graph, &mut r, &mut p, &mut x, cliques);

        p.remove(&v);
        x.insert(v);
    }
}

#[cfg(test)]
mod test {
    use indoc::indoc;

    const CASE: &str = indoc! {"
        kh-tc
        qp-kh
        de-cg
        ka-co
        yn-aq
        qp-ub
        cg-tb
        vc-aq
        tb-ka
        wh-tc
        yn-cg
        kh-ub
        ta-co
        de-co
        tc-td
        tb-wq
        wh-td
        ta-ka
        td-qp
        aq-cg
        wq-ub
        ub-vc
        de-ta
        wq-aq
        wq-vc
        wh-yn
        ka-de
        kh-ta
        co-tc
        wh-qp
        tb-vc
        td-yn
    "};

    #[test]
    fn part_a() {
        assert_eq!(super::part_a(CASE), 7.into());
    }

    #[test]
    fn part_b() {
        assert_eq!(super::part_b(CASE), "co,de,ka,ta".into());
    }
}
