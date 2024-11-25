use crate::Answer;

pub trait ISolution {
    fn name(&self) -> &'static str;
    fn part_a(input: &str) -> Answer;
    fn part_b(input: &str) -> Answer;

    fn is_dummy(&self) -> bool {
        false
    }
}

pub struct DummySolution;

impl ISolution for DummySolution {
    fn name(&self) -> &'static str {
        unreachable!()
    }

    fn part_a(_input: &str) -> Answer {
        unreachable!()
    }

    fn part_b(_input: &str) -> Answer {
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

#[macro_export]
macro_rules! solution {
    ($name:expr, $date:expr) => {
        pub const SOLUTION: $crate::Solution = $crate::Solution {
            name: $name,
            date: $date,

            part_a,
            part_b,
        };
    };
}
