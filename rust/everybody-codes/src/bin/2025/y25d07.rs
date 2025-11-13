use everybody_codes::utilities::inputs::get_event_inputs;
use rustc_hash::FxHashMap;
use std::mem;
use utilities::structs::stopwatch::{ReportDuration, Stopwatch};

type Input<'a> = &'a str;

fn main() {
    let mut stopwatch = Stopwatch::new();
    stopwatch.start();
    let (input1, input2, input3) = get_event_inputs(25, 7);
    println!("Input parsed ({})", stopwatch.lap().report());
    println!("1. {} ({})", part1(&input1), stopwatch.lap().report());
    println!("2. {} ({})", part2(&input2), stopwatch.lap().report());
    println!("3. {} ({})", part3(&input3), stopwatch.lap().report());
    println!("Total: {}", stopwatch.stop().report());
}

fn parse(input: Input<'_>) -> (Vec<&'_ str>, FxHashMap<char, Vec<char>>) {
    let (names, paths) = input.split_once("\n\n").unwrap();
    let names = names.split(',').collect();
    let paths = paths
        .lines()
        .map(|line| {
            let mut iter = line.chars().filter(|it| it.is_ascii_alphabetic());
            let k = iter.next().unwrap();
            let v = iter.collect();
            (k, v)
        })
        .collect();
    (names, paths)
}

fn part1(input: Input) -> String {
    let (names, paths) = parse(input);

    let initial: Vec<char> = paths
        .keys()
        .filter(|it| it.is_uppercase())
        .copied()
        .collect();

    'outer: for name in names {
        let mut available = &initial;
        for c in name.chars() {
            if available.contains(&c) {
                if let Some(next) = paths.get(&c) {
                    available = next;
                } else {
                    if c as u8 == *name.as_bytes().last().unwrap() {
                        return name.to_string();
                    }
                    continue 'outer;
                }
            } else {
                continue 'outer;
            }
        }
    }
    unreachable!()
}

fn part2(input: Input) -> usize {
    let (names, paths) = parse(input);
    let initial: Vec<char> = paths
        .keys()
        .filter(|it| it.is_uppercase())
        .copied()
        .collect();
    let mut sum = 0;

    'outer: for (idx, &name) in names.iter().enumerate() {
        let mut available = &initial;
        for c in name.chars() {
            if available.contains(&c) {
                if let Some(next) = paths.get(&c) {
                    available = next;
                } else {
                    if c as u8 == *name.as_bytes().last().unwrap() {
                        sum += idx + 1;
                        continue;
                    }
                    continue 'outer;
                }
            } else {
                continue 'outer;
            }
        }
        sum += idx + 1;
    }
    sum
}

fn part3(input: Input) -> usize {
    let (mut names, paths) = parse(input);
    for i in 0.. {
        if i == names.len() {
            break;
        }
        let mut cur = "";
        mem::swap(&mut cur, &mut names[i]);
        for ii in (0..names.len()).rev() {
            if names[ii].contains(&cur) {
                names.remove(ii);
            }
        }
        mem::swap(&mut cur, &mut names[i]);
    }
    let initial: Vec<char> = paths
        .keys()
        .filter(|it| it.is_uppercase())
        .copied()
        .collect();
    let mut sum = 0;

    let mut cache: [Option<usize>; 512] = [None; 512];

    'outer: for name in names {
        let mut available = &initial;
        for c in name.chars() {
            if available.contains(&c) {
                if let Some(next) = paths.get(&c) {
                    available = next;
                } else {
                    continue 'outer;
                }
            } else {
                continue 'outer;
            }
        }
        let len = name.len();
        let last = name.as_bytes()[len - 1] as char;
        let hash = hash_of(last, len);
        sum += cache[hash]
            .unwrap_or_else(|| count_names(last, len, hash, &paths, &mut cache));
    }
    sum
}

fn count_names(
    c: char,
    depth: usize,
    hash: usize,
    paths: &FxHashMap<char, Vec<char>>,
    cache: &mut [Option<usize>; 512],
) -> usize {
    // base case 1: Max depth reached.
    if depth == 11 {
        cache[hash] = Some(1);
        return 1;
    }

    let mut name_count = if depth >= 7 { 1 } else { 0 };
    
    // base case 2: No children remaining.
    let Some(next) = paths.get(&c) else {
        cache[hash] = Some(name_count);
        return name_count;
    };

    // otherwise 
    for &nc in next {
        let n_hash = hash_of(nc, depth + 1);
        name_count += cache[n_hash]
            .unwrap_or_else(|| count_names(nc, depth + 1, n_hash, paths, cache));
    }
    cache[hash] = Some(name_count);
    name_count
}

fn hash_of(c: char, depth: usize) -> usize {
    ((c as usize - 97) << 4) | depth
}

#[test]
fn default() {
    let (input1, input2, input3) = get_event_inputs(25, 7);
    assert_eq!("Ulendris".to_string(), part1(&input1));
    assert_eq!(2529, part2(&input2));
    assert_eq!(1945135, part3(&input3));
}

// Input parsed (28μs)
// 1. Ulendris (11μs)
// 2. 2529 (13μs)
// 3. 1945135 (13μs)
// Total: 69μs
