use crate::common::{self, Solution};

pub struct Day09 {}

impl Solution for Day09 {
    fn name(&self) -> String {
        "Smoke Basin".to_owned()
    }

    fn part_a(&self) -> String {
        let data = parse(common::load("09"));
        let low = lowest(data);

        low.iter().map(|x| *x + 1).sum::<u32>().to_string()
    }

    fn part_b(&self) -> String {
        let data = parse(common::load("09"));
        let basins = basins(data);

        basins.iter().rev().take(3).product::<u32>().to_string()
    }
}

fn parse(inp: String) -> Vec<Vec<u32>> {
    inp.lines()
        .map(|x| x.chars().map(|f| f.to_digit(10).unwrap()).collect())
        .collect()
}

fn lowest(inp: Vec<Vec<u32>>) -> Vec<u32> {
    inp.iter()
        .enumerate()
        .flat_map(|(i, line)| {
            let inp = &inp;
            line.iter().enumerate().filter_map(move |(j, &h)| {
                if (i == 0 || h < inp[i - 1][j])
                    && (i == inp.len() - 1 || h < inp[i + 1][j])
                    && (j == 0 || h < inp[i][j - 1])
                    && (j == line.len() - 1 || h < inp[i][j + 1])
                {
                    return Some(h);
                }
                None
            })
        })
        .collect::<Vec<u32>>()
}

fn basins(mut inp: Vec<Vec<u32>>) -> Vec<u32> {
    let mut basins = Vec::new();
    for i in 0..inp.len() {
        for j in 0..inp[0].len() {
            if inp[i][j] < 9 {
                basins.push(basin(&mut inp, j, i));
            }
        }
    }

    basins.sort_unstable();
    basins
}

fn basin(map: &mut Vec<Vec<u32>>, x: usize, y: usize) -> u32 {
    map[y][x] = 9;
    [(0, -1), (0, 1), (-1, 0), (1, 0)]
        .iter()
        .map(|(xx, yy)| ((x as isize + xx) as usize, (y as isize + yy) as usize))
        .fold(1, |inc, (x, y)| {
            if map.get(y).and_then(|l| l.get(x)).map(|&n| n < 9) == Some(true) {
                return inc + basin(map, x, y);
            }
            inc
        })
}
