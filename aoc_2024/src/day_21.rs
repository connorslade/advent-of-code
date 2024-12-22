use std::collections::HashMap;
use std::iter::once;
use std::iter::repeat;

use common::{solution, Answer};
use nd_vec::{vector, Vec2};

solution!("Keypad Conundrum", 21);

const NUMPAD: [Vec2<usize>; 11] = [
    vector!(1, 3),
    vector!(0, 2),
    vector!(1, 2),
    vector!(2, 2),
    vector!(0, 1),
    vector!(1, 1),
    vector!(2, 1),
    vector!(0, 0),
    vector!(1, 0),
    vector!(2, 0),
    vector!(2, 3),
];

const DIRPAD: [Vec2<usize>; 5] = [
    vector!(1, 0),
    vector!(1, 1),
    vector!(0, 1),
    vector!(2, 1),
    vector!(2, 0),
];

fn part_a(input: &str) -> Answer {
    solve(input, 2).into()
}

fn part_b(input: &str) -> Answer {
    solve(input, 25).into()
}

fn solve(input: &str, dirpad_count: u8) -> u64 {
    let mut sum = 0;
    for case in input.lines() {
        let buttons = case
            .chars()
            .map(|x| x.to_digit(10).unwrap_or(10) as u8)
            .collect::<Vec<_>>();

        let mut out = 0;
        let mut pos = 10;
        for &next in buttons.iter() {
            let paths = get_paths(pos as usize, next as usize, &NUMPAD);
            out += paths
                .into_iter()
                .map(|p| solve_dirpad(&mut HashMap::new(), p, dirpad_count))
                .min()
                .unwrap_or(0);
            pos = next;
        }

        let number = case[..case.len() - 1].parse().unwrap_or(1);
        sum += out * number;
    }

    sum
}

fn solve_dirpad(
    memo: &mut HashMap<(Vec<Button>, u8), u64>,
    sequence: Vec<Button>,
    depth: u8,
) -> u64 {
    if depth == 0 {
        return sequence.len() as u64;
    }

    if let Some(&cache) = memo.get(&(sequence.clone(), depth)) {
        return cache;
    }

    let mut out = 0;
    let mut pos = Button::Activate;
    for &next in sequence.iter() {
        let paths = get_paths(pos as usize, next as usize, &DIRPAD);
        out += paths
            .into_iter()
            .map(|p| solve_dirpad(memo, p, depth - 1))
            .min()
            .unwrap_or(0);
        pos = next;
    }

    memo.insert((sequence, depth), out);
    out
}

fn get_paths(start: usize, end: usize, keys: &[Vec2<usize>]) -> Vec<Vec<Button>> {
    let (start, end) = (keys[start], keys[end]);
    if start == end {
        return vec![vec![Button::Activate]];
    }

    let dy = start.y().abs_diff(end.y());
    let dx = start.x().abs_diff(end.x());

    let py = repeat([Button::Down, Button::Up][(end.y() < start.y()) as usize]).take(dy);
    let px = repeat([Button::Right, Button::Left][(end.x() < start.x()) as usize]).take(dx);

    fn out(iter: impl Iterator<Item = Button>) -> Vec<Button> {
        iter.chain(once(Button::Activate)).collect()
    }

    if dy == 0 {
        vec![out(px)]
    } else if dx == 0 {
        vec![out(py)]
    } else if !keys.contains(&vector!(start.x(), end.y())) {
        vec![out(px.chain(py))]
    } else if !keys.contains(&vector!(end.x(), start.y())) {
        vec![out(py.chain(px))]
    } else {
        vec![out(px.clone().chain(py.clone())), out(py.chain(px))]
    }
}

#[derive(Copy, Clone, PartialEq, Eq, Hash)]
#[repr(u8)]
enum Button {
    Up = 0,
    Down = 1,
    Left = 2,
    Right = 3,
    Activate = 4,
}

#[cfg(test)]
mod test {
    use indoc::indoc;

    const CASE: &str = indoc! {"
        029A
        980A
        179A
        456A
        379A
    "};

    #[test]
    fn part_a() {
        assert_eq!(super::part_a(CASE), 126384.into());
    }

    #[test]
    fn part_b() {
        assert_eq!(super::part_b(CASE), 154115708116294_u64.into());
    }
}
