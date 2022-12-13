use std::cmp::Ordering;

use crate::{problem, Solution};

pub struct Day13;

impl Solution for Day13 {
    fn name(&self) -> &'static str {
        "Distress Signal"
    }

    fn part_a(&self) -> String {
        let raw = problem::load(2022, 13);
        let signals = parse(&raw);

        signals
            .chunks(2)
            .enumerate()
            .filter(|x| x.1[0].cmp(&x.1[1]) == Ordering::Less)
            .map(|x| 1 + x.0)
            .sum::<usize>()
            .to_string()
    }

    fn part_b(&self) -> String {
        let raw = problem::load(2022, 13);
        let mut signals = parse(&raw);
        let div = [Token::Number(6), Token::Number(2)];
        signals.extend(div.clone());
        signals.sort();

        signals
            .iter()
            .enumerate()
            .filter(|x| div.contains(x.1))
            .map(|x| x.0 + 1)
            .product::<usize>()
            .to_string()
    }
}

#[derive(Debug, Clone, PartialEq, Eq)]
enum Token {
    Number(u32),
    List(Vec<Token>),
}

fn parse(raw: &str) -> Vec<Token> {
    raw.split("\n\n")
        .flat_map(|x| {
            x.lines()
                .map(tokenisze)
                .map(Token::List)
                .collect::<Vec<_>>()
        })
        .collect()
}

impl Ord for Token {
    fn cmp(&self, other: &Self) -> Ordering {
        match (self, other) {
            (Token::Number(l), Token::Number(r)) => l.cmp(r),
            (Token::List(l), Token::List(r)) => l.cmp(r),
            (Token::Number(l), Token::List(r)) => vec![Token::Number(*l)].cmp(r),
            (Token::List(l), Token::Number(r)) => l.cmp(&vec![Token::Number(*r)]),
        }
    }
}

impl PartialOrd for Token {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        Some(self.cmp(other))
    }
}

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
            '[' if in_list == 0 => in_list += 1,
            ']' if in_list == 1 => {
                flush(&mut working, &mut out);
                in_list -= 1;
            }
            _ if in_list > 0 => working.push(i),

            ',' => flush(&mut working, &mut out),
            i if i.is_ascii_digit() => working.push(i),
            _ => {}
        }
    }

    flush(&mut working, &mut out);
    out
}

fn flush(working: &mut String, out: &mut Vec<Token>) {
    if working.is_empty() {
        return;
    }

    match working.parse::<u32>() {
        Ok(i) => out.push(Token::Number(i)),
        Err(_) => out.push(Token::List(tokenisze(working))),
    }
    working.clear();
}
