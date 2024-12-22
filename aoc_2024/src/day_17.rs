use std::fmt::{Debug, Write};

use common::{solution, Answer};
use itertools::Itertools;

solution!("Chronospatial Computer", 17);

fn part_a(input: &str) -> Answer {
    let mut processor = Processor::parse(input);

    while let Some(ins) = processor.next_instruction() {
        ins.opcode.evaluate(&mut processor, ins.argument);
    }

    processor
        .output
        .iter()
        .map(|x| x.to_string())
        .join(",")
        .into()
}

fn part_b(_input: &str) -> Answer {
    // There is no way to make a general solution to this problem, so we have to
    // rever engineer our specific input. In my case the input program
    // translates into the following, where x is the input to the first
    // register.
    //
    // (((x % 8) ^ 2) ^ x.wrapping_shr((x % 8) as u32 ^ 2) ^ 3) % 8
    // 
    // Because an input can cause multiple outputs, we then use a recursive
    // solver to create the needed input three bits at a time.

    let exp = [2, 4, 1, 2, 7, 5, 4, 7, 1, 3, 5, 5, 0, 3, 3, 0];

    fn solve(exp: &[u64], n: usize, a: u64) -> u64 {
        for inp in 0..=0b111u64 {
            let x = a << 3 | inp;
            let a = (((x % 8) ^ 2) ^ x.wrapping_shr((x % 8) as u32 ^ 2) ^ 3) % 8;

            if a == exp[n] {
                let out = if n == 0 { x } else { solve(exp, n - 1, x) };
                if out != 0 {
                    return out;
                }
            }
        }

        0
    }

    solve(&exp, exp.len() - 1, 0).into()
}

#[derive(Debug, Clone)]
struct Processor {
    registers: [u64; 3],
    program: Vec<u64>,
    ptr: usize,

    output: Vec<u64>,
}

#[derive(Debug, Clone)]
struct Instruction {
    opcode: OpCode,
    argument: (Option<Argument>, u64),
}

#[derive(Debug, Clone)]
enum OpCode {
    Adv, // A = (A/(2^x))
    Bxl, // B = (B^x) % 8
    Bst, // B = x % 8,
    Jnz, // Nothing of A=0, sets ptr to x
    Bxc, // B = B^C
    Out, // print x % 8
    Bdv, // B = (A/(2^x))
    Cdv, // C = (A/(2^x))
}

#[derive(Clone)]
enum Argument {
    Literal(u64),
    Register(u64),
}

impl Debug for Argument {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Argument::Literal(x) => f.write_fmt(format_args!("{x}")),
            Argument::Register(0) => f.write_char('A'),
            Argument::Register(1) => f.write_char('B'),
            Argument::Register(2) => f.write_char('C'),
            _ => unreachable!(),
        }
    }
}

impl Processor {
    fn parse(input: &str) -> Self {
        let (registers, program) = input.split_once("\n\n").unwrap();
        let mut registers = registers.lines().map(|x| {
            let (_, num) = x.rsplit_once(' ').unwrap();
            num.parse::<u64>().unwrap()
        });

        let registers = [
            registers.next().unwrap(),
            registers.next().unwrap(),
            registers.next().unwrap(),
        ];

        let (_, program) = program.rsplit_once(' ').unwrap();
        let program = program
            .replace('\n', "") //todo:cleanup
            .split(',')
            .map(|x| x.parse::<u64>().unwrap())
            .collect();

        Self {
            registers,
            program,
            ptr: 0,

            output: Vec::new(),
        }
    }

    fn reg(&self, reg: u64) -> u64 {
        self.registers[reg as usize]
    }

    fn reg_mut(&mut self, reg: u64) -> &mut u64 {
        &mut self.registers[reg as usize]
    }

    fn next_instruction(&mut self) -> Option<Instruction> {
        let opcode = *self.program.get(self.ptr)?;
        let combo = *self.program.get(self.ptr + 1)?;
        self.ptr += 2;

        Some(Instruction {
            opcode: OpCode::from_program(opcode)?,
            argument: (Argument::from_program(combo), combo),
        })
    }
}

impl OpCode {
    fn from_program(code: u64) -> Option<Self> {
        Some(match code {
            0 => OpCode::Adv,
            1 => OpCode::Bxl,
            2 => OpCode::Bst,
            3 => OpCode::Jnz,
            4 => OpCode::Bxc,
            5 => OpCode::Out,
            6 => OpCode::Bdv,
            7 => OpCode::Cdv,
            _ => return None,
        })
    }

    fn evaluate(self, proc: &mut Processor, (combo, literal): (Option<Argument>, u64)) {
        let x = combo.map(|x| x.get(proc)).unwrap_or_default();
        let l = literal;

        match self {
            OpCode::Adv => *proc.reg_mut(0) = proc.reg(0) / u64::pow(2, x as u32),
            OpCode::Bdv => *proc.reg_mut(1) = proc.reg(0) / u64::pow(2, x as u32),
            OpCode::Cdv => *proc.reg_mut(2) = proc.reg(0) / u64::pow(2, x as u32),

            OpCode::Jnz if proc.reg(0) == 0 => {}
            OpCode::Jnz => proc.ptr = l as usize,

            OpCode::Bxl => *proc.reg_mut(1) = proc.reg(1) ^ x,
            OpCode::Bst => *proc.reg_mut(1) = x % 8,
            OpCode::Bxc => *proc.reg_mut(1) = proc.reg(1) ^ proc.reg(2),
            OpCode::Out => proc.output.push(x % 8),
        }
    }

    #[allow(unused)]
    fn debug(&self, proc: &Processor, (combo, literal): (Option<Argument>, u64)) {
        match self {
            OpCode::Adv => println!("A = A >> {:?}", combo.unwrap()),
            OpCode::Bdv => println!("B = A >> {:?}", combo.unwrap()),
            OpCode::Cdv => println!("C = A >> {:?}", combo.unwrap()),

            OpCode::Jnz if proc.reg(0) == 0 => println!("No Jump"),
            OpCode::Jnz => println!("Jump to {literal}"),

            OpCode::Bxl => println!("B = B ^ {:?}", combo.unwrap()),
            OpCode::Bst => println!("B = {:?} % 8", combo.unwrap()),
            OpCode::Bxc => println!("B = B ^ C"),
            OpCode::Out => println!("Output {:?} % 8", combo.unwrap()),
        }
    }
}

impl Argument {
    fn from_program(code: u64) -> Option<Self> {
        Some(match code {
            0..=3 => Self::Literal(code),
            4 => Self::Register(0),
            5 => Self::Register(1),
            6 => Self::Register(2),
            _ => return None,
        })
    }

    fn get(self, processor: &Processor) -> u64 {
        match self {
            Argument::Literal(x) => x,
            Argument::Register(r) => processor.registers[r as usize],
        }
    }
}

#[cfg(test)]
mod test {
    use indoc::indoc;

    const CASE_A: &str = indoc! {"
        Register A: 729
        Register B: 0
        Register C: 0

        Program: 0,1,5,4,3,0
    "};

    #[test]
    fn part_a() {
        assert_eq!(super::part_a(CASE_A), "4,6,3,5,6,3,5,2,1,0".into());
    }
}
