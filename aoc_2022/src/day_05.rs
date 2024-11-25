use common::{solution, Answer};

solution!("Supply Stacks", (2022, 00));

fn part_a(input: &str) -> Answer {
    process(input, true).into()
}

fn part_b(input: &str) -> Answer {
    process(input, false).into()
}

fn process(raw: &str, part: bool) -> String {
    let raw = raw.replace('\r', "");
    let (crates, orders) = raw.split_once("\n\n").unwrap();
    let mut crates = parse_crates(crates);

    for i in orders.trim().lines() {
        let parts = i.split_whitespace().collect::<Vec<_>>();
        let count = parts[1].parse::<usize>().unwrap();
        let from = parts[3].parse::<usize>().unwrap() - 1;
        let to = parts[5].parse::<usize>().unwrap() - 1;

        let count = crates[from].len() - count..;
        let mut working = crates[from].drain(count).collect::<Vec<_>>();
        if part {
            working.reverse();
        }

        crates[to].extend(working);
    }

    crates
        .into_iter()
        .filter(|x| !x.is_empty())
        .map(|mut x| x.pop().unwrap())
        .collect()
}

fn parse_crates(raw: &str) -> Vec<Vec<char>> {
    let raw_len = raw.lines().next().unwrap().len() + 1;
    let mut out = vec![Vec::new(); raw_len / 4];

    for i in raw.lines().filter(|x| !x.starts_with(" 1")) {
        for (ji, i) in i.chars().enumerate().filter(|x| x.1.is_ascii_uppercase()) {
            out[(ji as f32 / 4.).ceil() as usize - 1].insert(0, i);
        }
    }

    out
}
