use common::{solution, Answer};

solution!("Scratchcards", 4);

fn part_a(input: &str) -> Answer {
    let cards = parse(input);
    cards
        .iter()
        .filter(|x| x.wins > 0)
        .map(|x| 2u32.pow(x.wins.saturating_sub(1) as u32))
        .sum::<u32>()
        .into()
}

fn part_b(input: &str) -> Answer {
    let cards = parse(input);

    let mut queue = (0..cards.len()).collect::<Vec<_>>();
    let mut visited = 0;

    while let Some(i) = queue.pop() {
        visited += 1;

        let card = &cards[i];
        if card.wins == 0 {
            continue;
        }

        for j in 0..card.wins as usize {
            queue.push(j + i + 1);
        }
    }

    visited.into()
}

struct Card {
    wins: u8,
}

fn parse(input: &str) -> Vec<Card> {
    let mut cards = Vec::new();
    for line in input.lines() {
        let (_, line) = line.split_once(": ").unwrap();
        let (winning, scratch) = line.split_once(" | ").unwrap();
        let parse = |s: &str| {
            s.split_whitespace()
                .map(|x| x.parse().unwrap())
                .collect::<Vec<u8>>()
        };

        let winning = parse(winning);
        let scratch = parse(scratch);
        cards.push(Card {
            wins: scratch.iter().filter(|x| winning.contains(x)).count() as u8,
        });
    }

    cards
}

#[cfg(test)]
mod test {
    use indoc::indoc;

    const CASE: &str = indoc! {"
        Card 1: 41 48 83 86 17 | 83 86  6 31 17  9 48 53
        Card 2: 13 32 20 16 61 | 61 30 68 82 17 32 24 19
        Card 3:  1 21 53 59 44 | 69 82 63 72 16 21 14  1
        Card 4: 41 92 73 84 69 | 59 84 76 51 58  5 54 83
        Card 5: 87 83 26 28 32 | 88 30 70 12 93 22 82 36
        Card 6: 31 18 13 56 72 | 74 77 10 23 35 67 36 11
    "};

    #[test]
    fn part_a() {
        assert_eq!(super::part_a(CASE), 13.into());
    }

    #[test]
    fn part_b() {
        assert_eq!(super::part_b(CASE), 30.into());
    }
}
