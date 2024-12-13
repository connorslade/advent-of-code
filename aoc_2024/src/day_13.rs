use std::collections::{HashMap, VecDeque};

use common::{solution, Answer};
use nd_vec::{vector, Vec2};

solution!("Claw Contraption", 13);

fn part_a(input: &str) -> Answer {
    let problem = Problem::parse(input);
    problem
        .cases
        .iter()
        .map(|x| x.cheapest())
        .filter(|&x| x != u64::MAX)
        .sum::<u64>()
        .into()
}

fn part_b(input: &str) -> Answer {
    /* i used mathematica for p2

    out = 0;
    Do[
      x = input[[i]];
      solve =
       Solve[x[[1]][[1]] == x[[2]][[1]]*a + x[[3]][[1]]*b &&
         x[[1]][[2]] == x[[2]][[2]]*a + x[[3]][[2]]*b, {a, b}, Integers];
      result = 3*solve[[All, 1, 2]] + solve[[All, 2, 2]];
      out += If[Length[result] == 0, {0}, result];
      , {i, 1, Length[input]}];
    out

    */

    Answer::Unimplemented
}

#[derive(Debug)]
struct Problem {
    cases: Vec<Case>,
}

#[derive(Debug)]
struct Case {
    a_button: Vec2<u64>,
    b_button: Vec2<u64>,
    goal: Vec2<u64>,
}

fn parse_button(input: &str) -> Vec2<u64> {
    let (_, parts) = input.rsplit_once(": ").unwrap();
    let (x, y) = parts.split_once(", ").unwrap();
    vector!(x[1..].parse().unwrap(), y[1..].parse().unwrap())
}

impl Problem {
    fn parse(input: &str) -> Self {
        let cases = input
            .split("\n\n")
            .map(|x| {
                let mut lines = x.split('\n');
                let a_button = parse_button(lines.next().unwrap());
                let b_button = parse_button(lines.next().unwrap());

                let (_, prize) = lines.next().unwrap().rsplit_once(": ").unwrap();
                let (x, y) = prize.split_once(", ").unwrap();
                let prize = vector!(x[2..].parse().unwrap(), y[2..].parse().unwrap());

                Case {
                    a_button,
                    b_button,
                    goal: prize,
                }
            })
            .collect();
        Self { cases }
    }

    fn part_b(mut self) -> Self {
        for case in self.cases.iter_mut() {
            case.goal += vector!(10000000000000, 10000000000000);
        }

        self
    }
}

impl Case {
    fn cheapest(&self) -> u64 {
        // a->3, b->1
        fn inner(
            case: &Case,
            memo: &mut HashMap<(Vec2<u64>, (u64, u64)), u64>,
            pos: Vec2<u64>,
            counts: (u64, u64),
            price: u64,
        ) -> u64 {
            if let Some(&cache) = memo.get(&(pos, counts)) {
                return cache;
            }

            if pos == case.goal {
                return price;
            }

            if counts.0 > 100 || counts.1 > 100 {
                return u64::MAX;
            }

            let min = inner(
                case,
                memo,
                pos + case.a_button,
                (counts.0 + 1, counts.1),
                price + 3,
            )
            .min(inner(
                case,
                memo,
                pos + case.b_button,
                (counts.0, counts.1 + 1),
                price + 1,
            ));

            memo.insert((pos, counts), min);

            min
        }

        inner(self, &mut HashMap::new(), vector!(0, 0), (0, 0), 0)
    }
}

#[cfg(test)]
mod test {
    use indoc::indoc;

    const CASE: &str = indoc! {"
        Button A: X+94, Y+34
        Button B: X+22, Y+67
        Prize: X=8400, Y=5400

        Button A: X+26, Y+66
        Button B: X+67, Y+21
        Prize: X=12748, Y=12176

        Button A: X+17, Y+86
        Button B: X+84, Y+37
        Prize: X=7870, Y=6450

        Button A: X+69, Y+23
        Button B: X+27, Y+71
        Prize: X=18641, Y=10279
    "};

    #[test]
    fn part_a() {
        assert_eq!(super::part_a(CASE), 480.into());
    }

    #[test]
    fn part_b() {
        assert_eq!(super::part_b(CASE), ().into());
    }
}
