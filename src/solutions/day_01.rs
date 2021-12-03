use crate::common::{self, Solution};

pub fn part_a() -> Solution {
    Solution::new("01-Sonar Sweep-A", || {
        let data = common::load("01")
            .lines()
            .map(|x| x.parse::<u32>().unwrap())
            .collect::<Vec<u32>>();
        let mut inc = 0;

        for i in 1..data.len() {
            if data[i - 1] < data[i] {
                inc += 1;
            }
        }

        inc.to_string()
    })
}

pub fn part_b() -> Solution {
    Solution::new("01-Sonar Sweep-B", || {
        let d = common::load("01")
            .lines()
            .map(|x| x.parse::<u32>().unwrap())
            .collect::<Vec<u32>>();
        let mut inc = 0;

        for i in 3..d.len() {
            let a = d[i - 1] + d[i - 2] + d[i - 3];
            let b = d[i] + d[i - 1] + d[i - 2];

            if b > a {
                inc += 1;
            }
        }

        inc.to_string()
    })
}
