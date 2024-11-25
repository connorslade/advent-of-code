use hashbrown::HashSet;

use common::{solution, Answer};

solution!("Tuning Trouble", 6);

fn part_a(input: &str) -> Answer {
    process(input, 4).into()
}

fn part_b(input: &str) -> Answer {
    process(input, 14).into()
}

fn process(input: &str, size: usize) -> usize {
    let mut chars = HashSet::new();
    'o: for i in input.chars().enumerate().collect::<Vec<_>>().windows(size) {
        for j in i {
            if !chars.insert(j.1) {
                chars.clear();
                continue 'o;
            }
        }

        return i[size - 1].0 + 1;
    }

    unreachable!()
}
