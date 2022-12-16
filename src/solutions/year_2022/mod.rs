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
mod day_11;
mod day_12;
mod day_13;
mod day_14;
mod day_15;
mod day_16;

pub const ALL: [&dyn Solution; 16] = [
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
    &day_11::Day11,
    &day_12::Day12,
    &day_13::Day13,
    &day_14::Day14,
    &day_15::Day15,
    &day_16::Day16,
];
