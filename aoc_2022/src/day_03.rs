use hashbrown::HashSet;

use common::{solution, Answer};

solution!("Rucksack Reorganization", 3);

fn part_a(input: &str) -> Answer {
    let mut out = 0;

    for i in input.trim().lines() {
        let mut both = i[0..i.len() / 2].chars().collect::<Vec<_>>();
        let pocket_2 = i[i.len() / 2..].chars().collect::<Vec<_>>();
        both.retain(|x| pocket_2.contains(x));
        both.dedup();

        debug_assert!(both.len() == 1);
        out += score_item(both[0]) as usize;
    }

    out.into()
}

fn part_b(input: &str) -> Answer {
    let mut out = 0;

    for i in input.trim().lines().collect::<Vec<_>>().chunks(3) {
        let mut all = HashSet::new();
        i.iter().for_each(|x| all.extend(x.chars()));
        i.iter().for_each(|x| all.retain(|y| x.contains(*y)));

        debug_assert!(all.len() == 1);
        out += score_item(*all.iter().next().unwrap()) as usize;
    }

    out.into()
}

fn score_item(char_: char) -> u8 {
    match char_ as u8 {
        97..=122 => char_ as u8 - 96,
        65..=90 => char_ as u8 - 38,
        _ => unreachable!(),
    }
}
