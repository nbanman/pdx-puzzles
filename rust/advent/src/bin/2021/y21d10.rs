use std::{collections::VecDeque, iter::Rev};

use advent::utilities::get_input::get_input;
use itertools::Itertools;
use rustc_hash::FxHashMap;
use utilities::structs::stopwatch::{ReportDuration, Stopwatch};

type Input<'a> = &'a str;
type Output = usize;

fn main() {
    let mut stopwatch = Stopwatch::new();
    stopwatch.start();
    let input = get_input(21, 10).unwrap();
    let counterparts = get_counterparts();

    println!("Input parsed ({})", stopwatch.lap().report());
    println!("1. {} ({})", part1(&input, &counterparts), stopwatch.lap().report());
    println!("2. {} ({})", part2(&input, &counterparts), stopwatch.lap().report());
    println!("Total: {}", stopwatch.stop().report());
}

fn get_counterparts() -> FxHashMap<char, char> {
    let counterparts = [
        ('(', ')'),
        ('[', ']'),
        ('{', '}'),
        ('<', '>'),
    ];
    counterparts
        .into_iter()
        .collect()
}

fn score<F, G>(
    line: &str,
    counterparts: &FxHashMap<char, char>,
    on_corrupt: F,
    on_finish: G,
) -> Option<usize>
where 
    F: Fn(char) -> Option<usize>,
    G: Fn(Rev<std::collections::vec_deque::IntoIter<char>>) -> Option<usize>,
{
    let mut stack = VecDeque::new();
    for candidate in line.chars() {
        if let Some(counterpart) = counterparts.get(&candidate) {
            stack.push_back(*counterpart);
        } else {
            if Some(candidate) != stack.pop_back() {
                return on_corrupt(candidate);
            } 
        }
    }
    on_finish(stack.into_iter().rev())
}

fn part1(input: Input, counterparts: &FxHashMap<char, char>) -> Output {
    let syntax_error_score = [(')', 3), (']', 57), ('}', 1197), ('>', 25137)];
    let syntax_error_score: FxHashMap<char, usize> = syntax_error_score.into_iter().collect();
   
    input.lines()
        .filter_map(|line| {
            score(
                line,
                counterparts,
                |c| syntax_error_score.get(&c).copied(),
                |_| None,
            )
        }) 
        .sum()
}

fn part2(input: Input, counterparts: &FxHashMap<char, char>) -> Output {
    let point_value = [
        (')', 1), (']', 2), ('}', 3), ('>', 4),
    ];
    let point_value: FxHashMap<char, usize> = point_value.into_iter().collect();
    
    let stack_score = |iter: Rev<std::collections::vec_deque::IntoIter<char>>| {
        Some(iter.fold(0, |acc, c| acc * 5 + point_value[&c]))
    };
    
    let scores: Vec<usize> = input.lines()
        .filter_map(|line| {
            score(
                line, 
                counterparts, 
                |_| None, 
                |c| stack_score(c)
            )
        })
        .sorted()
        .collect();
    scores[scores.len() / 2]
}

#[test]
fn default() {
    let input = get_input(21, 10).unwrap();
    let counterparts = get_counterparts();
    assert_eq!(167379, part1(&input, &counterparts));
    assert_eq!(2776842859, part2(&input, &counterparts));
}

#[test]
fn example() {
    let input = r"[({(<(())[]>[[{[]{<()<>>
[(()[<>])]({[<{<<[]>>(
{([(<{}[<>[]}>{[]{[(<()>
(((({<>}<{<{<>}{[]{[]{}
[[<[([]))<([[{}[[()]]]
[{[{({}]{}}([{[{{{}}([]
{<[[]]>}<{[{[{[]{()[[[]
[<(<(<(<{}))><([]([]()
<{([([[(<>()){}]>(<<{{
<{([{{}}[<[[[<>{}]]]>[]]
";
    let counterparts = get_counterparts();
    assert_eq!(26397, part1(&input, &counterparts));
    assert_eq!(288957, part2(&input, &counterparts));
}

// Input parsed (27μs)
// 1. 167379 (39μs)
// 2. 2776842859 (39μs)
// Total: 109μs