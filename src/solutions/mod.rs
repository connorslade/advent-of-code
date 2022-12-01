use crate::common::Solution;

mod year_2021;

pub fn get_year(year: u32) -> &'static [&'static dyn Solution] {
    match year {
        2021 => &year_2021::ALL,
        _ => &[],
    }
}
