use std::{collections::VecDeque, usize};

use advent::utilities::get_input::get_input;
use itertools::Itertools;
use md5::{digest::core_api::CoreWrapper, Digest, Md5, Md5Core};
use rayon::iter::IntoParallelIterator;
use utilities::structs::stopwatch::{ReportDuration, Stopwatch};
use rayon::iter::ParallelIterator;

type Input<'a> = &'a str;
type Output = usize;

fn main() {
    let mut stopwatch = Stopwatch::new();
    stopwatch.start();
    let input = get_input(16, 14).unwrap();
    println!("Input parsed ({})", stopwatch.lap().report());
    println!("1. {} ({})", part1(&input), stopwatch.lap().report());
    println!("2. {} ({})", part2(&input), stopwatch.lap().report());
    println!("Total: {}", stopwatch.stop().report());
}

fn solve(salt: Input, chunk_size: usize, hashing: fn(&mut CoreWrapper<Md5Core>, String) -> String) -> Output {
    // for each hex value 0-f, store index of last time 5-string appeared
    let mut fives: [Option<usize>; 16] = [None; 16];

    // mutable list of validated keys
    let mut keys = Vec::with_capacity(70);

    // rolling list of 1,000 of the 3-string value of hashes. returns 'X' if no 3-string in hash
    let mut threes: VecDeque<char> = VecDeque::with_capacity(1001);

    // Iterator starting with increasing index, generating a hash based on that and the salt.
    // For each hash, record any 5-string in the fives with the current index. Add the 3-string
    // value to the rolling list. When 3-string values start rolling off, check fives to see if
    // that value has shown up as a five. If so, add it to the list of keys. Keep going until the
    // 64th key is found.
    let hash_generator = (0..)
        .flat_map(|n| {
            let start = n * chunk_size;
            let end = start + chunk_size;
            let chunk: Vec<_> = (start..end)
                .into_par_iter()
                .map(|seed| {
                    let mut digest = Md5::new();
                    let to_hash = format!("{}{}", salt, seed);
                    hashing(&mut digest, to_hash)
                })
                .collect();
            chunk.into_iter()
        });

    for (index, hash) in hash_generator.enumerate() {
        // For each hash, record any 5-string in the fives with the current index.
        for five in hash.chars()
            .tuple_windows()
            .filter(|(a, b, c, d, e)| a == b && b == c && c == d && d == e)
            .map(|it| it.0)
        {
            fives[hex_index(five)] = Some(index);
        }
        let three = hash.chars()
            .tuple_windows()
            .find(|(a, b, c)| a == b && b == c)
            .map(|(it, _, _)| it)
            .unwrap_or('X');
        if let Some(key) = add_to_threes(&mut threes, three, &fives, index) {
            keys.push(key);
        }
        if keys.len() == 64 {
            break;
        }
    }
    return *keys.last().unwrap();
}

fn add_to_threes(
    threes: &mut VecDeque<char>,
    three: char,
    fives: &[Option<usize>],
    index: usize,
) -> Option<usize> {
    // add value
    threes.push_back(three);

    // do nothing if list is not full
    if threes.len() <= 1000 {
        return None;
    }

    // else, start rolling off
    let evaluate = threes.pop_front().unwrap();
    if evaluate == 'X' {
        return None;
    }
    let eval_index = index - 1000;

    // check fives to see if any of the next 1,000 hashes has a 5-string matching the rolling off 3-string
    if (eval_index + 1..=index).contains(&fives[hex_index(evaluate)].unwrap_or(usize::MAX)) {
        Some(eval_index)
    } else {
        None
    }
}

fn hex_index(evaluate: char) -> usize {
    let evaluate = evaluate as u8 as usize;
    let offset = if evaluate < 58 { 48 } else { 87 };
    evaluate - offset
}

fn part1(salt: Input) -> Output {
    fn hashing(digest: &mut CoreWrapper<Md5Core>, to_hash: String) -> String {
        digest.update(to_hash);
        let hash = digest.finalize_reset();
        let mut buf = [0u8; 32];
        base16ct::lower::encode_str(&hash, &mut buf).unwrap().to_string()
    }
    solve(salt, 1, hashing)
}

fn part2(salt: Input) -> Output {
    fn hashing(digest: &mut CoreWrapper<Md5Core>, to_hash: String) -> String {
        let mut buf = [0u8; 32];
        (0..2017).fold(to_hash, |acc, _| {
            digest.update(acc);
            let arr = digest.finalize_reset();
            base16ct::lower::encode_str(&arr, &mut buf).unwrap().to_string()
        })
    }
    solve(salt, 512, hashing)
}

#[test]
fn default() {
    let input = get_input(16, 14).unwrap();
    assert_eq!(18626, part1(&input));
    assert_eq!(20092, part2(&input));
}

// Input parsed (20μs)
// 1. 18626 (10ms)
// 2. 20092 (344ms)
// Total: 355ms