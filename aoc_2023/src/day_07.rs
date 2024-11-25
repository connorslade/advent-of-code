use std::{cmp::Ordering, fmt::Debug};

use common::{Answer, ISolution};
use itertools::Itertools;

pub struct Day07;

impl ISolution for Day07 {
    fn name(&self) -> &'static str {
        "Camel Cards"
    }

    fn part_a(&self, input: &str) -> Answer {
        let hands = parse(input, "AKQJT98765432");
        solve(hands, Hand::score_a).into()
    }

    fn part_b(&self, input: &str) -> Answer {
        let hands = parse(input, "AKQT98765432J");
        solve(hands, Hand::score_b).into()
    }
}

fn solve(mut hands: Vec<Hand>, score: fn(&Hand) -> HandType) -> usize {
    hands.sort_by(|a, b| score(a).cmp(&score(b)).then_with(|| b.score_first(a)));

    hands
        .iter()
        .rev()
        .enumerate()
        .map(|(i, e)| e.bid as usize * (i + 1))
        .sum::<usize>()
}

struct Hand {
    cards: Vec<u8>,
    bid: u16,
}

fn parse(input: &str, mappings: &'static str) -> Vec<Hand> {
    let mut hands = Vec::new();

    for line in input.lines() {
        let (cards, bid) = line.split_at(5);

        let cards = cards
            .as_bytes()
            .iter()
            .map(|&c| 13 - mappings.find(c as char).unwrap() as u8)
            .collect();
        let bid = bid.trim().parse().unwrap();

        hands.push(Hand { cards, bid });
    }

    hands
}

#[derive(Debug, Clone, Copy, PartialEq, PartialOrd, Eq, Ord)]
enum HandType {
    FiveOfAKind,
    FourOfAKind,
    FullHouse,
    ThreeOfAKind,
    TwoPair,
    OnePair,
    HighCard,
}

impl Hand {
    fn score_a(&self) -> HandType {
        let mut counts = [0; 13];
        for &c in &self.cards {
            counts[13 - c as usize] += 1;
        }

        if counts.iter().any(|&c| c == 5) {
            HandType::FiveOfAKind
        } else if counts.iter().any(|&c| c == 4) {
            HandType::FourOfAKind
        } else if counts.iter().any(|&c| c == 3) && counts.iter().any(|&c| c == 2) {
            HandType::FullHouse
        } else if counts.iter().any(|&c| c == 3) {
            HandType::ThreeOfAKind
        } else if counts.iter().filter(|&&c| c == 2).count() == 2 {
            HandType::TwoPair
        } else if counts.iter().any(|&c| c == 2) {
            HandType::OnePair
        } else {
            HandType::HighCard
        }
    }

    fn score_b(&self) -> HandType {
        let mut counts = [0; 13];
        for &c in &self.cards {
            counts[13 - c as usize] += 1;
        }

        let jacks = counts[12];
        let counts = counts[0..12]
            .iter()
            .copied()
            .filter(|x| *x != 0)
            .sorted()
            .rev()
            .collect::<Vec<_>>();

        if counts.len() <= 1 || counts[0] + jacks == 5 {
            HandType::FiveOfAKind
        } else if counts[0] + jacks == 4 {
            HandType::FourOfAKind
        } else if ((counts[0] + jacks == 3) && (counts[1] == 2))
            || ((counts[0] == 3) && (counts[1] + jacks == 2))
        {
            HandType::FullHouse
        } else if counts[0] + jacks == 3 {
            HandType::ThreeOfAKind
        } else if (counts[0] + jacks == 2 && counts[1] == 2)
            || (counts[0] == 2 && counts[1] + jacks == 2)
        {
            HandType::TwoPair
        } else if counts[0] + jacks == 2 {
            HandType::OnePair
        } else {
            HandType::HighCard
        }
    }

    fn score_first(&self, other: &Hand) -> Ordering {
        for (&a, &b) in self.cards.iter().zip(other.cards.iter()) {
            if a != b {
                return a.cmp(&b);
            }
        }

        Ordering::Equal
    }
}

#[cfg(test)]
mod test {
    use common::ISolution;
    use indoc::indoc;

    use super::Day07;

    const CASE: &str = indoc! {"
        32T3K 765
        T55J5 684
        KK677 28
        KTJJT 220
        QQQJA 483
    "};

    #[test]
    fn part_a() {
        assert_eq!(Day07.part_a(CASE), 6440.into());
    }

    #[test]
    fn part_b() {
        assert_eq!(Day07.part_b(CASE), 5905.into());
    }
}
