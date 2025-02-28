use advent::utilities::get_input::get_input;
use utilities::{
    parsing::get_numbers::get_numbers,
    structs::stopwatch::{ReportDuration, Stopwatch},
};

type Input = Vec<Vec<usize>>;
type Output = usize;

fn main() {
    let mut stopwatch = Stopwatch::new();
    stopwatch.start();
    let input = get_input(24, 7).unwrap();
    println!("Input parsed ({})", stopwatch.lap().report());
    println!("1. {} ({})", part1(&input), stopwatch.lap().report());
    println!("2. {} ({})", part2(&input), stopwatch.lap().report());
    println!("Total: {}", stopwatch.stop().report());
}

#[derive(Clone, Copy, Debug)]
enum Operation {
    Sub,
    Divide,
    Slough,
}

impl Operation {
    fn operate(&self, a: usize, b: usize) -> Option<usize> {
        match self {
            Operation::Sub => a.checked_sub(b),
            Operation::Divide => {
                if a % b == 0 {
                    Some(a / b)
                } else {
                    None
                }
            }
            Operation::Slough => {
                let a = a.to_string();
                let b = b.to_string();
                if a.ends_with(&b) {
                    let trimmed = &a[..a.len() - b.len()];
                    trimmed.parse().ok()
                } else {
                    None
                }
            }
        }
    }
}

fn solve(equations: Input, operations: &[Operation]) -> Output {
    equations
        .into_iter()
        .filter(|equation| test_validity(equation, operations))
        .map(|equation| equation[0])
        .sum()
}

fn test_validity(equation: &[usize], operations: &[Operation]) -> bool {
    fn dfs(current: usize, equation: &[usize], index: usize, operations: &[Operation]) -> bool {
        if current < equation[1] {
            return false;
        }
        if index == 1 {
            return current == equation[1];
        }
        operations.iter().any(|operation| {
            if let Some(next) = operation.operate(current, equation[index]) {
                dfs(next, equation, index - 1, operations)
            } else {
                false
            }
        })
    }
    dfs(equation[0], equation, equation.len() - 1, operations)
}

fn parse_input(input: &str) -> Input {
    input.lines().map(get_numbers).collect()
}

fn part1(input: &str) -> Output {
    let equations = parse_input(input);
    solve(equations, &[Operation::Divide, Operation::Sub])
}

fn part2(input: &str) -> Output {
    let equations = parse_input(input);
    solve(
        equations,
        &[Operation::Divide, Operation::Slough, Operation::Sub],
    )
}

#[test]
fn default() {
    let input = get_input(24, 7).unwrap();
    assert_eq!(945512582195, part1(&input));
    assert_eq!(271691107779347, part2(&input));
}

// Input parsed (29μs)
// 1. 945512582195 (603μs)
// 2. 271691107779347 (1ms)
// Total: 2ms
