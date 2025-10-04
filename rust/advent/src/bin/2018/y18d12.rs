use std::iter::successors;
use advent::utilities::get_input::get_input;
use itertools::Itertools;
use utilities::structs::stopwatch::{ReportDuration, Stopwatch};

type Output = i64;

fn main() {
    let mut stopwatch = Stopwatch::new();
    stopwatch.start();
    let input = get_input(18, 12).unwrap();
    let input = parse_input(&input);
    println!("Input parsed ({})", stopwatch.lap().report());
    println!("1. {} ({})", part1(input.clone()), stopwatch.lap().report());
    println!("2. {} ({})", part2(input), stopwatch.lap().report());
    println!("Total: {}", stopwatch.stop().report());
}

// an iterator producing successive rows of plants
fn parse_input(input: &str) -> impl Iterator<Item = Vec<bool>> + Clone {
    let (initial_row, commands) = input.trim_end().split_once("\n\n").unwrap();
    let initial_row: Vec<bool> = initial_row.chars().skip(15)
        .map(|c| c == '#')
        .collect();

    // database of patterns that result in a plant going into the pot for the next iteration.
    let mut patterns = [false; 48];
    for command in commands.as_bytes().split(|&b| b == b'\n') {
        if command.last().unwrap() == &b'#' {
            let pattern = command.iter().take(5).fold(0, |acc, &b| {
                if b == b'#' {
                    (acc << 1) + 1
                } else {
                    acc << 1
                }
            });
            patterns[pattern] = true;
        }
    }

    let mask = 15;
    let generator = successors(Some(initial_row), move |plant| {
        // next creates the binary value of the five pots centered around the index, 2 to each side of center.
        // each iteration, the leftmost value is sloughed off, everything shifts one to the left, and a new
        // rightmost value is added.
        // that value is then checked for inclusion in the patterns database, returning true/false for that index
        let next: Vec<bool> = (0..plant.len() + 4)
            .scan(0, |state, index| {
                if index >= plant.len() || !plant[index] {
                    *state = (*state & mask) << 1;
                } else {
                    *state = ((*state & mask) << 1) + 1;
                }
                Some(*state)
            })
            .map(|pattern| patterns[pattern])
            .collect();
        Some(next)
    });
    generator
}

fn sum_of_pot_numbers(pots: &[bool], generations: usize) -> Output {
    pots.iter().enumerate()
        .map(|(index, &b)| if b { index as i64 - generations as i64 * 2 } else { 0 })
        .sum()
}

fn part1(generator: impl Iterator<Item = Vec<bool>>) -> Output {
    sum_of_pot_numbers(&generator.take(21).last().unwrap(), 20)
}

fn part2(generator: impl Iterator<Item = Vec<bool>>) -> Output {
    // too many generations to naively compute!
    let generations = 50_000_000_000;

    // upon observation, the growth is chaotic at first but then finds a stable pattern where growth is constant.
    // Thus, the strategy is to look at generations 10 at a time. When the difference between each is the 
    // same, we can surmise that the growth has stabilized. That group provides enough information to 
    // solve part 2.
    let group_size = 10;
    let first_stable: Vec<(usize, i64)> = generator
        .enumerate() // pair up rows with their index, which is the number of generations
        // transform rows to their pot sum
        .map(|(index, value)| (index, sum_of_pot_numbers(&value, index)) )
        // look at generations 10 at a time
        .chunks(group_size)
        .into_iter()
        .map(|chunk| chunk.collect_vec())
        // terminate and provide the first group where the difference in pot sums between each in the group is
        // the same
        .find(|group| {
            group.iter().tuple_windows()
                .map(|(&(_, a), &(_, b))| b - a)
                .counts()
                .len() == 1
        })
        .unwrap();

    // repeat_index is the generation of this last chaotic value, so we don't double count generations when applying
    // the stable generation count
    // last_unstable_value is the last "chaotic" value obtained by the generator
    let (repeat_index, last_unstable_value) = first_stable[0];

    // the amount that each successive generation adds to the pot number count
    let stable_increment = first_stable[1].1 - last_unstable_value;

    // putting it all together
    return last_unstable_value + stable_increment * (generations - repeat_index) as i64
}

#[test]
fn default() {
    let input = get_input(18, 12).unwrap();
    let input = parse_input(&input);
    assert_eq!(4110, part1(input.clone()));
    assert_eq!(2650000000466, part2(input));
}

// Input parsed (16μs)
// 1. 4110 (18μs)
// 2. 2650000000466 (198μs)
// Total: 236μs
