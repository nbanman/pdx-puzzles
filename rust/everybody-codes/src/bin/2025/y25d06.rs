use everybody_codes::utilities::inputs::get_event_inputs;
use utilities::structs::stopwatch::{ReportDuration, Stopwatch};

type Input<'a> = &'a str;

fn main() {
    let mut stopwatch = Stopwatch::new();
    stopwatch.start();
    let (input1, input2, input3) = get_event_inputs(25, 6);
    println!("Input parsed ({})", stopwatch.lap().report());
    println!("1. {} ({})", part1(&input1), stopwatch.lap().report());
    println!("2. {} ({})", part2(&input2), stopwatch.lap().report());
    println!("3. {} ({})", part3(&input3), stopwatch.lap().report());
    println!("Total: {}", stopwatch.stop().report());
}

fn part1(input: Input) -> usize {
    let mut knights = 0;
    let mut pairs = 0;
    for &b in input.as_bytes() {
        match b {
            b'A' => knights += 1,
            b'a' => pairs += knights,
            _ => {}
        }
    }
    pairs
}

fn part2(input: Input) -> usize {
    let mut knights: [usize; 3] = [0; 3];
    let mut pairs = 0;
    for &b in input.as_bytes() {
        match b {
            b if b.is_ascii_uppercase() => { knights[(b - b'A') as usize] += 1; },
            b => { pairs += knights[(b - b'a') as usize]; },
        }
    }
    pairs
}

fn part3(input: Input) -> usize {
    let mut pre_repeat: [usize; 3] = [0; 3];
    let mut pre: [usize; 3] = [0; 3];
    let mut post_repeat: [usize; 3] = [0; 3];
    let mut post: [usize; 3] = [0; 3];
    let mut pairs = 0;

    // constants
    let input = input.as_bytes();
    let distance = 1000;
    let repeats = 1000;

    //pre-fill
    for &b in &input[input.len() - distance..] {
        if b.is_ascii_uppercase() {
            pre_repeat[(b - b'A') as usize] += 1;
        }
    }
    for &b in &input[0..=distance] {
        if b.is_ascii_uppercase() {
            post[(b - b'A') as usize] += 1;
        }
    }

    for (idx, &b) in input.iter().enumerate() {
        if b.is_ascii_uppercase() {
            let cur_idx = (b - b'A') as usize;
            post[cur_idx] -= 1;
        } else {
            let b_idx = (b - b'a') as usize;
            pairs += pre_repeat[b_idx] * (repeats - 1)
                + pre[b_idx] * repeats
                + post[b_idx] * repeats
                + post_repeat[b_idx] * (repeats - 1);
        }
        // drop stuff behind
        let pre_idx = idx as isize - distance as isize ;
        if pre_idx < 0 {
            let pre_drop = input[pre_idx.rem_euclid(input.len() as isize) as usize];
            if pre_drop.is_ascii_uppercase() {
                pre_repeat[(pre_drop - b'A') as usize] -= 1;
            }
        } else {
            let pre_drop = input[pre_idx as usize];
            if pre_drop.is_ascii_uppercase() {
                pre[(pre_drop - b'A') as usize] -= 1;
            }
        }
        let post_idx = idx + distance + 1;
        if post_idx >= input.len() {
            let post_drop = input[post_idx.rem_euclid(input.len())];
            if post_drop.is_ascii_uppercase() {
                post_repeat[(post_drop - b'A') as usize] += 1;
            }
        } else {
            let post_drop = input[post_idx];
            if post_drop.is_ascii_uppercase() {
                post[(post_drop - b'A') as usize] += 1;
            }
        }
        if b.is_ascii_uppercase() {
            let cur_idx = (b - b'A') as usize;
            pre[cur_idx] += 1;
        }
    }
    pairs
}

#[test]
fn default() {
    let (input1, input2, input3) = get_event_inputs(25, 6);
    assert_eq!(190, part1(&input1));
    assert_eq!(4011, part2(&input2));
    assert_eq!(1665939853, part3(&input3));
}

// Input parsed (34μs)
// 1. 190 (7μs)
// 2. 4011 (2μs)
// 3. 1665939853 (118μs)
// Total: 165μs