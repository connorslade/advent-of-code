use common::{Answer, solution};
use nd_vec::{Vec2, vector};

solution!("Christmas Tree Farm", 12);

#[derive(Debug)]
struct Tree {
    size: Vec2<u32>,
    counts: Vec<u32>,
}

fn part_a(input: &str) -> Answer {
    let (presents, trees) = parse(input);

    (trees.iter())
        .filter(|tree| {
            (presents.iter().zip(tree.counts.iter()))
                .map(|(present, count)| *present as u32 * *count)
                .sum::<u32>()
                <= tree.size.x() * tree.size.y()
        })
        .count()
        .into()
}

fn part_b(_input: &str) -> Answer {
    Answer::Unimplemented
}

fn parse(input: &str) -> (Vec<usize>, Vec<Tree>) {
    let (presents_raw, trees_raw) = input.rsplit_once("\n\n").unwrap();

    let mut presents = Vec::new();
    for present in presents_raw.split("\n\n") {
        let raw = present[present.find('\n').unwrap()..].trim();
        presents.push(raw.chars().filter(|x| *x == '#').count());
    }

    let mut trees = Vec::new();
    for tree in trees_raw.lines() {
        let (size, counts) = tree.split_once(": ").unwrap();

        let (width, height) = size.split_once('x').unwrap();
        let size = vector!(width.parse().unwrap(), height.parse().unwrap());

        let counts = counts
            .split_whitespace()
            .map(|x| x.parse::<u32>().unwrap())
            .collect::<Vec<_>>();

        trees.push(Tree { size, counts });
    }

    (presents, trees)
}

#[cfg(test)]
mod test {
    use indoc::indoc;

    const CASE: &str = indoc! {"
        0:
        ###
        ##.
        ##.

        1:
        ###
        ##.
        .##

        2:
        .##
        ###
        ##.

        3:
        ##.
        ###
        ##.

        4:
        ###
        #..
        ###

        5:
        ###
        .#.
        ###

        4x4: 0 0 0 0 2 0
        12x5: 1 0 1 0 2 2
        12x5: 1 0 1 0 5 2
    "};

    #[test]
    fn part_a() {
        assert_eq!(super::part_a(CASE), 2.into());
    }

    #[test]
    fn part_b() {
        assert_eq!(super::part_b(CASE), ().into());
    }
}
