use common::{solution, Answer};

solution!("Bridge Repair", 7);

fn part_a(input: &str) -> Answer {
    let problem = Problem::parse(input);
    let mut sum = 0;

    for case in problem.cases {
        if case.is_valid_a() {
            sum += case.result;
        }
    }

    sum.into()
}

fn part_b(input: &str) -> Answer {
    let problem = Problem::parse(input);
    let mut sum = 0;

    for case in problem.cases {
        if case.is_valid_b_working() {
            sum += case.result;
        }
    }

    sum.into()
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
    fn is_valid_a(&self) -> bool {
        let op_count = self.inputs.len() - 1;
        let mut ops = vec![0; op_count];

        loop {
            let mut result = self.inputs[0];
            for (&op, &input) in ops.iter().zip(self.inputs.iter().skip(1)) {
                let op = [Operations::Add, Operations::Multiply, Operations::Concat][op];
                result = op.evaluate(result, input);
            }
                
                if result == self.result {
                    return true;
                }

            let mut i = 0;
            loop {
                if i >= op_count {
                    return false;
                }

                ops[i] += 1;
                if ops[i] > 1 {
                    ops[i] = 0;
                    i += 1;
                    continue;
                }

                break;
            }
        }
    }

    fn is_valid_b_working(&self) -> bool {
        let op_count = self.inputs.len() - 1;
        let mut ops = vec![0; op_count];

        dbg!(&self.inputs);
        loop {
            let mut result = self.inputs[0];
            // println!("{ops:?}");
            for (idx, &op) in ops.iter().enumerate() {
                let input = self.inputs[idx + 1];
                let op = [Operations::Add, Operations::Multiply, Operations::Concat][op];
                // println!(" | {op:?} ({result}, {input})");
                result = op.evaluate(result, input);
            }

            if result == self.result {
                // println!(" -> WORKS {result}");
                return true;
            }

            let mut i = 0;
            loop {
                if i >= op_count {
                    return false;
                }

                ops[i] += 1;
                if ops[i] > 2 {
                    ops[i] = 0;
                    i += 1;
                    continue;
                }

                break;
            }
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
