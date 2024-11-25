use std::collections::VecDeque;

use common::{solution, Answer};

solution!("Monkey in the Middle", (2022, 00));

fn part_a(input: &str) -> Answer {
    let monkeys = parse_monkeys(input);

    process(monkeys, 20, |x| x / 3).into()
}

fn part_b(input: &str) -> Answer {
    let monkeys = parse_monkeys(input);

    let magic = monkeys.iter().map(|x| x.test.divisor).product::<u64>();
    process(monkeys, 10000, |x| x % magic).into()
}

struct Monkey {
    items: VecDeque<u64>,
    inspected: usize,
    operation: Operation,
    test: Test,
}

enum Operation {
    Add(u64),
    Multiply(u64),
    Square,
}

struct Test {
    divisor: u64,
    // [true, false]
    monkey: [usize; 2],
}

fn process(mut monkeys: Vec<Monkey>, iter: usize, proc: impl Fn(u64) -> u64) -> usize {
    for _ in 0..iter {
        for m in 0..monkeys.len() {
            while let Some(item) = monkeys[m].items.pop_front() {
                monkeys[m].inspected += 1;
                let item = proc(monkeys[m].operation.process(item));
                let goto = monkeys[m].test.process(item);
                monkeys[goto].items.push_back(item);
            }
        }
    }

    monkeys.sort_unstable_by_key(|x| x.inspected);
    monkeys.pop().unwrap().inspected * monkeys.pop().unwrap().inspected
}

fn parse_monkeys(raw: &str) -> Vec<Monkey> {
    let mut out = Vec::new();

    for i in raw.lines().collect::<Vec<_>>().chunks(7) {
        let items = i[1]
            .split_once(": ")
            .unwrap()
            .1
            .split(", ")
            .map(|x| x.parse::<u64>().unwrap())
            .collect::<VecDeque<_>>();

        let operation = Operation::parse(i[2].split_once(" = ").unwrap().1);
        let test = Test::parse(&i[3..6]);

        out.push(Monkey {
            items,
            inspected: 0,
            operation,
            test,
        });
    }

    out
}

impl Operation {
    fn parse(inp: &str) -> Self {
        let mut parts = inp.split_whitespace();
        assert_eq!(parts.next().unwrap(), "old");

        let op = parts.next().unwrap();
        let value = parts.next().unwrap();
        match op {
            "*" => match value {
                "old" => Self::Square,
                _ => Self::Multiply(value.parse::<u64>().unwrap()),
            },
            "+" => Self::Add(value.parse::<u64>().unwrap()),
            _ => panic!("Unsuppored operation"),
        }
    }

    fn process(&self, old: u64) -> u64 {
        match self {
            Self::Add(x) => old + x,
            Self::Multiply(x) => old * x,
            Self::Square => old * old,
        }
    }
}

impl Test {
    fn parse(inp: &[&str]) -> Self {
        let divisor = inp[0].split_once("by ").unwrap().1.parse::<u64>().unwrap();

        let mut monkey = [0; 2];
        for (i, line) in inp[1..].iter().enumerate() {
            let monkey_id = line
                .split_once("monkey ")
                .unwrap()
                .1
                .parse::<usize>()
                .unwrap();
            monkey[i] = monkey_id;
        }

        Self { divisor, monkey }
    }

    fn process(&self, item: u64) -> usize {
        if item % self.divisor == 0 {
            return self.monkey[0];
        }
        self.monkey[1]
    }
}
