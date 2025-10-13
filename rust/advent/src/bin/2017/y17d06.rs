use advent::utilities::get_input::get_input;
use indexmap::IndexSet;
use utilities::{parsing::get_numbers::ContainsNumbers, structs::stopwatch::{ReportDuration, Stopwatch}};

type Input = (IndexSet<Vec<usize>>, Vec<usize>);
type Output = usize;

fn main() {
    let mut stopwatch = Stopwatch::new();
    stopwatch.start();
    let input = get_input(17, 6).unwrap();
    let input = parse_input(&input);
    println!("Input parsed ({})", stopwatch.lap().report());
    println!("1. {} ({})", part1(&input), stopwatch.lap().report());
    println!("2. {} ({})", part2(&input), stopwatch.lap().report());
    println!("Total: {}", stopwatch.stop().report());
}

fn parse_input(input: &str) -> Input {
    let mut state: Vec<usize> = input.get_numbers().collect();
    let mut previous: IndexSet<Vec<usize>> = IndexSet::new();
    previous.insert(state.clone());
    loop {
        state = reallocate(&state);
        if !previous.insert(state.clone()) {
            return (previous, state);
        }
    }
}

fn reallocate(state: &Vec<usize>) -> Vec<usize> {
    let (index, alloc) = state.iter().copied().enumerate()
        .rev()
        .max_by_key(|&(_, v)| v)
        .unwrap();
    let mut new_list = state.clone();
    new_list[index] = 0;
    for i in 1..=alloc {
        new_list[(index + i) % state.len()] += 1;
    }
    new_list
}

fn part1(input: &Input) -> Output {
    let (set, _, ) = input;
    set.len()
}

fn part2(input: &Input) -> Output {
    let (set, last) = input;
    set.len() - set.get_index_of(last).unwrap()
}

#[test]
fn default() {
    let input = get_input(17, 6).unwrap();
    let input = parse_input(&input);
    assert_eq!(12841, part1(&input));
    assert_eq!(8038, part2(&input));
}

// Input parsed (1ms)
// 1. 12841 (5μs)
// 2. 8038 (2μs)
// Total: 1ms