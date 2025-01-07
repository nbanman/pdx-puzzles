use std::collections::HashMap;

use advent::utilities::get_input::get_input;
use lazy_regex::regex;
use utilities::structs::stopwatch::{ReportDuration, Stopwatch};

type Input = Vec<HashMap<String, usize>>;
type Output = usize;

fn main() {
    let mut stopwatch = Stopwatch::new();
    stopwatch.start();
    let input = get_input(23, 2).unwrap();
    let input = parse_input(&input);
    println!("Input parsed ({})", stopwatch.lap().report());
    println!("1. {} ({})", part1(&input), stopwatch.lap().report());
    println!("2. {} ({})", part2(&input), stopwatch.lap().report());
    println!("Total: {}", stopwatch.stop().report());
}

fn parse_input(input: &str) -> Input {
    let pattern = regex!(r"(\d+) (\w+)");

    input
        .lines()
        .map(|line| {
            let mut game: HashMap<String, usize> = HashMap::new();
            pattern.captures_iter(line).for_each(|m| {
                let amt = m.get(1).map_or(0, |amt| {
                    amt.as_str().parse::<usize>().unwrap()
                });
                let color = m.get(2).map_or("unknown", |color| {
                    color.as_str()
                });
                match game.get(color) {
                    Some(n) => if n < &amt {
                        game.insert(color.to_string(), amt);
                    },
                    None => {
                        game.insert(color.to_string(), amt);
                    }
                }
                if let Some(n) = game.get(color) {
                    if n < &amt {
                        game.insert(color.to_string(), amt);
                    }
                }
            });
            game
        }).collect()
}

fn part1(input: &Input) -> Output {
    let mut standard_bag: HashMap<String, usize> = HashMap::new();
    standard_bag.insert(String::from("red"), 12);
    standard_bag.insert(String::from("green"), 13);
    standard_bag.insert(String::from("blue"), 14);

    input
        .iter()
        .enumerate()
        .filter(|(_, game)| {
            game.iter().all(|(color, amt)| {
                standard_bag.get(color).unwrap_or(&0) >= amt
            })
        }).map(|(index, _)| index + 1)
        .sum()
}

fn part2(input: &Input) -> Output {
    input
        .iter()
        .map(|game| {
            game.values().product::<usize>()
        })
        .sum()
}

#[test]
fn default() {
    let input = get_input(23, 2).unwrap();
    let input = parse_input(&input);
    assert_eq!(2377, part1(&input));
    assert_eq!(71220, part2(&input));
}
