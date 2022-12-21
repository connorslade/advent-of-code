use hashbrown::HashMap;

use crate::{problem, Solution};

pub struct Day21;

impl Solution for Day21 {
    fn name(&self) -> &'static str {
        "Monkey Math"
    }

    fn part_a(&self) -> String {
        let raw = problem::load(2022, 21);
        let monkeys = MonkeyBusiness::new(&raw);
        monkeys.evaluate("root").to_string()
    }

    fn part_b(&self) -> String {
        let raw = problem::load(2022, 21);
        let _monkeys = MonkeyBusiness::new(&raw).root_eq();
        todo!()
    }
}

#[derive(Debug, Clone)]
enum Value {
    Number(i64),
    Operation(Operation),
}

#[derive(Debug, Clone, PartialEq)]
enum Operator {
    Assert,
    Add,
    Subtract,
    Multiply,
    Divide,
}

#[derive(Debug, Clone)]
struct Operation {
    operator: Operator,
    operands: [String; 2],
}

#[derive(Debug, Clone)]
struct MonkeyBusiness {
    data: HashMap<String, Value>,
}

impl MonkeyBusiness {
    fn new(raw: &str) -> Self {
        let mut data = HashMap::new();

        for i in raw.lines() {
            let (monkey, math) = i.split_once(": ").unwrap();
            data.insert(monkey.to_owned(), Value::new(math));
        }

        Self { data }
    }

    fn evaluate(&self, monkey: &str) -> i64 {
        self._evaluate(monkey).unwrap()
    }

    fn _evaluate(&self, monkey: &str) -> Option<i64> {
        match self.data.get(monkey) {
            Some(Value::Number(n)) => Some(*n),
            Some(Value::Operation(o)) => o.evaluate(self),
            None => None,
        }
    }

    fn root_eq(mut self) -> Self {
        let root = self.data.get_mut("root").unwrap();
        if let Value::Operation(o) = root {
            o.operator = Operator::Assert;
        }
        self
    }
}

impl Value {
    fn new(raw: &str) -> Self {
        if let Ok(n) = raw.parse() {
            return Self::Number(n);
        }
        Self::Operation(Operation::new(raw))
    }

    fn as_operation(&self) -> &Operation {
        match self {
            Self::Operation(o) => o,
            _ => panic!("Not an operation"),
        }
    }
}

impl Operation {
    fn new(raw: &str) -> Self {
        let op = &raw[5..6];
        let operands = [raw[0..4].to_string(), raw[7..11].to_string()];
        let operator = match op {
            "+" => Operator::Add,
            "-" => Operator::Subtract,
            "*" => Operator::Multiply,
            "/" => Operator::Divide,
            _ => panic!("Invalid operator: {}", op),
        };
        Self { operator, operands }
    }

    fn evaluate(&self, monkeys: &MonkeyBusiness) -> Option<i64> {
        let a = monkeys._evaluate(&self.operands[0])?;
        let b = monkeys._evaluate(&self.operands[1])?;
        Some(match self.operator {
            Operator::Add => a + b,
            Operator::Subtract => a - b,
            Operator::Multiply => a * b,
            Operator::Divide => a / b,
            Operator::Assert => return None,
        })
    }
}
