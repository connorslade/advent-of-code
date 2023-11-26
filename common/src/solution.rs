use crate::Answer;

pub trait Solution {
    fn name(&self) -> &'static str;
    fn part_a(&self, input: &str) -> Answer;
    fn part_b(&self, input: &str) -> Answer;

    fn is_dummy(&self) -> bool {
        false
    }
}

pub struct DummySolution;

impl Solution for DummySolution {
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
