use crate::{problem, Solution};

pub struct Day03;

impl Solution for Day03 {
    fn name(&self) -> &'static str {
        "Binary Diagnostic"
    }

    fn part_a(&self) -> String {
        let data = problem::load(2021, 3);
        let num_len = data.lines().next().unwrap().len();

        let mut gamma = vec![0; num_len];
        let mut epsilon = vec![1; num_len];

        for i in 0..num_len {
            let mut z = 0;
            let mut o = 0;

            data.lines().for_each(|j| match j.chars().nth(i).unwrap() {
                '0' => z += 1,
                '1' => o += 1,
                _ => {}
            });

            if o > z {
                epsilon[i] = 0;
                gamma[i] = 1;
            }
        }

        let gamma = int_from_bin(&gamma).unwrap();
        let epsilon = int_from_bin(&epsilon).unwrap();

        (epsilon * gamma).to_string()
    }

    fn part_b(&self) -> String {
        let data = problem::load(2021, 3);
        let num_len = data.lines().next().unwrap().len();

        let mut oxygen_keep = data.lines().collect::<Vec<&str>>();
        let mut oxygen_raw = vec![[0, 0]; num_len];
        let mut oxygen_gen = 0;

        let mut co2_keep = oxygen_keep.clone();
        let mut co2_raw = oxygen_raw.clone();
        let mut co2_scrub = 0;

        for i in 0..num_len {
            // Filter Oxygen
            let imax = get_imax(&oxygen_raw, i);
            oxygen_raw = gen_raw(oxygen_raw, num_len, &oxygen_keep);
            oxygen_keep.retain(|x| x.chars().into_iter().nth(i).unwrap() == imax);

            // Filter Co2
            let imax = get_imax(&co2_raw, i);
            co2_raw = gen_raw(co2_raw, num_len, &co2_keep);
            co2_keep.retain(|x| x.chars().into_iter().nth(i).unwrap() != imax);

            if oxygen_keep.len() == 1 {
                oxygen_gen = isize::from_str_radix(oxygen_keep.first().unwrap(), 2).unwrap();
            }

            if co2_keep.len() == 1 {
                co2_scrub = isize::from_str_radix(co2_keep.first().unwrap(), 2).unwrap();
            }
        }

        (oxygen_gen * co2_scrub).to_string()
    }
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

fn gen_raw(mut old: Vec<[u32; 2]>, num_len: usize, keep: &[&str]) -> Vec<[u32; 2]> {
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
        *old.get_mut(i).unwrap() = [z, o];
    }

    old
}

fn get_imax(raw: &[[u32; 2]], i: usize) -> char {
    if raw[i][0] > raw[i][1] {
        return '0';
    };
    '1'
}
