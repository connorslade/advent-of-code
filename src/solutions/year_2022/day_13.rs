use crate::{problem, Solution};

pub struct Day13;

impl Solution for Day13 {
    fn name(&self) -> &'static str {
        ""
    }

    fn part_a(&self) -> String {
        let raw = problem::load(2022, 13);
        let signals = parse(&raw);
        let mut correct = 0;

        signals
            .iter()
            .enumerate()
            .filter(|x| {
                if x.1.verify() {
                    println!("CORRECT: {:?}", x.1);
                } else {
                    println!("INCORRECT: {:?}", x.1);
                };
                x.1.verify()
            })
            .map(|x| 1 + x.0)
            .sum::<usize>()
            .to_string()
    }

    fn part_b(&self) -> String {
        let raw = problem::load(2022, 13);
        todo!()
    }
}

#[derive(Debug, Clone)]
enum Token {
    Number(u32),
    List(Vec<Token>),
}

#[derive(Debug, Clone)]
struct Signal {
    left: Vec<Token>,
    right: Vec<Token>,
}

fn parse(raw: &str) -> Vec<Signal> {
    raw.split("\n\n").map(Signal::parse).collect()
}

impl Signal {
    fn parse(raw: &str) -> Self {
        let mut raw_parts = raw.lines();
        let extract = |x: Vec<Token>| {
            if x.is_empty() {
                return vec![];
            }

            match &x[0] {
                Token::List(i) => i.to_vec(),
                Token::Number(_) => vec![x[0].clone()],
            }
        };

        Self {
            left: extract(tokenisze(raw_parts.next().unwrap())),
            right: extract(tokenisze(raw_parts.next().unwrap())),
        }
    }

    /*
    When comparing two values, the first value is called left and the second value is called right. Then:

    If both values are integers, the lower integer should come first. If the left integer is lower than the right integer, the inputs are in the right order. If the left integer is higher than the right integer, the inputs are not in the right order. Otherwise, the inputs are the same integer; continue checking the next part of the input.
    If both values are lists, compare the first value of each list, then the second value, and so on. If the left list runs out of items first, the inputs are in the right order. If the right list runs out of items first, the inputs are not in the right order. If the lists are the same length and no comparison makes a decision about the order, continue checking the next part of the input.
    If exactly one value is an integer, convert the integer to a list which contains that integer as its only value, then retry the comparison. For example, if comparing [0,0,0] and 2, convert the right value to [2] (a list containing 2); the result is then found by instead comparing [0,0,0] and [2].

    Using these rules, you can determine which of the pairs in the example are in the right order
    */
    fn verify(&self) -> bool {
        // println!("{:?}", self);
        let mut index = 0;

        loop {
            // If left is out of bounds, it's correct
            if index >= self.left.len() {
                return true;
            }

            // If right is out of bounds, it's incorrect
            if index >= self.right.len() {
                return false;
            }

            let left = &self.left[index];
            let right = &self.right[index];

            // If both are numbers, compare them
            if let (Token::Number(l), Token::Number(r)) = (left, right) {
                if l > r {
                    return false;
                }
            }

            // If both are lists, compare them
            if let (Token::List(l), Token::List(r)) = (left, right) {
                if {
                    !Signal {
                        left: l.to_vec(),
                        right: r.to_vec(),
                    }
                    .verify()
                } {
                    return false;
                }
            }

            let num_list = |num, list: Vec<Token>| {
                Signal {
                    left: vec![Token::Number(num)],
                    right: list.to_vec(),
                }
                .verify()
            };

            match (left, right) {
                (Token::Number(l), Token::List(r)) => {
                    if !num_list(*l, r.to_vec()) {
                        return false;
                    }
                }
                (Token::List(l), Token::Number(r)) => {
                    if !num_list(*r, l.to_vec()) {
                        return false;
                    }
                }
                _ => {}
            };

            index += 1;
        }

        true
    }
}

// [[4,4],4,4]
//   ^
//    list: 2
//     out:
// working:

fn tokenisze(raw: &str) -> Vec<Token> {
    let mut out = Vec::new();
    let mut working = String::new();
    let mut in_list = 0;

    for i in raw.chars().filter(|x| !x.is_ascii_whitespace()) {
        match i {
            '[' if in_list > 0 => in_list += 1,
            ']' if in_list > 0 => in_list -= 1,
            _ => {}
        }

        match i {
            '[' if in_list == 0 => {
                flush(&mut working, &mut out, false);
                in_list += 1;
            }
            ']' if in_list == 1 => {
                flush(&mut working, &mut out, true);
                in_list -= 1;
            }
            _ if in_list > 0 => working.push(i),

            ',' => flush(&mut working, &mut out, false),
            i if i.is_ascii_digit() => working.push(i),
            _ => {}
        }
    }

    if !working.is_empty() {
        flush(&mut working, &mut out, false);
    }

    out
}

fn flush(working: &mut String, out: &mut Vec<Token>, nest: bool) {
    if working.is_empty() {
        return;
    }

    if nest {
        out.push(Token::List(tokenisze(&working)));
        working.clear();
        return;
    }

    if let Ok(i) = working.parse::<u32>() {
        out.push(Token::Number(i));
    } else {
        out.push(Token::List(tokenisze(&working)));
    }

    working.clear();
}
