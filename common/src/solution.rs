use crate::Answer;

pub trait ISolution {
    fn name(&self) -> &'static str;
    fn part_a(&self, input: &str) -> Answer;
    fn part_b(&self, input: &str) -> Answer;

    fn is_dummy(&self) -> bool {
        false
    }
}

pub struct DummySolution;

impl ISolution for DummySolution {
    fn name(&self) -> &'static str {
        unreachable!()
    }

    fn part_a(&self, _input: &str) -> Answer {
        unreachable!()
    }

    fn part_b(&self, _input: &str) -> Answer {
        unreachable!()
    }

    fn is_dummy(&self) -> bool {
        true
    }
}

pub struct Solution {
    pub name: &'static str,
    pub date: (u16, u8),

    pub part_a: fn(&str) -> Answer,
    pub part_b: fn(&str) -> Answer,
}
