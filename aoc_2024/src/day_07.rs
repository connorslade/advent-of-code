use common::{solution, Answer};

solution!("Bridge Repair", 7);

fn part_a(input: &str) -> Answer {
    solve(input, false).into()
}

fn part_b(input: &str) -> Answer {
    solve(input, true).into()
}

fn solve(input: &str, part_b: bool) -> u64 {
    let problem = Problem::parse(input);

    // For each of the equations, we will add its result value if it is valid.
    // For part a, we check if its valid using `is_valid` with part_b = false,
    // because an equation that is valid for part a is must be valid for part b,
    // we can get a small speedup by only doing the more intense part_b = true
    // check if needed.
    problem
        .cases
        .into_iter()
        .filter(|x| x.is_valid(false) || (part_b && x.is_valid(true)))
        .map(|x| x.result)
        .sum::<u64>()
}

struct Problem {
    cases: Vec<TestCase>,
}

struct TestCase {
    result: u64,
    inputs: Vec<u64>,
}

#[derive(Clone, Copy, Debug)]
enum Operations {
    Add,
    Multiply,
    Concat,
}

impl Problem {
    fn parse(input: &str) -> Self {
        let cases = input
            .lines()
            .map(|x| {
                let (result, inputs) = x.split_once(": ").unwrap();

                let result = result.parse::<u64>().unwrap();
                let inputs = inputs
                    .split_whitespace()
                    .map(|x| x.parse::<u64>().unwrap())
                    .collect::<Vec<_>>();

                TestCase { result, inputs }
            })
            .collect::<Vec<_>>();

        Self { cases }
    }
}

impl TestCase {
    fn is_valid(&self, part_b: bool) -> bool {
        let op_count = self.inputs.len() - 1;
        let mut ops = vec![0; op_count];

        'outer: loop {
            // Set the result to be the first input value to start. Then update
            // the result for each operation using the previous result and the
            // next number as inputs.
            let mut result = self.inputs[0];
            for (&op, &input) in ops.iter().zip(self.inputs.iter().skip(1)) {
                let op = [Operations::Add, Operations::Multiply, Operations::Concat][op];
                result = op.evaluate(result, input);
            }

            // If the result we get after applying the operations gets us the
            // expected result, this equation is valid.
            if result == self.result {
                return true;
            }

            // Increments the leftmost operation, carrying if it exceeds 1 for
            // part a or 2 for part b.
            for op in ops.iter_mut() {
                *op += 1;
                if *op <= (1 + part_b as usize) {
                    continue 'outer;
                }
                *op = 0;
            }

            return false;
        }
    }
}

impl Operations {
    fn evaluate(&self, a: u64, b: u64) -> u64 {
        match self {
            Operations::Add => a + b,
            Operations::Multiply => a * b,
            Operations::Concat => {
                let mut out = a;
                let mut tmp = b;
                while tmp > 0 {
                    tmp /= 10;
                    out *= 10;
                }
                out + b
            }
        }
    }
}

#[cfg(test)]
mod test {
    use indoc::indoc;

    const CASE: &str = indoc! {"
        190: 10 19
        3267: 81 40 27
        83: 17 5
        156: 15 6
        7290: 6 8 6 15
        161011: 16 10 13
        192: 17 8 14
        21037: 9 7 18 13
        292: 11 6 16 20
    "};

    #[test]
    fn part_a() {
        assert_eq!(super::part_a(CASE), 3749.into());
    }

    #[test]
    fn part_b() {
        assert_eq!(super::part_b(CASE), 11387.into());
    }
}
