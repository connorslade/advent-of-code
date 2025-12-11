use std::collections::HashMap;

use common::{Answer, solution};

solution!("Reactor", 11);

fn part_a(input: &str) -> Answer {
    count_paths(&mut HashMap::new(), &parse(input), "you", [true; _]).into()
}

fn part_b(input: &str) -> Answer {
    count_paths(&mut HashMap::new(), &parse(input), "svr", [false; _]).into()
}

fn count_paths<'a>(
    memo: &mut HashMap<(&'a str, [bool; 2]), u64>,
    map: &HashMap<&'a str, Vec<&'a str>>,
    current: &'a str,
    seen @ [fft, dac]: [bool; 2],
) -> u64 {
    let entry = (current, seen);
    if let Some(memo) = memo.get(&entry) {
        return *memo;
    }

    if current == "out" {
        return (fft && dac) as u64;
    }

    let seen = [fft || current == "fft", dac || current == "dac"];
    let out = (map[current].iter())
        .map(|child| count_paths(memo, map, child, seen))
        .sum();

    memo.insert(entry, out);
    out
}

fn parse(input: &str) -> HashMap<&str, Vec<&str>> {
    let mut map = HashMap::new();
    for line in input.lines() {
        let (from, to) = line.split_once(": ").unwrap();
        let to = to.split_whitespace().collect::<Vec<_>>();
        map.insert(from, to);
    }

    map
}

#[cfg(test)]
mod test {
    use indoc::indoc;

    const CASE_A: &str = indoc! {"
        aaa: you hhh
        you: bbb ccc
        bbb: ddd eee
        ccc: ddd eee fff
        ddd: ggg
        eee: out
        fff: out
        ggg: out
        hhh: ccc fff iii
        iii: out
    "};

    const CASE_B: &str = indoc! {"
        svr: aaa bbb
        aaa: fft
        fft: ccc
        bbb: tty
        tty: ccc
        ccc: ddd eee
        ddd: hub
        hub: fff
        eee: dac
        dac: fff
        fff: ggg hhh
        ggg: out
        hhh: out
    "};

    #[test]
    fn part_a() {
        assert_eq!(super::part_a(CASE_A), 5.into());
    }

    #[test]
    fn part_b() {
        assert_eq!(super::part_b(CASE_B), 2.into());
    }
}
