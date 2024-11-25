use aoc_lib::{direction::Direction, matrix::Matrix};
use common::{solution, Answer};
use hashbrown::HashMap;
use nd_vec::vector;

solution!("Chiton", (2022, 00));

fn part_a(input: &str) -> Answer {
    let matrix = Matrix::new_chars(input, |chr| chr.to_digit(10).unwrap() as u8);

    let mut out = usize::MAX;
    let mut visited = HashMap::new();
    let mut queue = Vec::new();
    queue.push((vector!(0, 0), 0));

    while let Some((pos, cost)) = queue.pop() {
        if pos == matrix.size - vector!(1, 1) {
            out = out.min(cost);
            continue;
        }

        visited.insert(pos, cost);
        for dir in Direction::ALL {
            if let Some((next, new_cost)) = dir
                .try_advance(pos)
                .and_then(|x| Some((x, cost + *matrix.get(x)? as usize)))
            {
                if let Some(prev) = visited.get(&next) {
                    if *prev <= new_cost {
                        continue;
                    }
                }

                queue.push((next, new_cost));
            }
        }
    }

    out.into()
}

fn part_b(_input: &str) -> Answer {
    Answer::Unimplemented
}

#[cfg(test)]
mod test {
    use common::solution;
    use indoc::indoc;

    const CASE: &str = indoc! {"
        1163751742
        1381373672
        2136511328
        3694931569
        7463417111
        1319128137
        1359912421
        3125421639
        1293138521
        2311944581
    "};

    #[test]
    fn part_a() {
        assert_eq!(super::part_a(CASE), 40.into());
    }

    #[test]
    fn part_b() {
        assert_eq!(super::part_b(CASE), ().into());
    }
}
