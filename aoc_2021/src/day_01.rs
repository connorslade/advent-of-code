use common::{solution, Answer};

solution!("Sonar Sweep", 1);

fn part_a(input: &str) -> Answer {
    let data = input
        .lines()
        .map(|x| x.parse::<u32>().unwrap())
        .collect::<Vec<u32>>();

    data.windows(2).filter(|x| x[0] < x[1]).count().into()
}

fn part_b(input: &str) -> Answer {
    let d = input
        .lines()
        .map(|x| x.parse::<u32>().unwrap())
        .collect::<Vec<u32>>();

    d.windows(4).filter(|x| x[2] > x[0]).count().into()
}
