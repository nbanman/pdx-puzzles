use advent::utilities::get_input::get_input;
use utilities::structs::stopwatch::{ReportDuration, Stopwatch};

type Input = Vec<Instruction>;
type Output = i32;

#[derive(Debug, Copy, Clone)]
enum Register { A, B }

impl From<&str> for Register {
    fn from(value: &str) -> Self {
        match &value[0..1] {
            "a" => Self::A,
            "b" => Self::B,
            _ => unreachable!(),
        }
    }
}

#[derive(Debug, Copy, Clone)]
enum Instruction {
    Hlf(Register),
    Tpl(Register),
    Inc(Register),
    Jmp(i32),
    Jie(Register, i32),
    Jio(Register, i32),
}

impl From<&str> for Instruction {
    fn from(value: &str) -> Self {
        let mut tokens = value.split(' ');
        let (command, arg1) = (tokens.next().unwrap(), tokens.next().unwrap());
        match command {
            "hlf" => Self::Hlf(arg1.into()),
            "tpl" => Self::Tpl(arg1.into()),
            "inc" => Self::Inc(arg1.into()),
            "jmp" => Self::Jmp(arg1.parse().unwrap()),
            "jie" => {
                let offset = tokens.next().unwrap().parse().unwrap();
                Self::Jie(arg1.into(), offset)
            },
            "jio" => {
                let offset = tokens.next().unwrap().parse().unwrap();
                Self::Jio(arg1.into(), offset)
            },
            _ => unreachable!(),
        }
    }
}

fn main() {
    let mut stopwatch = Stopwatch::new();
    stopwatch.start();
    let input = get_input(15, 23).unwrap();
    let input = parse_input(&input);
    println!("Input parsed ({})", stopwatch.lap().report());
    println!("1. {} ({})", part1(&input), stopwatch.lap().report());
    println!("2. {} ({})", part2(&input), stopwatch.lap().report());
    println!("Total: {}", stopwatch.stop().report());
}

fn parse_input(input: &str) -> Input {
    input.lines().map(|it| it.into()).collect()
}

fn solve(instructions: &Input, mut a: i32) -> Output {
    let mut b: i32 = 0;
    let mut index: i32 = 0;
    let len = instructions.len() as i32;
    while index < len {
        match instructions[index as usize] {
            Instruction::Hlf(register) => {
                match register {
                    Register::A => { a /= 2; },
                    Register::B => { b /= 2; },
                }
                index += 1;
            },
            Instruction::Tpl(register) => {
                match register {
                    Register::A => { a *= 3; },
                    Register::B => { b *= 3; },
                }
                index += 1;
            },
            Instruction::Inc(register) => {
                match register {
                    Register::A => { a += 1; },
                    Register::B => { b += 1; },
                }
                index += 1;
            },
            Instruction::Jmp(offset) => { index += offset; },
            Instruction::Jie(register, offset) => index +=
                jmp_if(register, offset, a, b, |reg_value| reg_value & 1 == 0),
            Instruction::Jio(register, offset) => index +=
                jmp_if(register, offset, a, b, |reg_value| reg_value == 1),
        }
    }
    b
}

fn jmp_if<F>(register: Register, offset: i32, a: i32, b: i32, predicate: F) -> i32
where F: Fn(i32) -> bool
{
    let reg_value = match register {
        Register::A => a,
        Register::B => b,
    };
    if predicate(reg_value) {
        offset
    } else {
        1
    }
}

fn part1(instructions: &Input) -> Output {
    solve(instructions, 0)
}

fn part2(instructions: &Input) -> Output {
    solve(instructions, 1)
}

#[test]
fn default() {
    let input = get_input(15, 23).unwrap();
    let input = parse_input(&input);
    assert_eq!(255, part1(&input));
    assert_eq!(334, part2(&input));
}

// Input parsed (20μs)
// 1. 255 (7μs)
// 2. 334 (5μs)
// Total: 35μs