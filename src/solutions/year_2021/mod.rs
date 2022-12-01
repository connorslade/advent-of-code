use crate::Solution;

mod day_01;
mod day_02;
mod day_03;
mod day_04;
mod day_05;
mod day_06;
mod day_07;
mod day_08;
mod day_09;
mod day_10;

pub const ALL: [&dyn Solution; 10] = [
    &day_01::Day01,
    &day_02::Day02,
    &day_03::Day03,
    &day_04::Day04,
    &day_05::Day05,
    &day_06::Day06,
    &day_07::Day07,
    &day_08::Day08,
    &day_09::Day09,
    &day_10::Day10,
];
