use crate::Solution;

mod day_01;
mod day_02;
mod day_03;

pub const ALL: [&dyn Solution; 3] = [&day_01::Day01, &day_02::Day02, &day_03::Day03];
