use common::{Answer, Solution};

pub struct Day11;

impl Solution for Day11 {
    fn name(&self) -> &'static str {
        "Dumbo Octopus"
    }

    fn part_a(&self, input: &str) -> Answer {
        let mut octopi = parse(input);

        (0..100)
            .map(|_| step_octopi(&mut octopi))
            .sum::<usize>()
            .into()
    }

    fn part_b(&self, input: &str) -> Answer {
        let mut octopi = parse(input);
        let octopi_count = octopi.len() * octopi[0].len();
        let mut i = 0;

        loop {
            i += 1;

            let flash = step_octopi(&mut octopi);
            if flash == octopi_count {
                break;
            }
        }

        i.into()
    }
}

struct Octopus {
    state: u8,
    flashed: bool,
}

impl Octopus {
    fn tick(&mut self) -> bool {
        if self.flashed {
            return false;
        }

        self.state += 1;
        if self.state > 9 {
            self.state = 0;
            self.flashed = true;
            return true;
        }

        false
    }
}

fn step_octopi(octopi: &mut Vec<Vec<Octopus>>) -> usize {
    for y in 0..octopi.len() {
        for x in 0..octopi[0].len() {
            tick_octopi(octopi, x, y);
        }
    }

    let mut flash = 0;
    for i in octopi.iter_mut() {
        for j in i.iter_mut() {
            if j.flashed {
                flash += 1;
            }

            j.flashed = false;
        }
    }

    flash
}

fn tick_octopi(octopi: &mut Vec<Vec<Octopus>>, x: usize, y: usize) {
    let flash = octopi[y][x].tick();
    if !flash {
        return;
    }

    for i in octo_neighbors(octopi, x, y) {
        tick_octopi(octopi, i.0, i.1);
    }
}

fn octo_neighbors(octopi: &Vec<Vec<Octopus>>, x: usize, y: usize) -> Vec<(usize, usize)> {
    let mut out = Vec::new();
    let (lenx, leny) = (octopi[0].len() as isize, octopi.len() as isize);
    let (x, y) = (x as isize, y as isize);

    for i in 0..3 {
        for j in 0..3 {
            if i == 1 && j == 1 {
                continue;
            }

            let x = x + i - 1;
            let y = y + j - 1;

            if x < 0 || y < 0 || x >= lenx || y >= leny {
                continue;
            }

            out.push((x as usize, y as usize));
        }
    }

    out
}

fn parse(raw: &str) -> Vec<Vec<Octopus>> {
    let mut out = Vec::new();

    for i in raw.lines() {
        out.push(
            i.chars()
                .map(|x| Octopus {
                    state: x.to_digit(10).unwrap() as u8,
                    flashed: false,
                })
                .collect(),
        );
    }

    out
}
