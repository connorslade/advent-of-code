use crate::{problem, Solution};

pub struct Day02;

impl Solution for Day02 {
    fn name(&self) -> &'static str {
        "Dive!"
    }

    fn part_a(&self) -> String {
        let d = problem::load(2021, 2);
        let mut dep: u32 = 0;
        let mut hor: u32 = 0;

        for i in d.lines() {
            let seg = i.split(' ').collect::<Vec<&str>>();
            let x = seg[1].parse::<u32>().unwrap();

            match seg[0] {
                "forward" => hor += x,
                "up" => dep -= x,
                "down" => dep += x,
                _ => {}
            }
        }

        (dep * hor).to_string()
    }

    fn part_b(&self) -> String {
        let d = problem::load(2021, 2);
        let mut dep: u32 = 0;
        let mut hor: u32 = 0;
        let mut aim: u32 = 0;

        for i in d.lines() {
            let seg = i.split(' ').collect::<Vec<&str>>();
            let x = seg[1].parse::<u32>().unwrap();

            match seg[0] {
                "forward" => {
                    hor += x;
                    dep += aim * x;
                }
                "up" => aim -= x,
                "down" => aim += x,
                _ => {}
            }
        }

        (dep * hor).to_string()
    }
}
