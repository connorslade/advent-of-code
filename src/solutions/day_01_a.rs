use crate::common;

pub fn main() {
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
}
