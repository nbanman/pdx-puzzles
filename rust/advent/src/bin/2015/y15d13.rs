use std::cmp::max;

use advent::utilities::get_input::get_input;
use itertools::Itertools;
use lazy_regex::regex;
use utilities::structs::stopwatch::{ReportDuration, Stopwatch};

type Input = Vec<Vec<i32>>;
type Output = i32;

fn main() {
    let mut stopwatch = Stopwatch::new();
    stopwatch.start();
    let input = get_input(15, 13).unwrap();
    let input = parse_input(&input);
    println!("Input parsed ({})", stopwatch.lap().report());
    println!("1. {} ({})", part1(&input), stopwatch.lap().report());
    println!("2. {} ({})", part2(&input), stopwatch.lap().report());
    println!("Total: {}", stopwatch.stop().report());
}

fn parse_input(input: &str) -> Input {
    let mut people: Vec<&str> = Vec::new();
    let mut arrangements: Input = Vec::new();
    let rx = regex!(r"(\w+) would (gain|lose) (\d+) happiness units by sitting next to (\w+).");
    for cap in rx.captures_iter(input) {
        let (p1, gain_or_lose, units, p2) = (
            cap.get(1).unwrap().as_str(),
            cap.get(2).unwrap().as_str(),
            cap.get(3).unwrap().as_str(),
            cap.get(4).unwrap().as_str(),
        );
        let p1 = person_index(p1, &mut people);
        let p2 = person_index(p2, &mut people);
        let happiness = units.parse::<i32>().unwrap()
            * if gain_or_lose == "gain" { 1 } else { -1 };
        insert_happiness(&mut arrangements, happiness, p1, p2);
    }
    arrangements
}

fn person_index<'a>(p: &'a str, people: &mut Vec<&'a str>) -> usize {
    people
        .iter()
        .position(|&person| person == p)
        .unwrap_or_else(|| {
            people.push(p);
            people.len() - 1
        })
}

fn insert_happiness(arrangements: &mut Vec<Vec<i32>>, happiness: i32, a: usize, b: usize) {
    while arrangements.len() < max(a, b) + 1 {
        arrangements.push(Vec::new());
    }
    let a_list = arrangements.get_mut(a).unwrap();
    while a_list.len() < b + 1 {
        a_list.push(0);
    }
    a_list[b] = happiness;
}

fn solve(arrangements: &Input, people: usize) -> Output {
    (1..people).permutations(people - 1)
        .map(|perm| {
            let start = perm[0];
            perm.into_iter()
                .chain(std::iter::once(0))
                .tuple_windows()
                .chain(std::iter::once((0, start)))
                .map(|(left, right)| {
                    let left_to_right = arrangements.get(left)
                        .map(|left_happy| left_happy.get(right).unwrap_or(&0))
                        .unwrap_or(&0);
                    let right_to_left = arrangements.get(right)
                        .map(|right_happy| right_happy.get(left).unwrap_or(&0))
                        .unwrap_or(&0);
                    left_to_right + right_to_left
                })
                .sum()
        })
        .max()
        .unwrap()
}

fn part1(arrangements: &Input) -> Output {
    solve(arrangements, arrangements.len())
}

fn part2(arrangements: &Input) -> Output {
    solve(arrangements, arrangements.len() + 1)
}

#[test]
fn default() {
    let input = get_input(15, 13).unwrap();
    let input = parse_input(&input);
    assert_eq!(664, part1(&input));
    assert_eq!(640, part2(&input));
}

// Input parsed (2ms)
// 1. 664 (346μs)
// 2. 640 (2ms)
// Total: 5ms