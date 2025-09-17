use advent::utilities::get_input::get_input;
use utilities::structs::stopwatch::{ReportDuration, Stopwatch};

type Input = Vec<Instruction>;
type Output = i32;

fn main() {
    let mut stopwatch = Stopwatch::new();
    stopwatch.start();
    let input = get_input(20, 8).unwrap();
    let input = parse_input(&input);
    println!("Input parsed ({})", stopwatch.lap().report());
    println!("1. {} ({})", part1(&input), stopwatch.lap().report());
    println!("2. {} ({})", part2(&input), stopwatch.lap().report());
    println!("Total: {}", stopwatch.stop().report());
}

enum Op {
    Acc,
    Nop,
    Jmp,
}

impl From<&str> for Op {
    fn from(value: &str) -> Self {
        match value {
            "acc" => Op::Acc,
            "nop" => Op::Nop,
            "jmp" => Op::Jmp,
            _ => panic!("Unknown op {}", value),
        }
    }
}
struct Instruction {
    op: Op,
    arg: Output,
}
impl Instruction {
    fn execute(&self, acc: &mut Output, parser: &mut Output) {
        match self.op {
            Op::Acc => {
                *acc += self.arg;
                *parser += 1;
            },
            Op::Nop => { *parser += 1; },
            Op::Jmp => { *parser += self.arg; },
        }
    }
}

fn parse_input(input: &str) -> Input {
    input.lines()
        .map(|line| {
            let (op, arg) = line.split_once(' ').unwrap();
            let arg = arg.trim_start_matches('+').parse().unwrap();
            Instruction { op: op.into(), arg }
        })
        .collect()
}

fn solve(instructions: &Input, flip: Option<usize>) -> (Output, bool) {
    let mut past_states = vec![false; instructions.len()];
    let mut parser: Output = 0;
    let mut acc = 0;
    while (0..instructions.len()).contains(&(parser as usize)) {
        if past_states[parser as usize] { return (acc, false); }
        past_states[parser as usize] = true;
        let current = &instructions[parser as usize];
        if let Some(flip) = flip {
            if parser as usize == flip {
                let flipped_instruction = match current.op {
                    Op::Nop => { Instruction { op: Op::Jmp, .. *current } }
                    Op::Jmp => { Instruction { op: Op::Nop, .. *current } }
                    Op::Acc => { panic!("ACC cannot be a flipped instruction"); },
                };
                flipped_instruction.execute(&mut acc, &mut parser);
            } else {
                current.execute(&mut acc, &mut parser);
            }
        } else {
            current.execute(&mut acc, &mut parser);
        }
    }
    (acc, true)
}

fn part1(instructions: &Input) -> Output {
    solve(instructions, None).0
}

fn part2(instructions: &Input) -> Output {
    for flipped in 0..instructions.len() {
        if matches!(instructions[flipped].op, Op::Acc) { continue; }
        let answer = solve(instructions, Some(flipped));
        if answer.1 {
            return answer.0;
        }
    }
    unreachable!()
}

#[test]
fn default() {
    let input = get_input(20, 8).unwrap();
    let input = parse_input(&input);
    assert_eq!(1915, part1(&input));
    assert_eq!(944, part2(&input));
}

// Input parsed (41μs)
// 1. 1915 (6μs)
// 2. 944 (58μs)
// Total: 107μs