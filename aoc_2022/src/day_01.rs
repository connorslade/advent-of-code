use common::{solution, Answer};

solution!("Calorie Counting", (2022, 00));

fn part_a(input: &str) -> Answer {
    let elfs = get_elfs(input);

    (*elfs.last().unwrap()).into()
}

fn part_b(input: &str) -> Answer {
    let elfs = get_elfs(input);

    elfs.iter().rev().take(3).sum::<u32>().into()
}

fn get_elfs(data: &str) -> Vec<u32> {
    let mut out = data
        .replace('\r', "")
        .split("\n\n")
        .map(|x| x.lines().map(|x| x.parse::<u32>().unwrap()).sum())
        .collect::<Vec<_>>();
    out.sort();
    out
}
