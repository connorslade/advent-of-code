use common::{Answer, ISolution};

pub struct Day01;

impl ISolution for Day01 {
    fn name(&self) -> &'static str {
        "Calorie Counting"
    }

    fn part_a(&self, input: &str) -> Answer {
        let elfs = get_elfs(input);

        (*elfs.last().unwrap()).into()
    }

    fn part_b(&self, input: &str) -> Answer {
        let elfs = get_elfs(input);

        elfs.iter().rev().take(3).sum::<u32>().into()
    }
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
