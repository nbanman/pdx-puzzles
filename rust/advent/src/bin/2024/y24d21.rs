use advent::utilities::get_input::get_input;
use std::collections::HashMap;
use itertools::Itertools;
use utilities::structs::coord::Coord2;
use utilities::structs::stopwatch::{ReportDuration, Stopwatch};

type Input<'a> = &'a str;
type Pos = Coord2;
type Output = u64;
type Actions = HashMap<char, HashMap<char, Vec<char>>>;
type Cache = HashMap<State, Output>;

fn main() {
    let mut stopwatch = Stopwatch::new();
    stopwatch.start();
    let input = get_input(24, 21).unwrap();
    println!("Input parsed ({})", stopwatch.lap().report());
    println!("1. {} ({})", solve(&input, 2), stopwatch.lap().report());
    println!("2. {} ({})", solve(&input, 25), stopwatch.lap().report());
    println!("Total: {}", stopwatch.stop().report());
}

#[derive(PartialEq, Eq, Hash, Copy, Clone, Debug)]
struct State {
    from: char,
    to: char,
    level: usize,
}

fn solve(input: Input, robots: usize) -> Output {
    let actions = get_actions();
    let mut cache = get_cache(&actions);
    input.lines()
        .map(|code| {
            let n = code[..code.len() - 1].parse::<u64>().unwrap();
            let presses = key_presses(code, robots, &actions, &mut cache);
            n * presses
        })
        .sum()
}

fn get_actions() -> Actions {
    let mut actions = HashMap::new();
    let num_pad = "789456123#0A";
    for (a_index, a) in num_pad.chars().enumerate() {
        if a == '#' { continue; }
        let a_pos = Pos::new2d(a_index as i64 % 3, a_index as i64 / 3);
        for (b_index, b) in num_pad.chars().enumerate() {
            if b == '#' { continue; }
            let b_pos = Pos::new2d(b_index as i64 % 3, b_index as i64 / 3);
            let delta = b_pos - a_pos;
            let y_sign = delta.y().signum();
            let y_line = if y_sign < 0 {
                vec!['^'; delta.y().abs() as usize]
            } else if y_sign > 0 {
                vec!['v'; delta.y().abs() as usize]
            } else {
                Vec::new()
            };
            let x_sign = delta.x().signum();
            let x_line = if x_sign < 0 {
                vec!['<'; delta.x().abs() as usize]
            } else if x_sign > 0 {
                vec!['>'; delta.x().abs() as usize]
            } else {
                Vec::new()
            };
            let moves = if x_sign < 0 {
                if a_pos.y() == 3 && b_pos.x() == 0 {
                    y_line.into_iter().chain(x_line.into_iter())
                } else {
                    x_line.into_iter().chain(y_line.into_iter())
                }
            } else if a_pos.x() == 0 && b_pos.y() == 3 {
                x_line.into_iter().chain(y_line.into_iter())
            } else {
                y_line.into_iter().chain(x_line.into_iter())
            };
            actions.entry(a)
                .or_insert(HashMap::new())
                .insert(b, moves.chain(std::iter::once('A')).collect());
        }
    }

    let dir_pad = "#^A<v>";
    for (a_index, a) in dir_pad.chars().enumerate() {
        if a == '#' { continue; }
        let a_pos = Pos::new2d(a_index as i64 % 3, a_index as i64 / 3);
        for (b_index, b) in dir_pad.chars().enumerate() {
            if b == '#' { continue; }
            let b_pos = Pos::new2d(b_index as i64 % 3, b_index as i64 / 3);
            let delta = b_pos - a_pos;
            let y_sign = delta.y().signum();
            let y_line = if y_sign < 0 {
                vec!['^'; delta.y().abs() as usize]
            } else if y_sign > 0 {
                vec!['v'; delta.y().abs() as usize]
            } else {
                Vec::new()
            };
            let x_sign = delta.x().signum();
            let x_line = if x_sign < 0 {
                vec!['<'; delta.x().abs() as usize]
            } else if x_sign > 0 {
                vec!['>'; delta.x().abs() as usize]
            } else {
                Vec::new()
            };
            let moves = if x_sign < 0 {
                if a_pos.y() == 0 && b_pos.x() == 0 {
                    y_line.into_iter().chain(x_line.into_iter())
                } else {
                    x_line.into_iter().chain(y_line.into_iter())
                }
            } else if a_pos.x() == 0 && b_pos.y() == 0 {
                x_line.into_iter().chain(y_line.into_iter())
            } else {
                y_line.into_iter().chain(x_line.into_iter())
            };
            actions.entry(a)
                .or_insert(HashMap::new())
                .insert(b, moves.chain(std::iter::once('A')).collect());
        }
    }
    actions
}

fn get_cache(actions: &Actions) -> Cache {
    let mut cache = HashMap::new();
    for (&a, b_actions) in actions.iter() {
        for (&b, actions) in b_actions.iter() {
            cache.insert(
                State { from: a, to: b, level: 0 },
                actions.len() as u64,
            );
        }
    }
    cache
}

fn key_presses(code: Input, robots: usize, actions: &Actions, cache: &mut Cache) -> u64 {
    std::iter::once('A').chain(code.chars())
        .tuple_windows()
        .map(|(a, b)| search(
            State { from: a, to: b, level: robots },
            actions,
            cache,
        ))
        .sum()
}

fn search(state: State, actions: &Actions, cache: &mut Cache) -> Output {
    if let Some(presses) = cache.get(&state) {
        return *presses;
    };
    let State { from, to, level } = state;
    let next = std::iter::once('A')
        .chain(actions[&from][&to].iter().copied())
        .tuple_windows::<(char, char)>();
    let presses: Output = next
        .map(|(a, b)| search(
            State { from: a, to: b, level: level - 1 },
            actions,
            cache,
        ))
        .sum();
    cache.insert(state, presses);
    presses
}

#[test]
fn default() {
    let input = get_input(24, 21).unwrap();
    assert_eq!(169390, solve(&input, 2));
    assert_eq!(210686850124870, solve(&input, 25));
}

#[test]
fn examples() {
    let inputs = [r"029A
980A
179A
456A
379A
", ];
    assert_eq!(126384, solve(inputs[0], 2));
}

// Input parsed (13μs)
// 1. 169390 (61μs)
// 2. 210686850124870 (107μs)
// Total: 184μs