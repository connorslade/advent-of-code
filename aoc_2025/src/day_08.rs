use std::{
    collections::{HashMap, HashSet},
    iter::repeat_n,
};

use common::{Answer, solution};
use itertools::Itertools;
use nd_vec::{Vec3, vector};

type Pos = Vec3<i64>;
type Pair = (Pos, Pos);

solution!("Playground", 8);

fn part_a(input: &str) -> Answer {
    let (boxes, closest_paris) = parse(input);

    let mut connections = HashMap::<_, Vec<_>>::new();
    let n = [1000, 10][cfg!(test) as usize];
    for (_dist, (a, b)) in closest_paris.iter().take(n) {
        connections.entry(*a).or_default().push(*b);
        connections.entry(*b).or_default().push(*a);
    }

    let mut seen = HashSet::new();
    let mut sizes = Vec::new();
    for start in connections.keys() {
        let size = subgraph_size(&connections, &mut seen, *start);
        (size > 0).then(|| sizes.push(size));
    }

    let unconnected = boxes.iter().filter(|x| !seen.contains(x)).count();
    sizes.extend(repeat_n(1, unconnected));
    sizes.sort();

    sizes.iter().rev().take(3).product::<u64>().into()
}

fn part_b(input: &str) -> Answer {
    let (boxes, closest_paris) = parse(input);

    let mut connections = HashMap::<_, Vec<_>>::new();
    for (_dist, (a, b)) in closest_paris.iter() {
        connections.entry(*a).or_default().push(*b);
        connections.entry(*b).or_default().push(*a);

        let mut seen = HashSet::new();
        for start in connections.keys() {
            let size = subgraph_size(&connections, &mut seen, *start);
            if size == boxes.len() as u64 {
                return (a.x() * b.x()).into();
            }
        }
    }

    ().into()
}

fn parse(input: &str) -> (Vec<Pos>, Vec<(i64, Pair)>) {
    let boxes = input
        .lines()
        .map(|x| {
            let mut coords = x.split(',').map(|x| x.parse::<i64>().unwrap());
            vector!(
                coords.next().unwrap(),
                coords.next().unwrap(),
                coords.next().unwrap()
            )
        })
        .collect::<Vec<_>>();

    let mut closest_paris = boxes
        .iter()
        .combinations(2)
        .map(|x| ((*x[0] - *x[1]).magnitude_squared(), (*x[0], *x[1])))
        .collect::<Vec<_>>();
    closest_paris.sort_by_key(|x| x.0);

    (boxes, closest_paris)
}

fn subgraph_size(
    connections: &HashMap<Vec3<i64>, Vec<Vec3<i64>>>,
    seen: &mut HashSet<Vec3<i64>>,
    node: Vec3<i64>,
) -> u64 {
    if !seen.insert(node) {
        return 0;
    }

    let mut out = 1;
    for child in &connections[&node] {
        out += subgraph_size(connections, seen, *child);
    }

    out
}

#[cfg(test)]
mod test {
    use indoc::indoc;

    const CASE: &str = indoc! {"
        162,817,812
        57,618,57
        906,360,560
        592,479,940
        352,342,300
        466,668,158
        542,29,236
        431,825,988
        739,650,466
        52,470,668
        216,146,977
        819,987,18
        117,168,530
        805,96,715
        346,949,466
        970,615,88
        941,993,340
        862,61,35
        984,92,344
        425,690,689
    "};

    #[test]
    fn part_a() {
        assert_eq!(super::part_a(CASE), 40.into());
    }

    #[test]
    fn part_b() {
        assert_eq!(super::part_b(CASE), 25272.into());
    }
}
