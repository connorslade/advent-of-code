use common::Solution;

mod aoc_lib;
mod day_01;
mod day_02;
// [import_marker]

#[rustfmt::skip]
pub const ALL: &[&dyn Solution] = &[
    &day_01::Day01,
    &day_02::Day02,
    // [list_marker]
];
