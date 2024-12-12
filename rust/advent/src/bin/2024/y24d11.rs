use std::{array, collections::HashMap};

use advent::utilities::get_input::get_input;
use utilities::{parsing::get_numbers::ContainsNumbers, structs::stopwatch::{ReportDuration, Stopwatch}};

type Input<'a> = &'a str;
type Output = usize;

fn main() {
    let mut stopwatch = Stopwatch::new();
    stopwatch.start();
    let input = get_input(24, 11).unwrap();
    println!("Input parsed ({})", stopwatch.lap().report());
    println!("1. {} ({})", solve(&input, 25), stopwatch.lap().report());
    println!("2. {} ({})", solve(&input, 75), stopwatch.lap().report());
    println!("Total: {}", stopwatch.stop().report());
}

#[derive(Clone, Copy, Debug)]
enum StoneState {
    Change(usize),
    Split(usize, usize),
}

impl StoneState {
    fn new(n: usize) -> Self {
        if n == 0 {
            Self::Change(1)
        } else {
            let n_string = n.to_string();
            let n_len = n_string.len();
            if n_string.len() & 1 == 0 {
                let l = n_string[..n_len / 2].parse().unwrap();
                let r = n_string[n_len / 2..].parse().unwrap();
                Self::Split(l, r)
            } else {
                Self::Change(n * 2024)
            }
        }
    }
}

#[derive(Clone, Copy, Debug)]
struct Stone {
    initial_value: usize,
    amount: usize,
    created: usize,
    state: StoneState,
}

fn solve(input: Input, blinks: usize) -> Output {
    let mut gestator: [HashMap<usize, usize>; 3] = array::from_fn(|_| HashMap::new());
    let mut tracker: HashMap<usize, (usize, usize, usize)> = HashMap::new();
    let mut stones: Vec<Stone> = input.get_numbers()
        .map(|initial| Stone { 
            initial_value: initial,
            amount: 1,
            created: 0,
            state: StoneState::new(initial), 
        })
        .collect();
    let mut new_stones: Vec<Stone> = Vec::new();
    for blink in 1..=blinks {
        let gestated: Vec<(usize, usize)> = gestator[blink % 3].drain().collect();
        for (value, amount) in gestated {
            if let Some(&(l, r , gestation)) = tracker.get(&value) {
                let bucket = gestator
                    .get_mut((blink + gestation) % 3)
                    .unwrap();
                *bucket.entry(l).or_default() += amount;
                *bucket.entry(r).or_default() += amount;
            } else {
                let new_stone = Stone {
                    initial_value: value,
                    amount,
                    created: blink,
                    state: StoneState::new(value),
                };
                new_stones.push(new_stone);
            }
        }
        for stone in stones {
            match stone.state {
                StoneState::Change(next_value) => {
                    add_stone(
                        next_value, 
                        stone, 
                        blink, 
                        &mut new_stones, 
                        &mut tracker, 
                        &mut gestator,
                        false,
                    );
                },
                StoneState::Split(l_val, r_val) => {
                    add_stone(l_val, stone, blink, &mut new_stones, &mut tracker, &mut gestator, true);
                    add_stone(r_val, stone, blink, &mut new_stones, &mut tracker, &mut gestator, true);
                    tracker.insert(stone.initial_value, (l_val, r_val, blink - stone.created));
                },
            }
        }
        stones = new_stones;
        new_stones = Vec::new();
    }
    let mut total_stones = 0;
    total_stones += stones.into_iter()
        .map(|stone| stone.amount)
        .sum::<usize>();
    
    // Gestator has double the amount it should because it hasn't split yet, so divide by 2.
    total_stones += gestator.into_iter()
        .flat_map(|bucket| bucket.into_values())
        .sum::<usize>() / 2;
    total_stones
}

fn add_stone(
    next_value: usize, 
    stone: Stone, 
    blink: usize,
    new_stones: &mut Vec<Stone>,
    tracker: &mut HashMap<usize, (usize, usize, usize)>,
    gestator: &mut [HashMap<usize, usize>; 3],
    from_split: bool,
) {
    if let Some(&(l, r, gestation)) = tracker.get(&next_value) {
        *gestator[(blink + gestation) % 3].entry(l).or_default() += stone.amount;
        *gestator[(blink + gestation) % 3].entry(r).or_default() += stone.amount;
    } else {
        let next = if from_split {
            Stone {
                initial_value: next_value,
                created: blink,
                state: StoneState::new(next_value),
                ..stone
            }
        } else {
            Stone {
                state: StoneState::new(next_value),
                ..stone
            }
        };
        new_stones.push(next);
    }
}

#[test]
fn default() {
    let input = get_input(24, 11).unwrap();
    assert_eq!(231278, solve(&input, 25)); 
    assert_eq!(274229228071551, solve(&input, 75)); 
}

// Input parsed (15μs)
// 1. 231278 (298μs)
// 2. 274229228071551 (4ms)
// Total: 5ms
