use common::{solution, Answer};

solution!("Mull It Over", 3);

fn part_a(input: &str) -> Answer {
    solve(input, false).into()
}

fn part_b(input: &str) -> Answer {
    solve(input, true).into()
}

fn solve(input: &str, part_b: bool) -> u32 {
    let mut out = 0;

    let mut parser = Parser::new(input);
    let mut active = true;

    while !parser.is_eof() {
        active |= parser.expect("do()");
        active &= !parser.expect("don't()");

        if parser.expect("mul(") {
            let Some(a) = parser.number() else { continue };
            if !parser.expect(",") {
                continue;
            }
            let Some(b) = parser.number() else { continue };
            if !parser.expect(")") {
                continue;
            }

            if active || !part_b {
                out += a * b;
            }
        } else {
            parser.advance(1);
        }
    }

    out
}

struct Parser {
    chars: Vec<char>,
    idx: usize,
}

impl Parser {
    pub fn new(input: &str) -> Self {
        Self {
            chars: input.chars().collect(),
            idx: 0,
        }
    }

    pub fn expect(&mut self, str: &str) -> bool {
        let valid = self.idx + str.len() < self.chars.len()
            && self.chars[self.idx..self.idx + str.len()]
                .iter()
                .zip(str.chars())
                .all(|(&a, b)| a == b);

        if valid {
            self.idx += str.len();
        }

        valid
    }

    pub fn number(&mut self) -> Option<u32> {
        let mut working = String::new();
        while self.chars[self.idx].is_ascii_digit() && self.idx < self.chars.len() {
            working.push(self.chars[self.idx]);
            self.idx += 1;
        }
        working.parse::<u32>().ok()
    }

    pub fn advance(&mut self, count: usize) {
        self.idx += count;
    }

    pub fn is_eof(&self) -> bool {
        self.idx >= self.chars.len()
    }
}
#[cfg(test)]
mod test {
    use indoc::indoc;

    const CASE_A: &str = indoc! {"
        xmul(2,4)%&mul[3,7]!@^do_not_mul(5,5)+mul(32,64]then(mul(11,8)mul(8,5))
    "};

    const CASE_B: &str = indoc! {"
        xmul(2,4)&mul[3,7]!^don't()_mul(5,5)+mul(32,64](mul(11,8)undo()?mul(8,5))
    "};

    #[test]
    fn part_a() {
        assert_eq!(super::part_a(CASE_A), 161.into());
    }

    #[test]
    fn part_b() {
        assert_eq!(super::part_b(CASE_B), 48.into());
    }
}
