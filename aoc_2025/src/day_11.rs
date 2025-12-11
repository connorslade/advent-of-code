use std::collections::HashMap;

use common::{Answer, solution};

solution!("Reactor", 11);

fn part_a(input: &str) -> Answer {
    let mut map = HashMap::new();

    for line in input.lines() {
        let (from, to) = line.split_once(": ").unwrap();
        let to = to.split_whitespace().collect::<Vec<_>>();
        map.insert(from, to);
    }

    fn count_paths(map: &HashMap<&str, Vec<&str>>, current: &str) -> u64 {
        if current == "out" {
            return 1;
        }

        let mut out = 0;

        for child in &map[current] {
            out += count_paths(map, child);
        }

        out
    }

    count_paths(&map, "you").into()
}

fn part_b(input: &str) -> Answer {
    let mut map = HashMap::new();

    for line in input.lines() {
        let (from, to) = line.split_once(": ").unwrap();
        let to = to.split_whitespace().collect::<Vec<_>>();
        map.insert(from, to);
    }

    fn count_paths<'a>(
        memo: &mut HashMap<(&'a str, bool, bool), u64>,
        map: &HashMap<&str, Vec<&'a str>>,
        current: &'a str,
        mut fft: bool,
        mut dac: bool,
    ) -> u64 {
        let key = (current, fft, dac);
        if let Some(memo) = memo.get(&key) {
            return *memo;
        }

        if current == "out" {
            if fft && dac {
                return 1;
            }
            return 0;
        }

        if current == "fft" {
            fft = true;
        }

        if current == "dac" {
            dac = true;
        }

        let mut out = 0;
        for child in &map[current] {
            out += count_paths(memo, map, child, fft, dac);
        }

        memo.insert(key, out);
        out
    }

    count_paths(&mut HashMap::new(), &map, "svr", false, false).into()
}

#[cfg(test)]
mod test {
    use indoc::indoc;

    const CASE: &str = indoc! {"
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
        assert_eq!(super::part_a(CASE), 5.into());
    }

    #[test]
    fn part_b() {
        assert_eq!(super::part_b(CASE), 2.into());
    }
}
