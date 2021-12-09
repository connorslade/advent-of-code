use crate::common::Solution;

pub mod day_01;
pub mod day_02;
pub mod day_03;
pub mod day_04;
pub mod day_05;
pub mod day_06;
pub mod day_07;
pub mod day_08;
pub mod day_09;

pub const ALL: [&dyn Solution; 9] = [
    &day_01::Day01 {},
    &day_02::Day02 {},
    &day_03::Day03 {},
    &day_04::Day04 {},
    &day_05::Day05 {},
    &day_06::Day06 {},
    &day_07::Day07 {},
    &day_08::Day08 {},
    &day_09::Day09 {},
];
