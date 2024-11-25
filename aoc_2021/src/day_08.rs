use common::{solution, Answer};

use hashbrown::HashMap;

const CHARS: [char; 7] = ['a', 'b', 'c', 'd', 'e', 'f', 'g'];

const DIGITS: [&str; 10] = [
    "abcefg", "cf", "acdeg", "acdfg", "bcdf", "abdfg", "abdefg", "acf", "abcdefg", "abcdfg",
];

solution!("Seven Segment Search", (2022, 00));

fn part_a(input: &str) -> Answer {
    let data = parse(input);
    let mut inc = 0;

    for i in data {
        inc +=
            i.1.iter()
                .filter(|x| [2, 3, 4, 7].contains(&x.len()))
                .count();
    }

    inc.into()
}

fn part_b(input: &str) -> Answer {
    let data = parse(input);
    let mut inc = 0;

    let perms = permutations(CHARS.to_vec());
    let mut sort_digits = DIGITS.to_vec();
    sort_digits.sort_unstable();

    for i in data {
        for p in &perms {
            let mut wires = HashMap::new();

            for j in CHARS {
                let pos = CHARS.iter().position(|x| *x == j).unwrap();
                *wires.entry(j).or_insert(p[pos]) = p[pos];
            }

            let mut new_clues = Vec::new();
            for clue in &i.0 {
                let mut x = String::new();
                for char in clue.chars() {
                    x.push(*wires.get(&char).unwrap());
                }
                let mut to_sort = x.chars().collect::<Vec<char>>();
                to_sort.sort_unstable();
                new_clues.push(
                    to_sort
                        .iter()
                        .map(|x| x.to_string())
                        .collect::<Vec<String>>()
                        .join(""),
                );
            }
            new_clues.sort();

            if new_clues == sort_digits {
                let mut n = Vec::new();
                for d in &i.1 {
                    let mut x = String::new();
                    for char in d.chars() {
                        x.push(*wires.get(&char).unwrap());
                    }
                    let mut to_sort = x.chars().collect::<Vec<char>>();
                    to_sort.sort_unstable();
                    x = to_sort
                        .iter()
                        .map(|x| x.to_string())
                        .collect::<Vec<String>>()
                        .join("");

                    n.push(DIGITS.iter().position(|i| **i == x).unwrap());
                }

                inc += n
                    .iter()
                    .map(|x| x.to_string())
                    .collect::<Vec<String>>()
                    .join("")
                    .parse::<u32>()
                    .unwrap();
                break;
            }
        }
    }

    inc.into()
}

fn parse(inp: &str) -> Vec<(Vec<String>, Vec<String>)> {
    let mut out = Vec::new();

    for i in inp.lines() {
        let mut parts = i.split('|');

        let test = parts
            .next()
            .unwrap()
            .split(' ')
            .filter(|x| !x.is_empty())
            .map(|x| x.to_owned())
            .collect::<Vec<String>>();

        let check = parts
            .next()
            .unwrap()
            .split(' ')
            .filter(|x| !x.is_empty())
            .map(|x| x.to_owned())
            .collect::<Vec<String>>();

        out.push((test, check));
    }

    out
}

// Modified from https://stackoverflow.com/a/59939809/12471934
fn permutations<T: Clone>(items: Vec<T>) -> Vec<Vec<T>>
where
    T: Ord,
{
    if items.len() == 1 {
        return vec![items];
    }

    let mut output: Vec<Vec<T>> = Vec::new();
    let mut unique_items = items.clone();

    unique_items.sort();
    unique_items.dedup();

    for first in unique_items {
        let mut remaining_elements = items.clone();

        let index = remaining_elements.iter().position(|x| *x == first).unwrap();
        remaining_elements.remove(index);

        for mut permutation in permutations(remaining_elements) {
            permutation.insert(0, first.clone());
            output.push(permutation);
        }
    }
    output
}
