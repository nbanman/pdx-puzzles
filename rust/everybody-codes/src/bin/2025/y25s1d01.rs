use std::cmp::min;

use everybody_codes::utilities::inputs::get_story_inputs;
use indexmap::IndexSet;
use itertools::Itertools;
use utilities::parsing::get_numbers::ContainsNumbers;
use utilities::structs::stopwatch::{ReportDuration, Stopwatch};

fn main() {
    let mut stopwatch = Stopwatch::new();
    stopwatch.start();

    let (input1, input2, input3) = get_story_inputs(25, 1, 1);
    println!("Input parsed ({})", stopwatch.lap().report());

    println!("1. {} ({})", solve(&input1, 100), stopwatch.lap().report());
    println!("2. {} ({})", solve(&input2, 5), stopwatch.lap().report());
    println!("3. {} ({})", part3(&input3), stopwatch.lap().report());
    println!("Total: {}", stopwatch.stop().report());
}

fn solve(input: &str, max_width: usize) -> usize {
    input
        .lines()
        .map(|line| {
            let (a, b, c, x, y, z, m) = line.get_numbers().collect_tuple().unwrap();
            eni(a, x, m, max_width) + eni(b, y, m, max_width) + eni(c, z, m, max_width)
        })
        .max()
        .unwrap()
}
fn eni(n: usize, exp: usize, modulus: usize, values: usize) -> usize {
    let (set, index_of_first_repeated, cycle_length, turns_in_cycle) = prep(n, exp, modulus);
    let keep_in_cycle = min(values, turns_in_cycle);
    let last_place_in_cycle =
        ((turns_in_cycle.checked_sub(1).unwrap_or_default()) % cycle_length) as isize;

    // handle the cycle part of the score
    let cycle_score = (0..keep_in_cycle as isize)
        .map(|i| {
            let index = (last_place_in_cycle - i).rem_euclid(cycle_length as isize) as usize
                + index_of_first_repeated;
            set[index]
        })
        .fold(0, |score, remainder: usize| {
            let score = score * 10usize.pow(get_width(remainder)) + remainder;
            score
        });

    // handle the prefix part of the score
    let keep_in_prefix = min(values, exp) - keep_in_cycle;
    let score= (index_of_first_repeated - keep_in_prefix..index_of_first_repeated)
        .rev()
        .map(|index| set[index])
        .fold(cycle_score, |score, remainder: usize| {
            score * 10usize.pow(get_width(remainder)) + remainder
        });
    // println!("n: {n}, exp: {exp}, mod: {modulus}, score: {score}");
    score
}

fn prep(n: usize, exp: usize, modulus: usize) -> (IndexSet<usize>, usize, usize, usize) {
    let mut set = IndexSet::with_capacity(modulus);
    let mut remainder = 1;
    let capacity = min(exp, modulus);
    let mut index_of_first_repeated = capacity; // aka prefix length
    let mut cycle_length = capacity;

    for i in 0..capacity {
        remainder = remainder * n % modulus;
        if !set.contains(&remainder) {
            set.insert(remainder);
        } else {
            index_of_first_repeated = set
                .get_index_of(&remainder)
                .expect("Set already checked, so will always have index.");
            cycle_length = i - index_of_first_repeated;
            break;
        }
    }

    let turns_in_cycle = exp - index_of_first_repeated;
    (set, index_of_first_repeated, cycle_length, turns_in_cycle)
}

fn part3(input: &str) -> usize {
    input
        .lines()
        .map(|line| {
            let (a, b, c, x, y, z, m) = line.get_numbers().collect_tuple().unwrap();
            eni2(a, x, m) + eni2(b, y, m) + eni2(c, z, m)
        })
        .max()
        .unwrap()
}

fn eni2(n: usize, exp: usize, modulus: usize) -> usize {
    let (set, index_of_first_repeated, cycle_length, turns_in_cycle) = prep(n, exp, modulus);
    let cycle_split = turns_in_cycle % cycle_length;
    let mut set_iter = set.iter();
    let prefix_score: usize = set_iter.by_ref().take(index_of_first_repeated).sum();
    let incomplete: usize = set_iter.by_ref().take(cycle_split).sum();
    let cycle_sum = incomplete + set_iter.sum::<usize>();
    prefix_score + incomplete + cycle_sum * (turns_in_cycle / cycle_length)
}

fn get_width(n: usize) -> u32 {
    let mut width = 1;
    let mut n = n;
    while n > 9 {
        n /= 10;
        width += 1;
    }
    return width;
}

#[test]
fn example() {
    // let (input1, input2, input3) = get_story_inputs(24, 1);
    let input1 = r"A=4 B=4 C=6 X=3 Y=4 Z=5 M=11
A=8 B=4 C=7 X=8 Y=4 Z=6 M=12
A=2 B=8 C=6 X=2 Y=4 Z=5 M=13
A=5 B=9 C=6 X=8 Y=6 Z=8 M=14
A=5 B=9 C=7 X=6 Y=6 Z=8 M=15
A=8 B=8 C=8 X=6 Y=9 Z=6 M=16";
    let input2 = r"A=4 B=4 C=6 X=3 Y=14 Z=15 M=11
A=8 B=4 C=7 X=8 Y=14 Z=16 M=12
A=2 B=8 C=6 X=2 Y=14 Z=15 M=13
A=5 B=9 C=6 X=8 Y=16 Z=18 M=14
A=5 B=9 C=7 X=6 Y=16 Z=18 M=15
A=8 B=8 C=8 X=6 Y=19 Z=16 M=16";
    let input3 = r"A=3657 B=3583 C=9716 X=903056852 Y=9283895500 Z=85920867478 M=188
A=6061 B=4425 C=5082 X=731145782 Y=1550090416 Z=87586428967 M=107
A=7818 B=5395 C=9975 X=122388873 Y=4093041057 Z=58606045432 M=102
A=7681 B=9603 C=5681 X=716116871 Y=6421884967 Z=66298999264 M=196
A=7334 B=9016 C=8524 X=297284338 Y=1565962337 Z=86750102612 M=145";
    let input4 = r"A=4 B=4 C=6 X=3000 Y=14000 Z=15000 M=110
A=8 B=4 C=7 X=8000 Y=14000 Z=16000 M=120
A=2 B=8 C=6 X=2000 Y=14000 Z=15000 M=130
A=5 B=9 C=6 X=8000 Y=16000 Z=18000 M=140
A=5 B=9 C=7 X=6000 Y=16000 Z=18000 M=150
A=8 B=8 C=8 X=6000 Y=19000 Z=16000 M=160";
    let input5 = r"A=3657 B=3583 C=9716 X=903056852 Y=9283895500 Z=85920867478 M=188
A=6061 B=4425 C=5082 X=731145782 Y=1550090416 Z=87586428967 M=107
A=7818 B=5395 C=9975 X=122388873 Y=4093041057 Z=58606045432 M=102
A=7681 B=9603 C=5681 X=716116871 Y=6421884967 Z=66298999264 M=196
A=7334 B=9016 C=8524 X=297284338 Y=1565962337 Z=86750102612 M=145";
    assert_eq!(11611972920, solve(input1, 100));
    assert_eq!(11051340, solve(input2, 5));
    assert_eq!(1507702060886, solve(input3, 5));
    assert_eq!(3279640, part3(input4));
    assert_eq!(7276515438396, part3(input5));
    // assert_eq!(28180, solve(&input3, 3));
}

#[test]
fn default() {
    let (input1, input2, input3) = get_story_inputs(25, 1, 1);
    assert_eq!(1281421558, solve(&input1, 100));
    assert_eq!(165117476211886, solve(&input2, 5));
    assert_eq!(670944509842136, part3(&input3));
}

// Input parsed (58μs)
// 1. 1281421558 (25μs)
// 2. 165117476211886 (194μs)
// 3. 670944509842136 (46ms)
// Total: 46ms