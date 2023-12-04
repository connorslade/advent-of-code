use common::{Answer, Solution};

pub struct Day04;

impl Solution for Day04 {
    fn name(&self) -> &'static str {
        "Scratchcards"
    }

    fn part_a(&self, input: &str) -> Answer {
        let cards = parse(input);
        cards
            .iter()
            .map(|x| x.count_wins())
            .filter(|x| *x > 0)
            .map(|x| 2u32.pow(x.saturating_sub(1) as u32))
            .sum::<u32>()
            .into()
    }

    fn part_b(&self, input: &str) -> Answer {
        let cards = parse(input);

        let mut queue = cards.clone();
        let mut visited = 0;

        while let Some(card) = queue.pop() {
            let wins = card.count_wins();
            visited += 1;
            if wins == 0 {
                continue;
            }

            let num = card.number as usize;
            for i in num + 1..=num + wins {
                queue.push(cards[i as usize - 1].clone());
            }
        }

        visited.into()
    }
}

#[derive(Clone)]
struct Card {
    number: u8,
    winning: Vec<u8>,
    scratch: Vec<u8>,
}

fn parse(input: &str) -> Vec<Card> {
    let mut cards = Vec::new();
    for line in input.lines() {
        let (_, line) = line.split_once(": ").unwrap();
        let (winning, scratch) = line.split_once(" | ").unwrap();
        let parse = |s: &str| s.split_whitespace().map(|x| x.parse().unwrap()).collect();
        cards.push(Card {
            number: cards.len() as u8 + 1,
            winning: parse(winning),
            scratch: parse(scratch),
        });
    }

    cards
}

impl Card {
    fn count_wins(&self) -> usize {
        self.scratch
            .iter()
            .filter(|x| self.winning.contains(x))
            .count()
    }
}

#[cfg(test)]
mod test {
    use common::Solution;
    use indoc::indoc;

    use super::Day04;

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
        assert_eq!(Day04.part_a(CASE), 13.into());
    }

    #[test]
    fn part_b() {
        assert_eq!(Day04.part_b(CASE), 30.into());
    }
}
