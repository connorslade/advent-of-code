use common::Solution;

#[macro_use]
mod aoc_lib;
mod day_01;
mod day_02;
mod day_03;
// [import_marker]

#[rustfmt::skip]
pub const ALL: &[&dyn Solution] = &[
    &day_01::Day01,
    &day_02::Day02,
    &day_03::Day03,
    // [list_marker]
];
