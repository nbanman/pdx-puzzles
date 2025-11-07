use advent::utilities::get_input::get_input;
use itertools::Itertools;
use utilities::{parsing::get_numbers::ContainsNumbers, structs::{coord::Coord2U, stopwatch::{ReportDuration, Stopwatch}}};

type Input = Vec<Instruction>;
type Output = usize;
type Pixel = Coord2U;

const LENGTH: usize = 1_000;
const LIGHT_SIZE: usize = LENGTH * LENGTH;

fn main() {
    let mut stopwatch = Stopwatch::new();
    stopwatch.start();
    let input = get_input(15, 6).unwrap();
    let input = parse_input(&input);
    println!("Input parsed ({})", stopwatch.lap().report());
    println!("1. {} ({})", part1(&input), stopwatch.lap().report());
    println!("2. {} ({})", part2(&input), stopwatch.lap().report());
    println!("Total: {}", stopwatch.stop().report());
}

#[derive(Debug, Clone, Copy)]
enum Command { On, Off, Toggle }

struct Instruction {
    command: Command,
    tl: Pixel,
    br: Pixel,
}

impl<'a> From<&'a str> for Instruction {
    fn from(value: &'a str) -> Self {
        let light = match value {
            light if light.starts_with("turn on") => Command::On,
            light if light.starts_with("turn off") => Command::Off,
            light if light.starts_with("toggle") => Command::Toggle,
            _ => unreachable!(),
        };
        let (x1, y1, x2, y2) = value.get_numbers().collect_tuple().unwrap();
        Self { command: light, tl: Pixel::new2d(x1, y1), br: Pixel::new2d(x2, y2) }
    }
}

fn parse_input(input: &str) -> Input {
    input.lines().map(|line| line.into()).collect()
}

fn solve<F>(instructions: &Input, operation: F) -> Output
where
    F: Fn(Command, usize) -> usize,
{
    let mut lights = vec![0usize; LIGHT_SIZE];
    for inst in instructions {
        for (y, x) in (inst.tl.y()..=inst.br.y()).cartesian_product(inst.tl.x()..=inst.br.x()) {
            let index = y * LENGTH + x;
            lights[index] = operation(inst.command, lights[index]);
        }
    }
    lights.iter().sum()
}

fn part1(instructions: &Input) -> Output {
    solve(instructions, |command, pixel_value| match command {
        Command::On => 1,
        Command::Off => 0,
        Command::Toggle => if pixel_value == 0 { 1 } else { 0 },
    })
}

fn part2(instructions: &Input) -> Output {
    solve(instructions, |command, pixel_value| match command {
        Command::On => pixel_value + 1,
        Command::Off => pixel_value.checked_sub(1).unwrap_or_default(),
        Command::Toggle => pixel_value + 2,
    })
}

#[test]
fn default() {
    let input = get_input(15, 6).unwrap();
    let input = parse_input(&input);
    assert_eq!(569999, part1(&input));
    assert_eq!(17836115, part2(&input));
}

// Input parsed (59μs)
// 1. 569999 (21ms)
// 2. 17836115 (24ms)
// Total: 46ms
