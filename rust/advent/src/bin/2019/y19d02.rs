use advent::utilities::{get_input::get_input, intcode::IntCode};
use itertools::Itertools;
use utilities::structs::stopwatch::{ReportDuration, Stopwatch};

type Input<'a> = &'a str;
type Output = usize;

fn main() {
    let mut stopwatch = Stopwatch::new();
    stopwatch.start();
    let input = get_input(19, 2).unwrap();
    println!("Input parsed ({})", stopwatch.lap().report());
    println!("1. {} ({})", part1(&input), stopwatch.lap().report());
    println!("2. {} ({})", part2(&input), stopwatch.lap().report());
    println!("Total: {}", stopwatch.stop().report());
}

fn compute(comp: &mut IntCode, noun: usize, verb: usize) -> Output {
    comp.code[1] = noun;
    comp.code[2] = verb;
    comp.run();
    comp.code[0]
}

fn part1(input: Input) -> Output {
    let mut comp: IntCode = input.into();
    compute(&mut comp, 12, 2)
}

fn part2(input: Input) -> Output {
    let mut comp: IntCode = input.into();
    let code = comp.code.clone();
    for (noun, verb) in (0..100).cartesian_product(0..100) {
        comp.reset();
        comp.code = code.clone();
        let computation = compute(&mut comp, noun, verb);
        if computation == 19690720 {
            return 100 * noun + verb;
        }
    }
    unreachable!();
}

#[test]
fn default() {
    let input = get_input(19, 2).unwrap();
    assert_eq!(3895705, part1(&input));
    assert_eq!(6417, part2(&input));
}

// Input parsed (15μs)
// 1. 3895705 (12μs)
// 2. 6417 (1ms)
// Total: 1ms
