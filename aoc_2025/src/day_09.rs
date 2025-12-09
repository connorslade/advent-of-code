use std::{cmp::Reverse, iter};

use common::{Answer, solution};
use itertools::Itertools;
use nd_vec::{Vec2, vector};

solution!("Movie Theater", 9);

fn part_a(input: &str) -> Answer {
    parse(input)
        .iter()
        .combinations(2)
        .map(|x| (x[0].x().abs_diff(x[1].x()) + 1) * (x[0].y().abs_diff(x[1].y()) + 1))
        .max()
        .unwrap()
        .into()
}

fn part_b(input: &str) -> Answer {
    let red = parse(input);
    red.iter()
        .tuple_combinations()
        .map(|(a, b)| {
            (
                (a.x().abs_diff(b.x()) + 1) * (a.y().abs_diff(b.y()) + 1),
                (a, b),
            )
        })
        .sorted_by_key(|(area, _)| Reverse(*area))
        .find(|(_area, (a, b))| {
            let min = vector!(a.x().min(b.x()), a.y().min(b.y()));
            let max = vector!(a.x().max(b.x()), a.y().max(b.y()));

            for (&α, &β) in red.iter().chain(iter::once(&red[0])).tuple_windows() {
                let lmin = vector!(α.x().min(β.x()), α.y().min(β.y()));
                let lmax = vector!(α.x().max(β.x()), α.y().max(β.y()));

                if α.x() == β.x() {
                    let x = β.x();
                    if ((lmin.y() < min.y() && lmax.y() > min.y())
                        || (lmin.y() < max.y() && lmax.y() > max.y())
                        || (lmin.y() >= min.y() && lmax.y() <= max.y()))
                        && x > min.x()
                        && x < max.x()
                    {
                        return false;
                    }
                } else if α.y() == β.y() {
                    let y = β.y();
                    if ((lmin.x() < min.x() && lmax.x() > min.x())
                        || (lmin.x() < max.x() && lmax.x() > max.x())
                        || (lmin.x() >= min.x() && lmax.x() <= max.x()))
                        && y > min.y()
                        && y < max.y()
                    {
                        return false;
                    }
                }
            }

            true
        })
        .unwrap()
        .0
        .into()
}

// not: >1565730054

fn parse(input: &str) -> Vec<Vec2<i64>> {
    (input.lines())
        .map(|x| {
            let (x, y) = x.split_once(',').unwrap();
            vector!(x.parse().unwrap(), y.parse().unwrap())
        })
        .collect::<Vec<_>>()
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
