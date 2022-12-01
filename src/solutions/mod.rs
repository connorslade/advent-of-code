use crate::Solution;

mod year_2021;
mod year_2022;

pub fn get_year(year: u32) -> &'static [&'static dyn Solution] {
    match year {
        2021 => &year_2021::ALL,
        2022 => &year_2022::ALL,
        _ => &[],
    }
}
