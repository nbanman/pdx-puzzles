use std::{array::from_fn, cmp::min, collections::HashSet};

use everybody_codes::utilities::inputs::get_inputs;
use utilities::parsing::get_numbers::ContainsNumbers;

const NUMBER_OF_COLUMNS: usize = 4;
type Formation = [Vec<usize>; NUMBER_OF_COLUMNS];

fn main() {
    let (input1, input2, input3) = get_inputs(24, 5);
    println!("1. {}", part1(&input1));
    println!("2. {}", part2(&input2));
    println!("3. {}", part3(&input3));
}

fn parse_input(input: &str) -> Formation {
    let mut columns = from_fn(|_| Vec::new());
    for (idx, n) in input.get_numbers::<usize>().enumerate() {
        columns[idx % 4].push(n);
    }
    columns
}

fn play_round(round: usize, columns: &mut Formation) {
    let clapper_col = (round - 1) % NUMBER_OF_COLUMNS;
    let next_col = round % NUMBER_OF_COLUMNS;
    let next_len = columns[next_col].len();
    let clapper = columns[clapper_col].remove(0);  
    let pos = (clapper - 1) % (next_len * 2);
    let pos = min(pos, next_len) - pos.checked_sub(next_len).unwrap_or_default();
    
    columns[next_col].insert(pos, clapper);
}

fn shout(columns: &Formation) -> usize {
    let shout = columns.iter().fold(0usize, |acc, column| {
        let front = column[0];
        acc * 10usize.pow((front as f64).log10().floor() as u32 + 1) + front
    });
    shout
}

fn part1(input: &str) -> usize {
    let mut columns = parse_input(input);
    for round in 1..=10 {
        play_round(round, &mut columns);
    }
    shout(&columns)
}

fn part2(input: &str) -> usize {
    let mut columns = parse_input(input);
    let digits = input.lines().next().unwrap().chars()
        .filter(|&c| c.is_ascii_digit())
        .count();
    let mut counter = vec![0usize; 10usize.pow(digits as u32)];
    for round in 1.. {
        play_round(round, &mut columns);
        let shouted = shout(&columns);
        counter[shouted] += 1;
        if counter[shouted] == 2024 {
            return round * shouted
        }
    }
    unreachable!()
}

fn part3(input: &str) -> usize {
    let mut columns = parse_input(input);
    let mut cache = HashSet::new();
    let mut highest_number = 0;
    for round in 1.. {
        play_round(round, &mut columns);
        let shouted = shout(&columns);
        if highest_number < shouted { highest_number = shouted; }
        if !cache.insert(columns.clone()) {
            return highest_number;
        }
    }
    unreachable!()
}

#[test]
fn examples() {
    let test1 = r"
2 3 4 5
3 4 5 2
4 5 2 3
5 2 3 4
    ".trim();
    let test2 = r"
2 3 4 5
6 7 8 9
    ".trim();
    assert_eq!(2323, part1(test1));
    assert_eq!(50877075, part2(test2));
    assert_eq!(6584, part3(test2));
}
