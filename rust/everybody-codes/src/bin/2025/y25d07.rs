use std::mem;
use everybody_codes::utilities::inputs::get_event_inputs;
use rustc_hash::FxHashMap;
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

fn parse<'a>(input: Input<'a>) -> (Vec<&'a str>, FxHashMap<char, Vec<char>>) {
    let (names, paths) = input.split_once("\n\n").unwrap();
    let names = names.split(',').collect();
    let paths = paths.lines()
        .map(|line| {
            let mut iter = line.chars()
                .filter(|it| it.is_ascii_alphabetic());
            let k = iter.next().unwrap();
            let v = iter.collect();
            (k, v)
        })
        .collect();
    (names, paths)
}

fn part1(input: Input) -> String {
    let (names, paths) = parse(input);

    let initial: Vec<char> = paths.keys()
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
    let initial: Vec<char> = paths.keys()
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
    let initial: Vec<char> = paths.keys()
        .filter(|it| it.is_uppercase())
        .copied()
        .collect();
    let mut sum = 0;
    
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
        let max_len = 11 - name.len();
        let min_len = 7 - name.len();
        
        let mut steps = 0;
        let mut todo: Vec<char> = Vec::new();
        let mut next: Vec<char> = Vec::new();
        todo.extend_from_slice(&available);
        while !todo.is_empty() {
            steps += 1;
            if steps > max_len { break; }
            for c in todo.drain( .. ) {
                if steps >= min_len { sum += 1; }
                if let Some(slice) = paths.get(&c) {
                    next.extend_from_slice(slice);
                }
            }
            std::mem::swap(&mut todo, &mut next);
        }
    }
    sum
}

#[test]
fn default() {
    let (input1, input2, input3) = get_event_inputs(25, 7);
    assert_eq!("Ulendris".to_string(), part1(&input1));
    assert_eq!(2529, part2(&input2));
    assert_eq!(1945135, part3(&input3));
}

// Input parsed (40μs)
// 1. Ulendris (8μs)
// 2. 2529 (14μs)
// 3. 1945135 (14.611ms)
// Total: 14.681ms