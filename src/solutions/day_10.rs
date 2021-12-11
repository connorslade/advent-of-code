use crate::common::{self, Solution};

const CHARS: [(char, char); 4] = [('(', ')'), ('[', ']'), ('{', '}'), ('<', '>')];

pub struct Day10 {}

impl Solution for Day10 {
    fn name(&self) -> String {
        "Syntax Scoring".to_owned()
    }

    fn part_a(&self) -> String {
        let data = parse(common::load("10"));

        let mut total = 0;
        for i in data {
            let mut closeing = Vec::new();
            for j in i.chars() {
                if char_contains_key(j) {
                    closeing.push(char_for_char(j));
                    continue;
                }

                if closeing.is_empty() || j != closeing.pop().unwrap() {
                    total += match j {
                        ')' => 3,
                        ']' => 57,
                        '}' => 1197,
                        '>' => 25137,
                        _ => unreachable!(),
                    };
                    break;
                }
            }
        }

        total.to_string()
    }

    fn part_b(&self) -> String {
        let data = parse(common::load("10"));

        let mut scores = Vec::new();
        for i in data {
            let mut queue = Vec::new();
            let mut is_corrupted = false;
            for j in i.chars() {
                if char_contains_key(j) {
                    queue.push(char_for_char(j));
                    continue;
                }

                if queue.is_empty() || j != queue.pop().unwrap() {
                    is_corrupted = true;
                    break;
                }
            }

            if !is_corrupted {
                let mut score = 0;
                while !queue.is_empty() {
                    let ch = queue.pop().unwrap();
                    score = 5 * score
                        + match ch {
                            ')' => 1,
                            ']' => 2,
                            '}' => 3,
                            '>' => 4,
                            _ => unreachable!(),
                        };
                }
                scores.push(score);
            }
        }

        scores.sort_unstable();
        let mid = scores.len() / 2;
        scores[mid].to_string()
    }
}

fn parse(lines: String) -> Vec<String> {
    lines
        .lines()
        .map(|x| x.to_string())
        .collect::<Vec<String>>()
}

fn char_for_char(inp: char) -> char {
    for i in CHARS {
        if i.0 == inp {
            return i.1;
        }

        if i.1 == inp {
            return i.0;
        }
    }

    unreachable!()
}

fn char_contains_key(inp: char) -> bool {
    for i in CHARS {
        if i.0 == inp {
            return true;
        }
    }
    false
}
