use crate::common::{self, Solution};

pub fn main() -> Solution {
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

        println!("[*] OUT: {}", inc);
    })
}
