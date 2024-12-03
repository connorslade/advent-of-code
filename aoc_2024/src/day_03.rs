use aoc_lib::regex;
use common::{solution, Answer};

solution!("Mull It Over", 3);

fn part_a(input: &str) -> Answer {
    let instructions = parse(input);
    let mut acc = 0;
    for ins in instructions {
        acc += ins.a * ins.b;
    }

    acc.into()
}

fn part_b(input: &str) -> Answer {
    let instructions = parse_b(input);
    let mut acc = 0;
    for ins in instructions {
        acc += ins.a * ins.b;
    }

    acc.into()
}

#[derive(Debug)]
struct Mul {
    a: u32,
    b: u32,
}

fn parse(input: &str) -> Vec<Mul> {
    let mut out = Vec::new();

    let matches = regex!(r"mul\((\d*),(\d*)\)").captures_iter(input);

    for x in matches {
        let a = dbg!(x.get(1).unwrap().as_str()).parse::<u32>().unwrap();
        let b = dbg!(x.get(2).unwrap().as_str()).parse::<u32>().unwrap();

        out.push(Mul { a, b });
    }

    out
}

fn cmp_chars(chars: &[char], str: &str) -> bool {
    chars.len() == str.len() && str.chars().zip(chars).all(|(a, &b)| a == b)
}

fn parse_b(input: &str) -> Vec<Mul> {
    let mut out = Vec::new();

    let chars = input.chars().collect::<Vec<_>>();

    let mut i = 0;
    let mut active = true;

    let next_num = |i: &mut usize| {
        let mut working = String::new();
        while chars[*i].is_ascii_digit() && *i < chars.len() {
            working.push(chars[*i]);
            *i += 1;
        }
        working.parse::<u32>().ok()
    };

    while i < chars.len() {
        if i + 4 < chars.len() && cmp_chars(&chars[i..i + 4], "do()") {
            active = true;
        }

        if i + 7 < chars.len() && cmp_chars(&chars[i..i + 7], "don't()") {
            println!("dont");
            active = false;
        }

        if i + 3 < chars.len() && cmp_chars(&chars[i..i + 3], "mul") {
            i += 3;

            if chars[i] != '(' {
                i += 1;
                continue;
            }
            i += 1;

            let Some(a) = next_num(&mut i) else {
                i += 1;
                continue;
            };

            if chars[i] != ',' {
                i += 1;
                continue;
            }

            i += 1;
            let Some(b) = next_num(&mut i) else {
                i += 1;
                println!("failed b");
                continue;
            };

            if chars[i] != ')' {
                i += 1;
                continue;
            }

            let ins = Mul { a, b };

            if active {
                out.push(ins);
            }
        }

        i += 1;
    }

    out
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
