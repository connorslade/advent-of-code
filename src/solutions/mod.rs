use crate::common::Solution;

pub mod day_01;
pub mod day_02;
pub mod day_03;
pub mod day_04;

pub const ALL: [&dyn Solution; 4] = [
    &day_01::Day01 {},
    &day_02::Day02 {},
    &day_03::Day03 {},
    &day_04::Day04 {},
];
