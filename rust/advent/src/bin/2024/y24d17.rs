use advent::utilities::get_input::get_input;
use itertools::Itertools;
use utilities::{parsing::get_numbers::ContainsNumbers, structs::stopwatch::{ReportDuration, Stopwatch}};

type Input<'a> = &'a str;

fn main() {
    let mut stopwatch = Stopwatch::new();
    stopwatch.start();
    let input = get_input(24, 17).unwrap();
    println!("Input parsed ({})", stopwatch.lap().report());
    println!("1. {} ({})", part1(&input), stopwatch.lap().report());
    println!("2. {} ({})", part2(&input), stopwatch.lap().report());
    println!("Total: {}", stopwatch.stop().report());
}

fn parse_input(input: &str) -> (usize, usize, usize, Vec<usize>) {
    let mut numbers = input.get_numbers();
    let (a, b, c) = numbers.next_tuple().unwrap();
    let program = numbers.collect();
    (a, b, c, program)
}

fn combo_value(operand: usize, a: usize, b: usize, c: usize) -> usize {
    match operand {
        4 => a,
        5 => b,
        6 => c,
        x => x
    }
}

fn solve(a: usize, b: usize, c: usize, program: &[usize]) -> Vec<usize> {
    let (mut a, mut b, mut c) = (a, b, c);
    let mut cursor = 0;
    let mut out = Vec::new();

    while cursor < program.len() {
        let opcode = program[cursor];
        let operand = program[cursor + 1];
        cursor += 2;

        match opcode {
            0 => a /= 2usize.pow(combo_value(operand, a, b, c).try_into().unwrap()),
            1 => b = b ^ operand,
            2 => b = combo_value(operand, a, b, c) % 8,
            3 => if a != 0 { cursor = operand },
            4 => b = b ^ c,
            5 => out.push(combo_value(operand, a, b, c) % 8),
            6 => b = a / 2usize.pow(combo_value(operand, a, b, c).try_into().unwrap()),
            7 => c = a / 2usize.pow(combo_value(operand, a, b, c).try_into().unwrap()),
            x => { panic!("Invalid opcode. Should be between 0 and 7 instead of {x}."); }
        }
    }

    out
}

fn part1(input: Input) -> String {
    let (a, b, c, program) = parse_input(input);
    solve(a, b, c, &program).iter().join(",")
}

fn part2(input: Input) -> usize {
    let (_, b, c, program) = parse_input(input);
    let mut counter = 1;
    loop {
        let answer = solve(counter, b, c, &program);
        let matching = &answer == &program[program.len() - answer.len()..];
        if matching {
            if answer.len() == program.len() { break; }
            counter *= 8;
        } else {
            counter += 1;
        }
    }
    counter
}

#[test]
fn default() {
    let input = get_input(24, 17).unwrap();
    assert_eq!("5,1,3,4,3,7,2,1,7".to_string(), part1(&input));
    assert_eq!(216584205979245, part2(&input));
}

// Input parsed (9μs)
// 1. 5,1,3,4,3,7,2,1,7 (7μs)
// 2. 216584205979245 (204μs)
// Total: 223μs