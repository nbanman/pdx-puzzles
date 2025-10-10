use advent::utilities::{get_input::get_input, opcode::{Op, Parameters}};
use itertools::Itertools;
use rustc_hash::FxHashSet;
use utilities::structs::stopwatch::{ReportDuration, Stopwatch};

type Input = (usize, Vec<Command>);
type Output = usize;

struct Command {
    op: Op,
    line_no: usize,
    parameters: Parameters,
}

fn main() {
    let mut stopwatch = Stopwatch::new();
    stopwatch.start();
    let input = get_input(18, 21).unwrap();
    let input = parse_input(&input);
    println!("Input parsed ({})", stopwatch.lap().report());
    println!("1. {} ({})", part1(&input), stopwatch.lap().report());
    println!("2. {} ({})", part2(&input), stopwatch.lap().report());
    println!("Total: {}", stopwatch.stop().report());
}

fn parse_input(input: &str) -> Input {
    let mut lines = input.lines();
    let p = lines.next().unwrap()
        .as_bytes()
        .last()
        .map(|&b| (b - b'0') as usize)
        .unwrap();
    let code = lines
        .enumerate()
        .map(|(line_no, line)| {
            let (op, a, b, c) = line.split(' ').collect_tuple().unwrap();
            let op = Op::from(op);
            let parameters = Parameters {
                a: a.parse().unwrap(),
                b: b.parse().unwrap(),
                c: c.parse().unwrap(),
            };
            Command {
                op,
                line_no,
                parameters,
            }
        })
        .collect();
    (p, code)
}

fn solve(input: &Input, highest: bool) -> Output {
    let (p, commands) = input;
    let mut register = vec![0; 6];
    let mut r1_set: FxHashSet<usize> = FxHashSet::default();
    let mut last = 0;
    loop {
        let command = &commands[register[*p]];
        command.op.execute(&mut register, &command.parameters);
        register[*p] += 1;
        if command.line_no == 28 {
            if highest {
                if r1_set.contains(&register[1]) {
                    return last;
                }
            } else {
                return register[1];
            }
            r1_set.insert(register[1]);
            last = register[1];
        }
    }
}

fn part1(input: &Input) -> Output {
    solve(input, false)
}

fn part2(input: &Input) -> Output {
    solve(input, true)
}

#[test]
fn default() {
    let input = get_input(18, 21).unwrap();
    let input = parse_input(&input);
    assert_eq!(3345459, part1(&input));
    assert_eq!(5857354, part2(&input));
}

// Input parsed (20μs)
// 1. 3345459 (20μs)
// 2. 5857354 (16.175s)
// Total: 16.175s
