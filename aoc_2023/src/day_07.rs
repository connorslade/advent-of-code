use std::{cmp::Ordering, fmt::Debug};

use common::{Answer, Solution};

const CARDS_A: &str = "AKQJT98765432";
const CARDS_B: &str = "AKQT98765432J";

pub struct Day07;

impl Solution for Day07 {
    fn name(&self) -> &'static str {
        "Camel Cards"
    }

    fn part_a(&self, input: &str) -> Answer {
        let mut hands = parse(input, CARDS_A);

        hands.sort_by(|a, b| {
            let a_score = score(&a.cards) as u8;
            let b_score = score(&b.cards) as u8;

            if a_score == b_score {
                score_first(&b.cards, &a.cards)
            } else {
                a_score.cmp(&b_score)
            }
        });

        hands
            .iter()
            .rev()
            .enumerate()
            .map(|(i, e)| e.bid as usize * (i + 1))
            .sum::<usize>()
            .into()
    }

    fn part_b(&self, input: &str) -> Answer {
        let mut hands = parse(input, CARDS_B);

        hands.sort_by(|a, b| {
            let a_score = score_b(&a.cards) as u8;
            let b_score = score_b(&b.cards) as u8;

            if a_score == b_score {
                score_first(&b.cards, &a.cards)
            } else {
                a_score.cmp(&b_score)
            }
        });

        // KTJJT
        // QQQJA
        // T55J5
        // KK677
        // 32T3K

        //    249058757
        // => 248652697

        hands
            .iter()
            .rev()
            .enumerate()
            .map(|(i, e)| e.bid as usize * (i + 1))
            .sum::<usize>()
            .into()
    }
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

#[repr(u8)]
#[derive(Debug, Clone, Copy)]
enum HandType {
    FiveOfAKind,
    FourOfAKind,
    FullHouse,
    ThreeOfAKind,
    TwoPair,
    OnePair,
    HighCard,
}

fn score(cards: &[u8]) -> HandType {
    let mut counts = [0; 13];
    for &c in cards {
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

fn score_b(cards: &[u8]) -> HandType {
    let mut counts = [0; 13];
    for &c in cards {
        counts[13 - c as usize] += 1;
    }

    let jacks = counts[12];
    let counts = &counts[0..12];

    if counts.iter().any(|&c| c + jacks == 5) {
        HandType::FiveOfAKind
    } else if counts.iter().any(|&c| c + jacks == 4) {
        HandType::FourOfAKind
    } else if {
        counts.iter().any(|&c| c + jacks == 3)
            && counts
                .iter()
                .enumerate()
                .any(|(i, &c)| c == 2 && i != counts.iter().position(|x| x + jacks == 3).unwrap())
    } {
        HandType::FullHouse
    } else if counts.iter().any(|&c| c + jacks == 3) {
        HandType::ThreeOfAKind
    } else if counts.iter().any(|&c| c == 2) && counts.iter().any(|&c| c + jacks == 2) {
        HandType::TwoPair
    } else if counts.iter().any(|&c| c + jacks == 2) {
        HandType::OnePair
    } else {
        HandType::HighCard
    }
}

// fn score_b(cards: &[u8]) -> HandType {
//     let mut best_type = HandType::HighCard;

//     let mut jokers = Vec::new();
//     for (i, c) in cards.iter().enumerate() {
//         if *c == 1 {
//             jokers.push(i);
//         }
//     }

//     if jokers.is_empty() {
//         return score(cards);
//     }

//     // Take tha max of score(cards) with every permutation of joker replacements
//     let mut joker_values = vec![1; jokers.len()];
//     'outer: while joker_values[0] != 12 {
//         let idx = joker_values.len() - 1;
//         joker_values[idx] += 1;
//         for i in (0..joker_values.len()).rev() {
//             if joker_values[i] == 12 {
//                 joker_values[i] = 1;

//                 if i == 0 {
//                     break 'outer;
//                 }

//                 joker_values[i - 1] += 1;
//             }
//         }

//         let mut cards = cards.to_vec();
//         for (i, &v) in joker_values.iter().enumerate() {
//             cards[jokers[i]] = v;
//         }

//         let score = score(&cards);
//         if score as u8 > best_type as u8 {
//             best_type = score;
//         }
//     }

//     best_type
// }

fn score_first(a: &[u8], b: &[u8]) -> Ordering {
    for (&a, &b) in a.iter().zip(b.iter()) {
        if a != b {
            return a.cmp(&b);
        }
    }

    Ordering::Equal
}

impl Debug for Hand {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.debug_struct("Hand")
            .field(
                "cards",
                &self
                    .cards
                    .iter()
                    .map(|x| CARDS_B.as_bytes()[13 - *x as usize] as char)
                    .collect::<String>(),
            )
            .field("bid", &self.bid)
            .finish()
    }
}

#[cfg(test)]
mod test {
    use common::Solution;
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
