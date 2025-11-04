use std::cmp::min;

use itertools::Itertools;
use everybody_codes::utilities::inputs::get_event_inputs;
use utilities::structs::stopwatch::{ReportDuration, Stopwatch};

type Input<'a> = &'a str;

fn main() {
    let mut stopwatch = Stopwatch::new();
    stopwatch.start();
    let (input1, input2, input3) = get_event_inputs(25, 1);
    println!("Input parsed ({})", stopwatch.lap().report());
    println!("1. {} ({})", part1(&input1), stopwatch.lap().report());
    println!("2. {} ({})", part2(&input2), stopwatch.lap().report());
    println!("3. {} ({})", part3(&input3), stopwatch.lap().report());
    println!("Total: {}", stopwatch.stop().report());
}

fn parse(input: Input<'_>) -> (Vec<&'_ str>, Vec<(&'_ str, i64)>) {
    let (names, commands) = input.trim_end().split("\n\n").collect_tuple().unwrap();
    let names = names.split(',').collect_vec();
    let commands = commands.split(',')
        .map(|command| {
            let (dir, num) = command.split_at(1);
            let num: i64 = num.parse().unwrap();
            (dir, num)
        })
        .collect();
    (names, commands)
}

fn part1(input: Input<'_>) -> &'_ str {
    let (names, commands) = parse(input);
    let maximum = names.len() - 1;
    let mut index: usize = 0;
    for (dir, num) in commands {
        let num = num as usize;
        match dir {
            "L" => { index = index.checked_sub(num).unwrap_or_default(); }
            "R" => { index = min(maximum, index + num); }
            _ => unreachable!(),
        }
    }
    names[index]
}

fn part2(input: Input<'_>) -> &'_ str {
    let (names, commands) = parse(input);
    let mut index: i64 = 0;
    for (dir, num) in commands {
        match dir {
            "L" => { index -= num; }
            "R" => { index += num; }
            _ => unreachable!(),
        }
    }
    let index = index.rem_euclid(names.len() as i64) as usize;
    names[index]
}

fn part3(input: Input<'_>) -> &'_ str {
    let (mut names, commands) = parse(input);
    for (dir, num) in commands {
        let index = match dir {
            "L" => -num,
            "R" => num,
            _ => unreachable!(),
        };
        let index = index.rem_euclid(names.len() as i64) as usize;
        names.swap(0, index);
    }
    names[0]
}

#[test]
fn default() {
    let (input1, input2, input3) = get_event_inputs(25, 1);
    assert_eq!("Shaelgarath", part1(&input1));
    assert_eq!("Quarndin", part2(&input2));
    assert_eq!("Gorathmal", part3(&input3));
}

// Input parsed (29μs)
// 1. Shaelgarath (5μs)
// 2. Quarndin (3μs)
// 3. Gorathmal (3μs)
// Total: 44μs
