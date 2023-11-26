use hashbrown::HashMap;

use common::Solution;

pub struct Day21;

impl Solution for Day21 {
    fn name(&self) -> &'static str {
        "Monkey Math"
    }

    fn part_a(&self, input: &str) -> String {
        let monkeys = MonkeyBusiness::new(input);
        monkeys.evaluate("root").to_string()
    }

    fn part_b(&self, input: &str) -> String {
        let monkeys = MonkeyBusiness::new(input).root_eq();
        monkeys.solve("root").to_string()
    }
}

#[derive(Debug, Clone)]
enum Value {
    Number(i64),
    Operation(Operation),
}

#[derive(Debug, Clone, PartialEq)]
enum Operator {
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

    // == Evaluate ==
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

    // == Solve ==
    fn solve(&self, monkey: &str) -> i64 {
        self._solve(monkey, 0)
    }

    fn _solve(&self, name: &str, v: i64) -> i64 {
        match self.data.get(name).unwrap() {
            Value::Number(_) => v,
            Value::Operation(o) => {
                let l = &o.operands[0];
                let r = &o.operands[1];

                let hl = self._to_satisfy(l);
                match o.operator {
                    Operator::Add if hl => self._solve(l, v - self.evaluate(r)),
                    Operator::Subtract if hl => self._solve(l, v + self.evaluate(r)),
                    Operator::Multiply if hl => self._solve(l, v / self.evaluate(r)),
                    Operator::Divide if hl => self._solve(l, v * self.evaluate(r)),
                    Operator::Add => self._solve(r, v - self.evaluate(l)),
                    Operator::Subtract => self._solve(r, self.evaluate(l) - v),
                    Operator::Multiply => self._solve(r, v / self.evaluate(l)),
                    Operator::Divide => self._solve(r, self.evaluate(l) / v),
                }
            }
        }
    }

    fn _to_satisfy(&self, name: &str) -> bool {
        name == "humn"
            || match self.data.get(name).unwrap() {
                Value::Number(_) => false,
                Value::Operation(o) => {
                    self._to_satisfy(&o.operands[0]) || self._to_satisfy(&o.operands[1])
                }
            }
    }

    fn root_eq(mut self) -> Self {
        let root = self.data.get_mut("root").unwrap();
        if let Value::Operation(o) = root {
            o.operator = Operator::Subtract;
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
        })
    }
}
