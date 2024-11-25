use common::{solution, Answer};

solution!("Cathode-Ray Tube", (2022, 00));

fn part_a(input: &str) -> Answer {
    let instructions = parse(input);
    let cycles = cycle(&instructions);

    let mut out = 0;
    for i in (0..6).map(|x| 20 + 40 * x) {
        out += cycles[0..i].iter().sum::<i32>() * i as i32;
    }

    out.into()
}

fn part_b(input: &str) -> Answer {
    let instructions = parse(input);
    let mut out = "\n".to_owned();
    let mut sprite = 1;
    let mut cycle = 0;

    for i in instructions {
        let (goto, inc) = i.info();
        for i in cycle..goto + cycle {
            let diff = i % 40_i32 - sprite;
            if diff.abs() < 2 {
                out.push('#');
                continue;
            }
            out.push(' ');
        }

        cycle += goto;
        sprite += inc;
    }

    make_lines(&out, 40).into()
}

#[derive(Debug, Clone, Copy)]
enum Instruction {
    Noop,
    Addx(i32),
}

impl Instruction {
    fn info(&self) -> (i32, i32) {
        match self {
            Instruction::Noop => (1, 0),
            Instruction::Addx(x) => (2, *x),
        }
    }
}

fn make_lines(raw: &str, width: usize) -> String {
    raw.char_indices()
        .map(|(i, c)| {
            if i % width == 0 {
                return format!("{}\n", c);
            }
            c.to_string()
        })
        .collect()
}

fn cycle(ins: &[Instruction]) -> Vec<i32> {
    let mut cycle = 0;
    let mut cycles = vec![0; 240];
    cycles[0] = 1;

    for i in ins {
        match i {
            Instruction::Noop => cycle += 1,
            Instruction::Addx(x) => {
                cycles[cycle + 2] += x;
                cycle += 2;
            }
        }
    }

    cycles
}

fn parse(raw: &str) -> Vec<Instruction> {
    let mut out = Vec::new();

    for line in raw.lines() {
        let mut parts = line.split_whitespace();
        let ins = parts.next().unwrap();
        match ins {
            "addx" => out.push(Instruction::Addx(
                parts.next().unwrap().parse::<i32>().unwrap(),
            )),
            "noop" => out.push(Instruction::Noop),
            _ => panic!("Unknown instruction: {}", ins),
        }
    }

    out
}
