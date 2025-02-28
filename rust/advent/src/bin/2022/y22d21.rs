use advent::utilities::get_input::get_input;
use lazy_regex::regex;
use std::{fmt::Display, ops::Index};
use utilities::structs::{
    indexer::Indexer,
    stopwatch::{ReportDuration, Stopwatch},
};

type Input<'a> = &'a str;
type Int = i64;
type Output = Int;

#[derive(Copy, Clone, Debug)]
enum Operation {
    Add,
    Sub,
    Mul,
    Div,
    OddSub,
    OddDiv,
}

#[derive(Clone, Copy, Debug)]
enum Job {
    Unassigned,
    Call(Int),
    Wait(Operation, usize, usize),
}

#[derive(Debug)]
struct Monkeys {
    root: usize,
    jobs: Vec<Job>,
}

#[derive(Clone, Debug)]
enum Equation {
    X,
    Number(Int),
    Composite(Box<Equation>, Box<Equation>, Operation),
}

impl Display for Equation {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Equation::X => write!(f, "x"),
            Equation::Number(n) => write!(f, "{n}"),
            Equation::Composite(left, right, op) => {
                let op = match op {
                    Operation::Add => "+",
                    Operation::Sub => "-",
                    Operation::Mul => "*",
                    Operation::Div => "/",
                    Operation::OddSub => "o-",
                    Operation::OddDiv => "o/",
                };
                write!(f, "({left} {op} {right})")
            }
        }
    }
}

impl Equation {
    fn contains_x(&self) -> bool {
        match self {
            Equation::X => true,
            Equation::Number(_) => false,
            Equation::Composite(left, right, _) => left.contains_x() || right.contains_x(),
        }
    }

    fn solve(&self, to: Equation) -> Equation {
        match self {
            Equation::X => to.clone(),
            Equation::Number(_) => {
                panic!("numbers cannot be solved!")
            }
            Equation::Composite(left, right, op) => {
                let move_right = left.contains_x();
                let (moving, stay) = if move_right {
                    ((**right).clone(), (**left).clone())
                } else {
                    ((**left).clone(), (**right).clone())
                };
                let new_op = match op {
                    Operation::Add => Operation::Sub,
                    Operation::Sub => {
                        if move_right {
                            Operation::Add
                        } else {
                            Operation::OddSub
                        }
                    }
                    Operation::Mul => Operation::Div,
                    _ => {
                        if move_right {
                            Operation::Mul
                        } else {
                            Operation::OddDiv
                        }
                    }
                };
                let new_to = Self::Composite(Box::new(to.clone()), Box::new(moving), new_op);
                stay.solve(new_to)
            }
        }
    }

    fn calculate(&self) -> i64 {
        match self {
            Equation::X => {
                panic!("X should be removed from equation!");
            }
            Equation::Number(n) => *n,
            Equation::Composite(left, right, op) => {
                operate(*op, left.calculate(), right.calculate())
            }
        }
    }
}

fn main() {
    let mut stopwatch = Stopwatch::new();
    stopwatch.start();
    let input = get_input(22, 21).unwrap();
    println!("Input parsed ({})", stopwatch.lap().report());
    println!("1. {} ({})", part1(&input), stopwatch.lap().report());
    println!("2. {} ({})", part2(&input), stopwatch.lap().report());
    println!("Total: {}", stopwatch.stop().report());
}

fn get_monkeys(input: &str, part2: bool) -> Monkeys {
    let number_of_monkeys = input.lines().count();
    let mut indexer = Indexer::new();
    let mut root = 0;
    let mut jobs = vec![Job::Unassigned; number_of_monkeys];
    let mut left_call_register = vec![Vec::new(); number_of_monkeys];
    let mut right_call_register = vec![Vec::new(); number_of_monkeys];

    let pattern =
        regex!(r"(?P<monkey>\w+): (?P<call>\d+)?(?:(?P<left>\w+) (?P<op>[-+*/]) (?P<right>\w+))?");
    for caps in pattern.captures_iter(input) {
        let name = caps.index("monkey");
        let monkey = indexer.get_or_assign_index(name.to_string());
        if name == "root" {
            root = monkey;
        }

        let job = if let Some(call) = caps.name("call") {
            if name == "humn" && part2 {
                Job::Unassigned
            } else {
                let call = call.as_str().parse().unwrap();
                Job::Call(call)
            }
        } else {
            let left = indexer.get_or_assign_index(caps.index("left").to_string());
            let right = indexer.get_or_assign_index(caps.index("right").to_string());
            let operation = if monkey == root && part2 {
                Operation::Div
            } else {
                match caps.index("op") {
                    "+" => Operation::Add,
                    "-" => Operation::Sub,
                    "*" => Operation::Mul,
                    "/" => Operation::Div,
                    c => {
                        panic!("{c} not a recognized operation!");
                    }
                }
            };
            left_call_register[left].push(monkey);
            right_call_register[right].push(monkey);
            Job::Wait(operation, left, right)
        };
        jobs[monkey] = job;
    }
    for monkey in 0..jobs.len() {
        assign_call(monkey, &mut jobs, &left_call_register, &right_call_register);
    }
    Monkeys { root, jobs }
}

fn assign_call(
    monkey: usize,
    jobs: &mut Vec<Job>,
    left_call_register: &Vec<Vec<usize>>,
    right_call_register: &Vec<Vec<usize>>,
) {
    if let Job::Wait(operation, left, right) = jobs[monkey] {
        if let Job::Call(left_call) = jobs[left] {
            if let Job::Call(right_call) = jobs[right] {
                let call = operate(operation, left_call, right_call);
                jobs[monkey] = Job::Call(call);
                for &left_monkey in left_call_register[monkey].iter() {
                    assign_call(left_monkey, jobs, left_call_register, right_call_register);
                }
                for &right_monkey in right_call_register[monkey].iter() {
                    assign_call(right_monkey, jobs, left_call_register, right_call_register);
                }
            }
        }
    }
}

fn operate(operation: Operation, a: Int, b: Int) -> Int {
    match operation {
        Operation::Add => a + b,
        Operation::Sub => a - b,
        Operation::Mul => a * b,
        Operation::Div => a / b,
        Operation::OddSub => -(a - b),
        Operation::OddDiv => b / a,
    }
}

fn part1(input: Input) -> Output {
    let Monkeys { root, jobs } = get_monkeys(input, false);
    match jobs[root] {
        Job::Call(root_call) => root_call,
        job => {
            panic!("Root not resolved!: {:?}", job);
        }
    }
}

fn part2(input: Input) -> Output {
    let Monkeys { root, jobs } = get_monkeys(input, true);
    get_equation(root, &jobs)
        .solve(Equation::Number(1))
        .calculate()
}

fn get_equation(monkey: usize, jobs: &Vec<Job>) -> Equation {
    match jobs[monkey] {
        Job::Unassigned => Equation::X,
        Job::Call(n) => Equation::Number(n),
        Job::Wait(operation, left, right) => Equation::Composite(
            Box::new(get_equation(left, jobs)),
            Box::new(get_equation(right, jobs)),
            operation,
        ),
    }
}

#[test]
fn default() {
    let input = get_input(22, 21).unwrap();
    assert_eq!(309248622142100, part1(&input));
    assert_eq!(3757272361782, part2(&input));
}

// Input parsed (188μs)
// 1. 309248622142100 (2ms)
// 2. 3757272361782 (1ms)
// Total: 3ms
