use common::{solution, Answer};

solution!("Dive!", 2);

fn part_a(input: &str) -> Answer {
    let mut dep: u32 = 0;
    let mut hor: u32 = 0;

    for i in input.lines() {
        let seg = i.split(' ').collect::<Vec<&str>>();
        let x = seg[1].parse::<u32>().unwrap();

        match seg[0] {
            "forward" => hor += x,
            "up" => dep -= x,
            "down" => dep += x,
            _ => {}
        }
    }

    (dep * hor).into()
}

fn part_b(input: &str) -> Answer {
    let mut dep: u32 = 0;
    let mut hor: u32 = 0;
    let mut aim: u32 = 0;

    for i in input.lines() {
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

    (dep * hor).into()
}
