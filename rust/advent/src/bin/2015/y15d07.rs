use advent::utilities::get_input::get_input;
use itertools::Itertools;
use rustc_hash::FxHashMap;
use utilities::structs::stopwatch::{ReportDuration, Stopwatch};

type Input<'a> = Vec<Instruction<'a>>;
type Output = u16;

#[derive(Debug, Copy, Clone)]
enum Arg<'a> {
    Wire(&'a str),
    Value(u16),
}

impl<'a> Arg<'a> {
    fn value(&self, wires: &'a FxHashMap<&'a str, u16>) -> Option<u16> {
        match self {
            Arg::Wire(wire) => wires.get(wire).copied(),
            Arg::Value(v) => Some(*v),
        }
    }
}

impl<'a> From<&'a str> for Arg<'a> {
    fn from(value: &'a str) -> Self {
        value.parse::<u16>()
            .map(|n| Self::Value(n))
            .unwrap_or(Self::Wire(value))
    }
}

#[derive(Debug, Copy, Clone)]
enum Command<'a> {
    Assign(Arg<'a>),
    And { x: Arg<'a>, y: &'a str },
    Or { x: &'a str, y: &'a str },
    Lshift { wire: &'a str, shift: u16 },
    Rshift { wire: &'a str, shift: u16 },
    Not(&'a str),
}

#[derive(Debug, Copy, Clone)]
struct Instruction<'a> {
    command: Command<'a>,
    assignee: &'a str,
}

impl<'a> Instruction<'a> {
    fn execute(&self, wires: &mut FxHashMap<&'a str, u16>) -> bool {
        match self.command {
            Command::Assign(arg) => {
                arg.value(wires)
                    .map(|v| {
                        wires.insert(self.assignee, v);
                        true
                    })
                    .unwrap_or_default()
            },
            Command::And { x, y } => {
                x.value(wires)
                    .map(|x| {
                        wires.get(y)
                            .copied()
                            .map(|y| {
                                wires.insert(self.assignee, x & y);
                                true
                            })
                            .unwrap_or_default()
                    })
                    .unwrap_or_default()
            },
            Command::Or { x, y } => {
                wires.get(x).copied()
                    .map(|x| {
                        wires.get(y).copied()
                            .map(|y| {
                                wires.insert(self.assignee, x | y);
                                true
                            })
                            .unwrap_or_default()
                    })
                    .unwrap_or_default()
            },
            Command::Lshift { wire, shift } => {
                wires.get(wire).copied()
                    .map(|wire| {
                        wires.insert(self.assignee, wire << shift);
                        true
                    })
                    .unwrap_or_default()
            },
            Command::Rshift { wire, shift } => {
                wires.get(wire).copied()
                    .map(|wire| {
                        wires.insert(self.assignee, wire >> shift);
                        true
                    })
                    .unwrap_or_default()
            },
            Command::Not(wire) => {
                wires.get(wire).copied()
                    .map(|wire| {
                        wires.insert(self.assignee, !wire);
                        true
                    })
                    .unwrap_or_default()
            },
        }
    }
}

impl<'a> From<&'a str> for Instruction<'a> {
    fn from(value: &'a str) -> Self {
        let mut tokens = value.split(' ');
        let (a, b, c) = (tokens.next().unwrap(), tokens.next().unwrap(), tokens.next().unwrap());
        match b {
            "->" => { // ASSIGN
                let command = Command::Assign(a.into());
                Self { command, assignee: c }
            },
            "AND" => {
                let command = Command::And { x: a.into(), y: c };
                tokens.next();
                let assignee = tokens.next().unwrap();
                Self { command, assignee }
            },
            "OR" => {
                let command = Command::Or { x: a, y: c };
                tokens.next();
                let assignee = tokens.next().unwrap();
                Self { command, assignee }
            },
            "LSHIFT" => {
                let command = Command::Lshift { wire: a, shift: c.parse().unwrap() };
                tokens.next();
                let assignee = tokens.next().unwrap();
                Self { command, assignee }
            }
            "RSHIFT" => {
                let command = Command::Rshift { wire: a, shift: c.parse().unwrap() };
                tokens.next();
                let assignee = tokens.next().unwrap();
                Self { command, assignee }
            }
            _ => { // NOT
                let command = Command::Not(b);
                Self { command, assignee: tokens.next().unwrap() }
            },
        }
    }
}

fn main() {
    let mut stopwatch = Stopwatch::new();
    stopwatch.start();
    let input = get_input(15, 7).unwrap();
    let input = parse_input(&input);
    println!("Input parsed ({})", stopwatch.lap().report());
    println!("1. {} ({})", part1(&input), stopwatch.lap().report());
    println!("2. {} ({})", part2(&input), stopwatch.lap().report());
    println!("Total: {}", stopwatch.stop().report());
}

fn parse_input(input: &str) -> Input<'_> {
    input.lines().map(|line| line.into()).collect()
}

fn run<'a>(instructions: &'a Input, wires: &mut FxHashMap<&'a str, u16>) -> Output {
    let mut instructions = instructions.iter().collect_vec();
    let mut unexecuted = Vec::new();
    while !instructions.is_empty() {
        for instruction in instructions.drain( .. ) {
            if !instruction.execute(wires) {
                unexecuted.push(instruction);
            }
        }
        std::mem::swap(&mut instructions, &mut unexecuted);
    }
    *wires.get("a").unwrap()
}

fn part1(instructions: &Input) -> Output {
    let mut wires = FxHashMap::default();
    run(instructions, &mut wires)
}

fn part2(instructions: &Input) -> Output {
    let mut wires = FxHashMap::default();
    let b = run(instructions, &mut wires);

    wires.clear();
    wires.insert("b", b);
    let instructions = instructions.iter()
        .filter(|instruction| instruction.assignee != "b")
        .copied()
        .collect();
    run(&instructions, &mut wires)
}

#[test]
fn default() {
    let input = get_input(15, 7).unwrap();
    let input = parse_input(&input);
    assert_eq!(46065, part1(&input));
    assert_eq!(14134, part2(&input));
}

// Input parsed (53μs)
// 1. 46065 (193μs)
// 2. 14134 (315μs)
// Total: 564μs 
