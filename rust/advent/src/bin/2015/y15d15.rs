use std::cmp::max;
use std::ops::Mul;

use advent::utilities::get_input::get_input;
use itertools::Itertools;
use utilities::{
    parsing::get_numbers::ContainsNumbers,
    structs::stopwatch::{ReportDuration, Stopwatch},
};

type Input = (Vec<Ingredient>, Vec<Vec<Int>>);
type Int = i32;
type Output = Int;

fn main() {
    let mut stopwatch = Stopwatch::new();
    stopwatch.start();
    let input = get_input(15, 15).unwrap();
    let input = parse_input(&input);
    println!("Input parsed ({})", stopwatch.lap().report());
    println!("1. {} ({})", part1(&input), stopwatch.lap().report());
    println!("2. {} ({})", part2(&input), stopwatch.lap().report());
    println!("Total: {}", stopwatch.stop().report());
}

#[derive(Debug, Copy, Clone)]
struct Ingredient {
    qualities: [Int; 4],
    calories: Int,
}

fn parse_input(input: &str) -> Input {
    let ingredients = input
        .get_numbers::<Int>()
        .tuples()
        .map(
            |(capacity, durability, flavor, texture, calories)| Ingredient {
                qualities: [capacity, durability, flavor, texture],
                calories,
            },
        )
        .collect_vec();

    let mut cookies: Vec<Vec<Int>> = (0..=100).map(|i| vec![i]).collect();
    let mut next: Vec<Vec<Int>> = Vec::new();
    for index in 1..ingredients.len() {
        for cookie in cookies.drain(..) {
            let current: Int = cookie.iter().sum();
            if current == 100 {
                let mut next_cookie = cookie.clone();
                next_cookie.push(0);
                next.push(next_cookie);
            } else if index == ingredients.len() - 1 {
                let mut next_cookie = cookie.clone();
                next_cookie.push(100 - current);
                next.push(next_cookie);
            } else {
                for n in 0..100 - current {
                    let mut next_cookie = cookie.clone();
                    next_cookie.push(n);
                    next.push(next_cookie);
                }
            }
        }
        std::mem::swap(&mut cookies, &mut next);
    }

    (ingredients, cookies)
}

fn score(cookie: &[Int], ingredients: &[Ingredient]) -> Int {
    let mut qualities = [0; 4];
    for (ingredient_index, &quantity) in cookie.iter().enumerate() {
        let ingredient_qualities = ingredients[ingredient_index].qualities;
        for i in 0..4 {
            qualities[i] += ingredient_qualities[i] * quantity;
        }
    }
    qualities
        .into_iter()
        .map(|v| max(0, v))
        .reduce(Int::mul)
        .unwrap()
}

fn meets_calories(cookie: &[Int], ingredients: &[Ingredient]) -> bool {
    let cookie_calories: Int = cookie
        .iter()
        .enumerate()
        .map(|(ingredient_index, &quantity)| quantity * ingredients[ingredient_index].calories)
        .sum();
    cookie_calories == 500
}

fn part1((ingredients, cookies): &Input) -> Output {
    cookies
        .iter()
        .map(|cookie| score(cookie, ingredients))
        .max()
        .unwrap()
}

fn part2((ingredients, cookies): &Input) -> Output {
    cookies
        .iter()
        .filter(|&cookie| meets_calories(cookie, ingredients))
        .map(|cookie| score(cookie, ingredients))
        .max()
        .unwrap()
}

#[test]
fn default() {
    let input = get_input(15, 15).unwrap();
    let input = parse_input(&input);
    assert_eq!(222870, part1(&input));
    assert_eq!(117936, part2(&input));
}

// Input parsed (11ms)
// 1. 222870 (1ms)
// 2. 117936 (386μs)
// Total: 13ms
