use aoc_lib::vector::IntoTuple2;
use common::{solution, Answer};
use nd_vec::{vector, Vec2};

solution!("Claw Contraption", 13);

fn part_a(input: &str) -> Answer {
    Problem::parse(input).solve().into()
}

fn part_b(input: &str) -> Answer {
    Problem::parse(input).part_b().solve().into()
}

struct Problem {
    cases: Vec<Case>,
}

struct Case {
    a_button: Vec2<u64>,
    b_button: Vec2<u64>,
    goal: Vec2<u64>,
}

impl Case {
    fn cheapest(&self) -> u64 {
        let cast = |x: Vec2<u64>| x.try_cast::<i64>().unwrap().into_tuple();
        let ((gx, gy), (ax, ay), (bx, by)) =
            (cast(self.goal), cast(self.a_button), cast(self.b_button));

        // The best a and b values for a case are the solutions to the following
        // system of equations, where g is the goal position, a/b are the number
        // of times you press the corespondent buttons (what we are solving
        // for), and (a|b)(x|y) are the offsets applied by each button press.
        //
        // gx = ax * a + bx * b
        // gy = ay * a + by * b
        //
        // By plugging those into Wolfram Alpha, I got the below equation for a,
        // then used that to find the equation of b. Because this is integer
        // math, we need to verify it by making sure a and b are greater than
        // zero and checking if the solution actually solves the system. If it
        // does, we return 3a + b, the price.

        let a = (by * gx - bx * gy) / (ax * by - ay * bx);
        let b = (gx - ax * a) / bx;

        if a <= 0 || b <= 0 || self.goal != self.a_button * a as u64 + self.b_button * b as u64 {
            return 0;
        }

        a as u64 * 3 + b as u64
    }
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

    fn solve(&self) -> u64 {
        self.cases.iter().map(|x| x.cheapest()).sum::<u64>()
    }

    fn part_b(mut self) -> Self {
        self.cases
            .iter_mut()
            .for_each(|case| case.goal += vector!(10000000000000, 10000000000000));
        self
    }
}

fn parse_button(input: &str) -> Vec2<u64> {
    let (_, parts) = input.rsplit_once(": ").unwrap();
    let (x, y) = parts.split_once(", ").unwrap();
    vector!(x[1..].parse().unwrap(), y[1..].parse().unwrap())
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
        assert_eq!(super::part_b(CASE), 875318608908_u64.into());
    }
}
