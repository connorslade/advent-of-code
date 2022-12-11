use std::{cell::RefCell, collections::VecDeque};

use crate::{problem, Solution};

pub struct Day11;

impl Solution for Day11 {
    fn name(&self) -> &'static str {
        "Monkey in the Middle"
    }

    fn part_a(&self) -> String {
        let raw = problem::load(2022, 11);
        let monkeys = parse_monkeys(&raw);

        process(monkeys, 20, |x| x / 3).to_string()
    }

    fn part_b(&self) -> String {
        let raw = problem::load(2022, 11);
        let monkeys = parse_monkeys(&raw);

        let magic = monkeys.iter().map(|x| x.test.divisor).product::<u64>();
        process(monkeys, 10000, |x| x % magic).to_string()
    }
}

struct Monkey {
    items: RefCell<VecDeque<u64>>,
    inspected: RefCell<usize>,
    operation: Operation,
    test: Test,
}

enum Operation {
    Add(Value),
    Multiply(Value),
}

struct Test {
    divisor: u64,
    // [true, false]
    monkey: [usize; 2],
}

enum Value {
    Old,
    Number(u64),
}

fn process(mut monkeys: Vec<Monkey>, iter: usize, proc: impl Fn(u64) -> u64) -> usize {
    for _ in 0..iter {
        for monkey in &monkeys {
            while let Some(mut item) = monkey.items.borrow_mut().pop_front() {
                *monkey.inspected.borrow_mut() += 1;
                item = proc(monkey.operation.process(item));
                monkeys[monkey.test.process(item)]
                    .items
                    .borrow_mut()
                    .push_back(item);
            }
        }
    }

    monkeys.sort_by_key(|x| -(*x.inspected.borrow() as isize));
    let a = *monkeys[0].inspected.borrow();
    let b = *monkeys[1].inspected.borrow();

    a * b
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
            items: RefCell::new(items),
            inspected: RefCell::new(0),
            operation,
            test,
        });
    }

    out
}

impl Operation {
    fn parse(inp: &str) -> Self {
        let mut parts = inp.split_whitespace();
        debug_assert_eq!(parts.next().unwrap(), "old");

        match parts.next().unwrap() {
            "*" => Self::Multiply(Value::parse(parts.next().unwrap())),
            "+" => Self::Add(Value::parse(parts.next().unwrap())),
            _ => panic!("Unsuppored operation"),
        }
    }

    fn process(&self, old: u64) -> u64 {
        match self {
            Self::Add(Value::Old) => old + old,
            Self::Add(Value::Number(n)) => old + n,
            Self::Multiply(Value::Old) => old * old,
            Self::Multiply(Value::Number(n)) => old * n,
        }
    }
}

impl Value {
    fn parse(inp: &str) -> Self {
        if inp == "old" {
            return Self::Old;
        }

        Self::Number(inp.parse::<u64>().unwrap())
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
