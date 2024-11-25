use crate::Answer;

pub struct Solution {
    pub name: &'static str,
    pub day: u8,

    pub part_a: fn(&str) -> Answer,
    pub part_b: fn(&str) -> Answer,
}

#[macro_export]
macro_rules! solution {
    ($name:expr, $day:expr) => {
        pub const SOLUTION: $crate::Solution = $crate::Solution {
            name: $name,
            day: $day,

            part_a,
            part_b,
        };
    };
}
