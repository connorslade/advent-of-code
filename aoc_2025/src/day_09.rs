use std::{cmp::Reverse, iter};

use common::{Answer, solution};
use itertools::Itertools;
use nd_vec::{Vec2, vector};

solution!("Movie Theater", 9);

fn part_a(input: &str) -> Answer {
    (parse(input).iter())
        .tuple_combinations()
        .map(|(a, b)| area(a, b))
        .max()
        .unwrap()
        .into()
}

fn part_b(input: &str) -> Answer {
    let red = parse(input);
    red.iter()
        .tuple_combinations()
        .map(|(a, b)| (area(a, b), (a, b)))
        .sorted_by_key(|(area, _)| Reverse(*area))
        .find(|(_area, (a, b))| {
            let bounds = bounds(a, b);
            !red.iter()
                .chain(iter::once(&red[0]))
                .tuple_windows()
                .any(|line| intersecting_line(line, bounds))
        })
        .unwrap()
        .0
        .into()
}

fn parse(input: &str) -> Vec<Vec2<u64>> {
    (input.lines())
        .map(|x| {
            let (x, y) = x.split_once(',').unwrap();
            vector!(x.parse().unwrap(), y.parse().unwrap())
        })
        .collect::<Vec<_>>()
}

fn area(a: &Vec2<u64>, b: &Vec2<u64>) -> u64 {
    (a.x().abs_diff(b.x()) + 1) * (a.y().abs_diff(b.y()) + 1)
}

fn bounds(a: &Vec2<u64>, b: &Vec2<u64>) -> (Vec2<u64>, Vec2<u64>) {
    (
        vector!(a.x().min(b.x()), a.y().min(b.y())),
        vector!(a.x().max(b.x()), a.y().max(b.y())),
    )
}

fn intersecting_line(
    (la, lb): (&Vec2<u64>, &Vec2<u64>),
    (min, max): (Vec2<u64>, Vec2<u64>),
) -> bool {
    let (lmin, lmax) = bounds(la, lb);

    la.x() == lb.x() // horizontal line
        && (((lmin.y() < min.y() && lmax.y() > min.y())
            || (lmin.y() < max.y() && lmax.y() > max.y())
            || (lmin.y() >= min.y() && lmax.y() <= max.y()))
            && la.x() > min.x()
            && la.x() < max.x())
        || la.y() == lb.y() // vertical line
            && (((lmin.x() < min.x() && lmax.x() > min.x())
                || (lmin.x() < max.x() && lmax.x() > max.x())
                || (lmin.x() >= min.x() && lmax.x() <= max.x()))
                && la.y() > min.y()
                && la.y() < max.y())
}

#[cfg(test)]
mod test {
    use indoc::indoc;

    const CASE: &str = indoc! {"
        7,1
        11,1
        11,7
        9,7
        9,5
        2,5
        2,3
        7,3
    "};

    #[test]
    fn part_a() {
        assert_eq!(super::part_a(CASE), 50.into());
    }

    #[test]
    fn part_b() {
        assert_eq!(super::part_b(CASE), 24.into());
    }
}
