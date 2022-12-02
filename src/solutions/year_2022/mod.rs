use crate::Solution;

mod day_01;
mod day_02;

pub const ALL: [&dyn Solution; 2] = [&day_01::Day01, &day_02::Day02];
