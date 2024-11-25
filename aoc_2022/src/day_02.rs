use common::{Answer, ISolution};

pub struct Day02;

impl ISolution for Day02 {
    fn name(&self) -> &'static str {
        "Rock Paper Scissors"
    }

    fn part_a(&self, input: &str) -> Answer {
        let mut score = 0;

        for (other, self_) in input
            .lines()
            .filter(|x| !x.is_empty())
            .map(|x| x.split_once(' ').unwrap())
        {
            let other_move = Move::from_str(other);
            let self_move = Move::from_str(self_);

            score += self_move as u32 + 1;
            score += score_round(other_move, self_move).to_score();
        }

        score.into()
    }

    fn part_b(&self, input: &str) -> Answer {
        let mut score = 0;

        for (other, self_) in input
            .lines()
            .filter(|x| !x.is_empty())
            .map(|x| x.split_once(' ').unwrap())
        {
            let other_move = Move::from_str(other);
            let self_move = match self_ {
                "X" => other_move.derive(false),
                "Y" => other_move,
                "Z" => other_move.derive(true),
                _ => unreachable!(),
            };

            score += self_move as u32 + 1;
            score += score_round(other_move, self_move).to_score();
        }

        score.into()
    }
}

#[derive(Debug, Clone, Copy, PartialEq)]
enum Move {
    Rock,
    Paper,
    Scissors,
}

#[derive(Debug)]
enum Outcome {
    Win,
    Lose,
    Tie,
}

impl Move {
    fn from_str(s: &str) -> Self {
        match s {
            "A" | "X" => Move::Rock,
            "B" | "Y" => Move::Paper,
            "C" | "Z" => Move::Scissors,
            _ => unreachable!(),
        }
    }

    fn from_index(i: usize) -> Self {
        match i {
            0 => Move::Rock,
            1 => Move::Paper,
            2 => Move::Scissors,
            _ => unreachable!(),
        }
    }

    fn derive(&self, win: bool) -> Self {
        Move::from_index((*self as usize + if win { 1 } else { 2 }) % 3)
    }
}

impl Outcome {
    fn to_score(&self) -> u32 {
        match self {
            Outcome::Lose => 0,
            Outcome::Tie => 3,
            Outcome::Win => 6,
        }
    }
}

fn score_round(other: Move, self_: Move) -> Outcome {
    if other == self_ {
        return Outcome::Tie;
    }

    if (other as u32 + 1) % 3 == self_ as u32 {
        return Outcome::Win;
    }

    Outcome::Lose
}
