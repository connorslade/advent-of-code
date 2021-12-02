solution!("02-Dive!-B", || {
    let d = common::load("02");
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
});
