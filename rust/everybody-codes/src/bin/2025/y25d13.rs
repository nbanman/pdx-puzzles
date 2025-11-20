use std::collections::VecDeque;
use everybody_codes::utilities::inputs::get_event_inputs;
use utilities::{structs::stopwatch::{ReportDuration, Stopwatch}};

type Input<'a> = &'a str;

fn main() {
    let mut stopwatch = Stopwatch::new();
    stopwatch.start();
    let (input1, input2, input3) = get_event_inputs(25, 13);
    println!("Input parsed ({})", stopwatch.lap().report());
    println!("1. {} ({})", part1(&input1), stopwatch.lap().report());
    println!("2. {} ({})", part2(&input2), stopwatch.lap().report());
    println!("3. {} ({})", part3(&input3), stopwatch.lap().report());
    println!("Total: {}", stopwatch.stop().report());
}

fn solve(input: Input, total_turns: u64) -> u32 {
    // ranges stored in VecDeque to approximate 'circular' layout of the ranges
    let mut lock: VecDeque<(u32, u32)> = VecDeque::with_capacity(500);
    lock.push_back((1, 1));

    // This bool cycles true/false, telling us to place ranges forward or backward on the lock.
    let mut forward = true;

    for (lo, hi) in ranges(input) {
        if forward {
            lock.push_back((lo, hi));
        } else {
            lock.push_front((lo, hi))
        }
        forward = !forward;
    }

    // calculate the point in the lock where the ranges start to be reversed, then rotate the
    // deque so that those ranges are at the end
    let reverse_point = lock.len() / 2 + 1;
    lock.rotate_right(reverse_point);

    // use mod math to eliminate a bunch of full circles.
    let dial_len: u32 = lock.iter().map(|(a, b)| b - a + 1).sum();
    let mut turns_left = (total_turns % dial_len as u64) as u32;

    // Iterate through the ranges. On each pass, lower turns_left by the # of numbers in that range.
    // When the # of numbers is higher than the remaining target, you know that the turn is in that
    // range. Depending on whether the range is added to the left or right of the initial position,
    // you either add from the low part of the range or subtract from the high part of the range.
    for i in 0..lock.len() {
        let &(lo, hi) = lock.get(i).unwrap();
        let numbers = hi - lo + 1;
        if turns_left < numbers {
            return if i < reverse_point {
                lo + turns_left
            } else {
                hi - turns_left
            }
        }
        turns_left -= numbers;
    }
    unreachable!()
}

fn ranges(input: Input) -> impl Iterator<Item = (u32, u32)> {
    input.lines().map(|line| {
        let mut rng = line.split('-');
        let lo = rng.next().unwrap().parse::<u32>().unwrap();
        let hi = rng.next()
            .map(|it| it.parse::<u32>().unwrap())
            .unwrap_or_else(|| lo);
        (lo, hi)
    })
}

fn part1(input: Input) -> u32 {
    solve(input, 2025)
}
fn part2(input: Input) -> u32 {
    solve(input, 20_252_025)
}
fn part3(input: Input) -> u32 {
    solve(input, 202_520_252_025)
}

#[test]
fn default() {
    let (input1, input2, input3) = get_event_inputs(25, 13);
    assert_eq!(353, part1(&input1));
    assert_eq!(7613, part2(&input2));
    assert_eq!(217823, part3(&input3));
}

// Input parsed (28μs)
// 1. 353 (10μs)
// 2. 7613 (4μs)
// 3. 217823 (14μs)
// Total: 59μs
