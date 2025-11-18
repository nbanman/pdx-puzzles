use everybody_codes::utilities::inputs::get_event_inputs;
use utilities::{parsing::get_numbers::ContainsNumbers, structs::stopwatch::{ReportDuration, Stopwatch}};

type Input<'a> = &'a str;

fn main() {
    let mut stopwatch = Stopwatch::new();
    stopwatch.start();
    let (input1, input2, input3) = get_event_inputs(25, 11);
    println!("Input parsed ({})", stopwatch.lap().report());
    println!("1. {} ({})", part1(&input1), stopwatch.lap().report());
    println!("2. {} ({})", part2(&input2), stopwatch.lap().report());
    println!("3. {} ({})", part3(&input3), stopwatch.lap().report());
    println!("Total: {}", stopwatch.stop().report());
}

fn parse(input: Input) -> Vec<u64> {
    input.get_numbers().collect()
}

fn solve(mut flock: Vec<u64>, round_limit: Option<usize>) -> (usize, Vec<u64>) {
    let mut changed = true;
    let mut round = 0;
    // phase 1
    while changed {
        round += 1;
        changed = false;
        for i in 0..flock.len() - 1 {
            if flock[i] > flock[i + 1] {
                flock[i] -= 1;
                flock[i + 1] += 1;
                changed = true;
            }
        }
    }
    round -= 1;
    changed = true;
    while changed {
        changed = false;
        for i in 0..flock.len() - 1 {
            if flock[i] < flock[i + 1] {
                flock[i] += 1;
                flock[i + 1] -= 1;
                changed = true;
            }
        }
        round += 1;
        if let Some(limit) = round_limit && round == limit {
            break;
        }
    }
    (round - 1, flock)
}

fn part1(input: Input) -> u64 {
    let flock = parse(input);
    let flock = solve(flock, Some(10)).1;
    flock.into_iter().enumerate().fold(0, |acc, (idx, v)| {
        acc + (idx as u64 + 1) * v
    })
}

fn part2(input: Input) -> usize {
    solve(parse(input), None).0
}

fn part3(input: Input) -> u64 {
    let flock = parse(input);
    let mean = flock.iter().sum::<u64>() / flock.len() as u64;
    flock.into_iter()
        .filter(|&n| n < mean)
        .map(|n| mean - n)
        .sum()
}

#[test]
fn default() {
    let (input1, input2, input3) = get_event_inputs(25, 11);
    assert_eq!(271, part1(&input1));
    assert_eq!(3984738, part2(&input2));
    assert_eq!(130353341887463, part3(&input3));
}

// 1. 271 (8μs)
// 2. 3984738 (145.818ms)
// 3. 130353341887463 (12μs)
// Total: 145.873ms