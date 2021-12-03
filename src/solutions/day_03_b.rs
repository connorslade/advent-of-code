use crate::common::{self, Solution};

pub fn main() -> Solution {
    Solution::new("Day 3-Binary Diagnostic-B", || {
        let data = common::load("03");
        let num_len = data.lines().next().unwrap().len();

        let mut oxygen_gen = 0;
        let mut co2_scrub = 0;

        let mut oxygen_keep = data.lines().collect::<Vec<&str>>();
        let mut oxygen_raw = vec![[0, 0]; num_len];

        let mut co2_keep = oxygen_keep.clone();
        let mut co2_raw = oxygen_raw.clone();

        for i in 0..num_len {
            let imax = get_imax(&oxygen_raw, i);
            oxygen_raw = gen_raw(oxygen_raw, num_len, &oxygen_keep);
            oxygen_keep = oxygen_keep
                .iter()
                .filter(|x| x.chars().into_iter().nth(i).unwrap() == imax)
                .map(|x| *x)
                .collect::<Vec<&str>>();

            let imax = get_imax(&co2_raw, i);
            co2_raw = gen_raw(co2_raw, num_len, &co2_keep);
            co2_keep = co2_keep
                .iter()
                .filter(|x| x.chars().into_iter().nth(i).unwrap() != imax)
                .map(|x| *x)
                .collect::<Vec<&str>>();

            if oxygen_keep.len() == 1 {
                oxygen_gen = isize::from_str_radix(oxygen_keep.first().unwrap(), 2).unwrap();
            }

            if co2_keep.len() == 1 {
                co2_scrub = isize::from_str_radix(co2_keep.first().unwrap(), 2).unwrap();
            }
        }

        let life = oxygen_gen * co2_scrub;

        life.to_string()
    })
}

fn gen_raw(mut old: Vec<[u32; 2]>, num_len: usize, keep: &Vec<&str>) -> Vec<[u32; 2]> {
    for i in 0..num_len {
        let mut z = 0;
        let mut o = 0;
        for j in keep {
            match j.chars().nth(i).unwrap() {
                '0' => z += 1,
                '1' => o += 1,
                _ => {}
            }
        }

        old[i] = [z, o];
    }

    old
}

fn get_imax(raw: &Vec<[u32; 2]>, i: usize) -> char {
    let this_raw = raw[i];
    let imax = if this_raw[0] > this_raw[1] { '0' } else { '1' };
    imax
}
