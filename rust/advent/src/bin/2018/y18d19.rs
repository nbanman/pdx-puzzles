use itertools::Itertools;
use advent::utilities::get_input::get_input;
use advent::utilities::opcode::{Op, Parameters};
use utilities::structs::stopwatch::{ReportDuration, Stopwatch};

type Input = (usize, Vec<(Op, Parameters)>);
type Output = usize;

fn main() {
    let mut stopwatch = Stopwatch::new();
    stopwatch.start();
    let input = get_input(18, 19).unwrap();
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
        .map(|line| {
            let (op, a, b, c) = line.split(' ').collect_tuple().unwrap();
            let op = Op::from(op);
            let parameters = Parameters {
                a: a.parse().unwrap(),
                b: b.parse().unwrap(),
                c: c.parse().unwrap(),
            };
            (op, parameters)
        })
        .collect();
    (p, code)
}

fn part1(input: &Input) -> Output {
    let (p, commands) = input;
    let mut register = vec![0; 6];
    while let Some((op, parameters)) = commands.get(register[*p]) {
        op.execute(&mut register, parameters);
        register[*p] += 1;
    }
    register[0]
}

fn part2(input: &Input) -> Output {
    let (p, commands) = input;
    let mut register = vec![1, 0, 0, 0, 0, 0];
    let mut prev = 0;
    while let Some((op, parameters)) = commands.get(register[*p]) {
        op.execute(&mut register, parameters);
        let pointer = &mut register[*p];
        *pointer += 1;
        if *pointer >= prev {
            prev = *pointer;
        } else {
            break;
        }
    }

    let c = commands[20].1.c;
    let target_num = register[c];

    // Loop is such that R3 starts as 1, R5 goes up by 1. R2 is R3 * R5. When R2 equals 10.5M, R0+= R3 
    // and R3++, R5 resets. If R2 goes past 10.5M w/o equaling it (not divisible), then R3++ and R5 resets
    // w/o RO going up. Thus, RO adds all the numbers that divide evenly into 10.5M. So add up all the 
    // factors of that.
    let mut factor_sum = target_num + 1;
    for i in 2..= (target_num as f64).sqrt().floor() as usize {
        if target_num % i == 0 {
            factor_sum += i + target_num / i
        }
    }
    factor_sum
}

#[test]
fn default() {
    let input = get_input(18, 19).unwrap();
    let input = parse_input(&input);
    assert_eq!(1764, part1(&input));
    assert_eq!(18992484, part2(&input));
}

// Input parsed (18μs)
// 1. 1764 (45ms)
// 2. 18992484 (8μs)
// Total: 45ms
