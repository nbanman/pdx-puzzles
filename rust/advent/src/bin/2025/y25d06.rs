use advent::utilities::get_input::get_input;
use itertools::Itertools;
use utilities::{
    parsing::get_numbers::ContainsNumbers,
    structs::{
        grid::{Grid2, GridIterator},
        stopwatch::{ReportDuration, Stopwatch},
    },
};

type Input<'a> = (&'a str, &'a str);
type Output = u64;

fn main() {
    let mut stopwatch = Stopwatch::new();
    stopwatch.start();
    let input = get_input(25, 6).unwrap();
    let input = parse_input(&input);
    println!("Input parsed ({})", stopwatch.lap().report());
    println!("1. {} ({})", part1(input), stopwatch.lap().report());
    println!("2. {} ({})", part2(input), stopwatch.lap().report());
    println!("Total: {}", stopwatch.stop().report());
}

fn parse_input(input: &'_ str) -> Input<'_> {
    let rp = input.as_bytes().iter().rposition(|&b| b == b'\n').unwrap() + 1;
    let (nums, ops) = input.split_at(rp);
    (nums, ops)
}

fn part1(input: Input) -> Output {
    let (nums, ops) = input;
    let ops: Vec<char> = ops.chars().filter(|c| !c.is_ascii_whitespace()).collect();
    nums.get_numbers::<Output>()
        .try_collect_grid(ops.len())
        .unwrap()
        .columns()
        .zip(ops)
        .map(|(col, op)| {
            if op == '*' {
                col.iter().fold(1, |acc, &&i| acc * i)
            } else {
                col.iter().fold(0, |acc, &&i| acc + i)
            }
        })
        .sum()
}

fn part2(input: Input) -> Output {
    let (nums, ops) = input;
    let mut sum = 0;
    let mut col_val = 0;
    let mut operator = ' ';
    let grid = Grid2::try_from(nums).unwrap();
    for (col, op) in grid.columns().zip(ops.chars()) {
        match op {
            '*' => {
                sum += col_val;
                col_val = 1;
                operator = '*';
            }
            '+' => {
                sum += col_val;
                col_val = 0;
                operator = '+';
            }
            ' ' => {}
            op => panic!("'{}' not recognized!", op),
        }
        let digit = col.iter()
            .filter(|c| c.is_ascii_digit())
            .join("")
            .parse::<Output>();

        if let Ok(digit) = digit && digit != 0 {
            if operator == '*' {
                col_val *= digit;
            } else if operator == '+' {
                col_val += digit;
            }
        }
    }
    sum + col_val
}

#[test]
fn default() {
    let input = get_input(25, 6).unwrap();
    let input = parse_input(&input);
    assert_eq!(4387670995909, part1(input));
    assert_eq!(9625320374409, part2(input));
}

#[test]
fn test1() {
    let input = r"123 328  51 64 
 45 64  387 23 
  6 98  215 314
*   +   *   +  ";
    let input = parse_input(input);
    assert_eq!(4277556, part1(input));
}

#[test]
fn test2() {
    let input = r"123 328  51 64 
 45 64  387 23 
  6 98  215 314
*   +   *   +  ";
    let input = parse_input(input);
    assert_eq!(3263827, part2(input));
}

// Input parsed (49μs)
// 1. 4387670995909 (135μs)
// 2. 9625320374409 (263μs)
// Total: 454μs
