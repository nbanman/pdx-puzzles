use std::cmp::{max, min};
use std::iter;
use itertools::Itertools;
use everybody_codes::utilities::inputs::get_story_inputs;
use utilities::structs::coord::Coord2U;
use utilities::structs::grid::{Grid, Grid2};
use utilities::structs::stopwatch::{ReportDuration, Stopwatch};

type Input<'a> = &'a str;
type Machine = Grid2<Pattern>;
type Token = Vec<Direction>;

fn main() {
    let mut stopwatch = Stopwatch::new();
    stopwatch.start();
    let (input1, input2, input3) = get_story_inputs(25, 2, 1);
    println!("Input parsed ({})", stopwatch.lap().report());
    println!("1. {} ({})", part1(&input1), stopwatch.lap().report());
    println!("2. {} ({})", part2(&input2), stopwatch.lap().report());
    println!("3. {} ({})", part3(&input3), stopwatch.lap().report());
    println!("Total: {}", stopwatch.stop().report());
}

fn parse(input: Input) -> (Machine, Vec<Token>) {
    let (machine, tokens) = input.split_once("\n\n").unwrap();

    let machine = Grid2::try_from(machine).unwrap();
    let width = machine.width();
    let machine: Vec<_> = machine.into_iter()
        .map(|c| if c == '*' { Pattern::Nail } else { Pattern::Space })
        .collect();
    let machine = Grid::new2d(machine, width).unwrap();

    let tokens = tokens
        .lines()
        .map(|line| {
            line.as_bytes().into_iter()
                .map(|&b| if b == b'L' { Direction::Left } else { Direction::Right })
                .collect()
        })
        .collect();
    (machine, tokens)
}
fn part1(input: Input) -> usize {
    let (machine, tokens) = parse(input);
    tokens.into_iter().enumerate()
        .map(|(n, token)| token_score(n, &token, &machine))
        .sum()
}

fn part2(input: Input) -> usize {
    let (machine, tokens) = parse(input);
    tokens.into_iter()
        .map(|token| {
            (0..(machine.width() + 1) / 2)
                .map(|n| token_score(n, &token, &machine))
                .max()
                .expect("Machine width will always be positive number")
        })
        .sum()
}

fn part3(input: Input) -> String {
    let (machine, tokens) = parse(input);
    let high_scores: Vec<Vec<(usize, usize)>> = tokens.iter()
        .map(|token| {
            (0..(machine.width() + 1) / 2)
                .map(|n| (n, token_score(n, &token, &machine)))
                .sorted_unstable_by(|a, b| b.1.cmp(&a.1))
                .collect()
        })
        .collect();
    let high_heuristic = get_heuristic(&high_scores);
    let mut high_taken = vec![false; (machine.width() + 1) / 2];

    let high_score = high_dfs(&high_scores, &mut high_taken, &high_heuristic, 0, 0, 0);

    let low_scores: Vec<Vec<(usize, usize)>> = high_scores.into_iter()
        .map(|scores| scores.into_iter().rev().collect())
        .collect();
    let low_heuristic: Vec<_> = get_heuristic(&low_scores);
    let mut low_taken = vec![false; (machine.width() + 1) / 2];

    let low_score = low_dfs(&low_scores, &mut low_taken, &low_heuristic, 0, 0, usize::MAX);
    format!("{} {}", low_score, high_score)
}

fn get_heuristic(high_scores: &Vec<Vec<(usize, usize)>>) -> Vec<usize> {
    let high_heuristic: Vec<_> = high_scores.iter()
        .dropping(1)
        .rev()
        .scan(0, |state, token_scores| {
            *state = *state + token_scores[0].1;
            Some(*state)
        })
        .collect::<Vec<_>>()
        .into_iter()
        .rev()
        .chain(iter::once(0))
        .collect();
    high_heuristic
}

fn high_dfs(
    scores: &Vec<Vec<(usize, usize)>>,
    taken: &mut Vec<bool>,
    heuristic: &Vec<usize>,
    token: usize,
    current: usize,
    high_score: usize,
) -> usize {
    let mut high_score = high_score;
    if token == scores.len() {
        return current;
    }
    for &(slot, score) in scores[token].iter() {
        if taken[slot] { continue; }
        if current + score + heuristic[token] <= high_score { continue; }

        taken[slot] = true;
        high_score = max(
            high_dfs(scores, taken, heuristic, token + 1, current + score, high_score),
            high_score
        );
        taken[slot] = false;
    }

    high_score
}

fn low_dfs(
    scores: &Vec<Vec<(usize, usize)>>,
    taken: &mut Vec<bool>,
    heuristic: &Vec<usize>,
    token: usize,
    current: usize,
    low_score: usize,
) -> usize {
    let mut low_score = low_score;
    if token == scores.len() {
        return current;
    }
    for &(slot, score) in scores[token].iter() {
        if taken[slot] { continue; }
        if current + score + heuristic[token] >= low_score { continue; }

        taken[slot] = true;
        low_score = min(
            low_dfs(scores, taken, heuristic, token + 1, current + score, low_score),
            low_score
        );
        taken[slot] = false;
    }

    low_score
}

fn token_score(slot: usize, token: &Token, machine: &Machine) -> usize {
    let start = slot * 2;
    let (end, _) = (0..machine.height())
        .fold((start, 0), |(x, hits), y| {
            let pos = machine[Coord2U::new2d(x, y)];
            match pos {
                Pattern::Nail => {
                    let dir = token[hits];
                    let new_x = match dir {
                        Direction::Left => x.checked_sub(1).unwrap_or(x + 1),
                        Direction::Right => {
                            let new_x = x + 1;
                            if new_x < machine.width() { new_x } else { x - 1 }
                        }
                    };
                    (new_x, hits + 1)
                },
                Pattern::Space => (x, hits),
            }
        });
    (end + 2).checked_sub(start / 2 + 1).unwrap_or_default()
}

#[derive(Copy, Clone, Debug)]
enum Pattern {
    Nail,
    Space,
}

#[derive(Copy, Clone, Debug)]
enum Direction {
    Left,
    Right,
}

#[test]
fn default() {
    let (input1, input2, input3) = get_story_inputs(25, 2, 1);
    assert_eq!(43, part1(&input1));
    assert_eq!(1143, part2(&input2));
    assert_eq!("38 112".to_string(), part3(&input3));
}

#[test]
fn examples() {
    let inputs = [r"*.*.*.*.*.*.*.*.*
.*.*.*.*.*.*.*.*.
*.*.*...*.*...*..
.*.*.*.*.*...*.*.
*.*.....*...*.*.*
.*.*.*.*.*.*.*.*.
*...*...*.*.*.*.*
.*.*.*.*.*.*.*.*.
*.*.*...*.*.*.*.*
.*...*...*.*.*.*.
*.*.*.*.*.*.*.*.*
.*.*.*.*.*.*.*.*.

RRRLRLRRRRRL
LLLLRLRRRRRR
RLLLLLRLRLRL
LRLLLRRRLRLR
LLRLLRLLLRRL
LRLRLLLRRRRL
LRLLLLLLRLLL
RRLLLRLLRLRR
RLLLLLRLLLRL", r"*.*.*.*.*.*.*.*.*.*.*.*.*
.*.*.*.*.*.*.*.*.*.*.*.*.
..*.*.*.*...*.*...*.*.*..
.*...*.*.*.*.*.*.....*.*.
*.*...*.*.*.*.*.*...*.*.*
.*.*.*.*.*.*.*.*.......*.
*.*.*.*.*.*.*.*.*.*...*..
.*.*.*.*.*.*.*.*.....*.*.
*.*...*.*.*.*.*.*.*.*....
.*.*.*.*.*.*.*.*.*.*.*.*.
*.*.*.*.*.*.*.*.*.*.*.*.*
.*.*.*.*.*.*.*.*.*...*.*.
*.*.*.*.*.*.*.*.*...*.*.*
.*.*.*.*.*.*.*.*.....*.*.
*.*.*.*.*.*.*.*...*...*.*
.*.*.*.*.*.*.*.*.*.*.*.*.
*.*.*...*.*.*.*.*.*.*.*.*
.*...*.*.*.*...*.*.*...*.
*.*.*.*.*.*.*.*.*.*.*.*.*
.*.*.*.*.*.*.*.*.*.*.*.*.

RRRLLRRRLLRLRRLLLRLR
RRRRRRRRRRLRRRRRLLRR
LLLLLLLLRLRRLLRRLRLL
RRRLLRRRLLRLLRLLLRRL
RLRLLLRRLRRRLRRLRRRL
LLLLLLLLRLLRRLLRLLLL
LRLLRRLRLLLLLLLRLRRL
LRLLRRLLLRRRRRLRRLRR
LRLLRRLRLLRLRRLLLRLL
RLLRRRRLRLRLRLRLLRRL", r"*.*.*.*.*.*.*.*.*
.*.*.*.*.*.*.*.*.
*.*.*...*.*...*..
.*.*.*.*.*...*.*.
*.*.....*...*.*.*
.*.*.*.*.*.*.*.*.
*...*...*.*.*.*.*
.*.*.*.*.*.*.*.*.
*.*.*...*.*.*.*.*
.*...*...*.*.*.*.
*.*.*.*.*.*.*.*.*
.*.*.*.*.*.*.*.*.

RRRLRLRRRRRL
LLLLRLRRRRRR
RLLLLLRLRLRL
LRLLLRRRLRLR
LLRLLRLLLRRL
LRLRLLLRRRRL", r"*.*.*.*.*.*.*.*.*.*.*.*.*
.*.*.*.*.*.*.*.*.*.*.*.*.
..*.*.*.*...*.*...*.*.*..
.*...*.*.*.*.*.*.....*.*.
*.*...*.*.*.*.*.*...*.*.*
.*.*.*.*.*.*.*.*.......*.
*.*.*.*.*.*.*.*.*.*...*..
.*.*.*.*.*.*.*.*.....*.*.
*.*...*.*.*.*.*.*.*.*....
.*.*.*.*.*.*.*.*.*.*.*.*.
*.*.*.*.*.*.*.*.*.*.*.*.*
.*.*.*.*.*.*.*.*.*...*.*.
*.*.*.*.*.*.*.*.*...*.*.*
.*.*.*.*.*.*.*.*.....*.*.
*.*.*.*.*.*.*.*...*...*.*
.*.*.*.*.*.*.*.*.*.*.*.*.
*.*.*...*.*.*.*.*.*.*.*.*
.*...*.*.*.*...*.*.*...*.
*.*.*.*.*.*.*.*.*.*.*.*.*
.*.*.*.*.*.*.*.*.*.*.*.*.

RRRLLRRRLLRLRRLLLRLR
RRRRRRRRRRLRRRRRLLRR
LLLLLLLLRLRRLLRRLRLL
RRRLLRRRLLRLLRLLLRRL
RLRLLLRRLRRRLRRLRRRL
LLLLLLLLRLLRRLLRLLLL", r"*.*.*.*.*.*.*.*.*.*.*.*.*.*.*.*.*.*.*.*
.*.*.*.*.*.*.*.*.*.*.*.*.*.*.*.*.*.*.*.
..*.*.*.*.*.*.........*.*.*.*.....*.*.*
.*.*...*.*.*.*.*.*.*.*.*.*.*...*.*.*.*.
*.*.*.*...*.*.*.*.*.....*.*.*.*...*.*..
.*...*.*...*.*.*.*.*.*.*.....*.*.*.*.*.
*.*.*.*.*.....*.*.*.*.*.*.*.*.*.*.*.*.*
.*.*.*.*.*.*...*.*.*.*.....*.*.*.*...*.
*.*...*.*.*.*.*.*.*.*...*.*.*...*.*.*.*
.*...*.*.*.*.*.*.*.*...*.*.*.*.*.*.*.*.
*.*.*.*.*.*...*.....*.*...*...*.*.*.*.*
.*...*.*.*.*.*...*.*.*.*.*...*.*...*.*.
*.*.*.*.*...*.*.*.*.*.*.*.*...*.*.*.*.*
.*.*.*.*.*.*.*.*...*.*.*.*.*.*.*.*.*.*.
....*.*.*.*...*.*.*.*.*.*.*...*.*.*...*
.*.*.*...*.*.*.*.*...*.*.*.*.*.*.*.*...
*.*.*.*.*.*.*.....*...*...*.*.*.*.*.*.*
.*.*...*.....*.*.*.*.*.*.*...*.*.*.*.*.
*.*.*.*.*.*.*.*.*.*.*.*.*.*.*.*.*.*.*.*
.*.*.*.*.*.*.*.*.*.*.*.*.*.*.*.*.*.*.*.

RRRRLLRRLLLLLLLRLLRL
RRRRRRRLRRLRRLRRRLRR
RRRLLRRRRRLRRRRRLRRR
LLLLRRLLRRLLLLLRRLLL
LRRRRLRRLRLLRLLRRLRR
RRRRRRRRLRRRRLLRRRLR"];
    assert_eq!(26, part1(inputs[0]));
    assert_eq!(115, part2(inputs[1]));
    assert_eq!("13 43".to_string(), part3(inputs[2]));
    assert_eq!("25 66".to_string(), part3(inputs[3]));
    assert_eq!("39 122".to_string(), part3(inputs[4]));
}

// Input parsed (28μs)
// 1. 43 (28μs)
// 2. 1143 (1ms)
// 3. 38 112 (166μs)
// Total: 1ms