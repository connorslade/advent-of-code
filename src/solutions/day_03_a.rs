use crate::common::{self, Solution};

pub fn main() -> Solution {
    Solution::new("Day 3-Binary Diagnostic-A", || {
        let data = common::load("03");
        let num_len = data.lines().next().unwrap().len();

        let mut gamma = vec![0; num_len];
        let mut epsilon = vec![0; num_len];

        for i in 0..num_len {
            let mut z = 0;
            let mut o = 0;
            for j in data.lines() {
                match j.chars().nth(i).unwrap() {
                    '0' => z += 1,
                    '1' => o += 1,
                    _ => {}
                }
            }

            gamma[i] = 0;
            epsilon[i] = 1;
            if o > z {
                epsilon[i] = 0;
                gamma[i] = 1;
            }
        }

        let gamma = int_from_bin(&gamma).unwrap();
        let epsilon = int_from_bin(&epsilon).unwrap();

        (epsilon * gamma).to_string()
    })
}

fn int_from_bin(inp: &[u32]) -> Option<usize> {
    usize::from_str_radix(
        &inp.iter()
            .map(|x| x.to_string())
            .collect::<Vec<String>>()
            .join(""),
        2,
    )
    .ok()
}
