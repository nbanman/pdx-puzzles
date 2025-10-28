use std::ops::Mul;
use std::collections::VecDeque;

use advent::utilities::get_input::get_input;
use utilities::structs::stopwatch::{ReportDuration, Stopwatch};

type Input = Vec<VecDeque<Expression>>;
type Output = usize;

#[derive(Debug, Clone)]
enum Expression {
    Value(usize),
    Plus,
    Times,
    Parens(Box<VecDeque<Expression>>),
}

impl Expression {
    fn evaluate<F>(self, mut eval: F) -> Option<Output>
    where F: FnMut(VecDeque<Expression>) -> Output,
    {
        match self {
            Expression::Value(v) => Some(v),
            Expression::Parens(expressions) => Some(eval(*expressions)),
            _ => None,
        }
    }

    fn parse(s: &str) -> VecDeque<Self> {
        let mut parser = 0;
        let bytes = s.as_bytes();
        let mut subexpressions = VecDeque::new();
        while parser < bytes.len() {
            if let Some(subexpression) = Self::parse_subexpression(bytes, &mut parser) {
                subexpressions.push_back(subexpression);
            }
        }
        subexpressions
    }
    
    fn parse_subexpression(bytes: &[u8], parser: &mut usize) -> Option<Self> {
        match bytes.get(*parser)? {
            b'+' => {
                *parser += 1;
                Some(Self::Plus)
            },
            b'*' => {
                *parser += 1;
                Some(Self::Times)
            },
            b'(' => {
                *parser += 1;
                let mut components = VecDeque::new();
                while let Some(component) = Self::parse_subexpression(bytes, parser) {
                    components.push_back(component);
                }
                Some(Self::Parens(Box::new(components)))
            },
            &b if (b as char).is_numeric() => {
                *parser += 1;
                Some(Self::Value(b as usize - 48))
            },
            b')' => {
                *parser += 1;
                None
            },
            _ => unreachable!(),
        }
    }
}

fn main() {
    let mut stopwatch = Stopwatch::new();
    stopwatch.start();
    let input = get_input(20, 18).unwrap();
    let input = parse_input(&input);
    println!("Input parsed ({})", stopwatch.lap().report());
    println!("1. {} ({})", part1(input.clone()), stopwatch.lap().report());
    println!("2. {} ({})", part2(input), stopwatch.lap().report());
    println!("Total: {}", stopwatch.stop().report());
}

fn parse_input(input: &str) -> Input {
    input.replace(' ', "")
        .lines()
        .map(|line| Expression::parse(line))
        .collect()
}

fn eval_1(mut expression: VecDeque<Expression>) -> Output {
    let mut left = expression.pop_front().unwrap().evaluate(eval_1).unwrap();
    while let (Some(operator), Some(right)) =
        (expression.pop_front(), expression.pop_front())
    {
        let right = right.evaluate(eval_1).unwrap();
        left = match operator {
            Expression::Plus => left + right,
            Expression::Times => left * right,
            _ => panic!("Operator must be Plus or Times"),
        };
    }
    left
}

fn eval_2(mut expression: VecDeque<Expression>) -> Output {
    let mut values: Vec<usize> = Vec::new();
    let mut left = expression.pop_front().unwrap().evaluate(eval_2).unwrap();
    while let (Some(operator), Some(right)) =
        (expression.pop_front(), expression.pop_front())
    {
        let right = right.evaluate(eval_2).unwrap();
        match operator {
            Expression::Plus => { left = left + right; },
            Expression::Times => {
                values.push(left);
                left = right;
            },
            _ => panic!("operator must be Plus or Times"),
        }
    }
    values.push(left);
    values.into_iter().reduce(usize::mul).unwrap()
}


fn part1(expressions: Input) -> Output {
    expressions.into_iter().map(eval_1).sum()
}

fn part2(expressions: Input) -> Output {
    expressions.into_iter().map(eval_2).sum()
}

#[test]
fn default() {
    let input = get_input(20, 18).unwrap();
    let input = parse_input(&input);
    assert_eq!(510_009_915_468, part1(input.clone()));
    assert_eq!(321_176_691_637_769, part2(input));
}

// Input parsed (369μs)
// 1. 510009915468 (227μs)
// 2. 321176691637769 (80μs)
// Total: 679μs
