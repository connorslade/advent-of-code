use common::{Answer, solution};

solution!("Secret Entrance", 1);

fn part_a(input: &str) -> Answer {
    let mut pos = 50;
    let mut out = 0;

    for line in input.lines() {
        let num = line[1..].parse::<u32>().unwrap();
        match line.as_bytes()[0] {
            b'L' => pos = (100 + pos - num % 100) % 100,
            b'R' => pos = (pos + num) % 100,
            _ => unreachable!(),
        }

        out += (pos == 0) as u64;
    }

    out.into()
}

fn part_b(input: &str) -> Answer {
    let mut pos = 50;
    let mut out = 0;

    for line in input.lines() {
        let num = line[1..].parse::<u64>().unwrap();
        match line.as_bytes()[0] {
            b'L' => {
                out += (99 - (pos + 99) % 100 + num) / 100;
                pos = (100 + pos - num % 100) % 100;
            }
            b'R' => {
                out += (pos + num) / 100;
                pos = (pos + num) % 100;
            }
            _ => unreachable!(),
        }
    }

    out.into()
}

#[cfg(test)]
mod test {
    use indoc::indoc;

    const CASE: &str = indoc! {"
        L68
        L30
        R48
        L5
        R60
        L55
        L1
        L99
        R14
        L82
    "};

    #[test]
    fn part_a() {
        assert_eq!(super::part_a(CASE), 3.into());
    }

    #[test]
    fn part_b() {
        assert_eq!(super::part_b(CASE), 6.into());
    }
}
