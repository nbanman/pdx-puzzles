use everybody_codes::utilities::inputs::get_event_inputs_provisional;
use utilities::{
    parsing::get_numbers::ContainsNumbers,
    structs::stopwatch::{ReportDuration, Stopwatch},
};

type Input<'a> = &'a str;

fn main() {
    let mut stopwatch = Stopwatch::new();
    stopwatch.start();
    let (input1, input2, input3) = get_event_inputs_provisional(26, 1);
    println!("Input parsed ({})", stopwatch.lap().report());
    if let Some(input1) = input1 {
        println!("1. {} ({})", part1(&input1), stopwatch.lap().report());
    }
    if let Some(input2) = input2 {
        println!("2. {} ({})", part2(&input2), stopwatch.lap().report());
    }
    if let Some(input3) = input3 {
        println!("3. {} ({})", part3(&input3), stopwatch.lap().report());
    }
    println!("Total: {}", stopwatch.stop().report());
}

fn part1(input: Input) -> u32 {
    input
        .get_numbers::<u32>()
        .map(|n| {
            let root = (n as f32).sqrt().ceil() as u32;
            root * root - n
        })
        .sum()
}

fn part2(input: Input) -> u32 {
    input
        .get_numbers::<u32>()
        .map(|n| {
            let frame = 3.max(((n as f32 + 4.0) / 4.0).ceil() as u32);
            frame * 4 - 4 - n
        })
        .sum()
}

fn part3(input: Input) -> u32 {
    input
        .get_numbers::<u32>()
        .map(|n| {
            let root = (n as f32).sqrt();
            let frame = if root < 3.0 {
                3
            } else if root == root.trunc() {
                root.trunc() as u32 + 1
            } else {
                root.ceil() as u32
            };
            let frame = if frame & 1 == 0 { frame + 1 } else { frame };
            frame * frame - 1 - n
        })
        .sum()
}

#[test]
fn default() {
    let (input1, input2, input3) = everybody_codes::utilities::inputs::get_event_inputs(26, 1);
    assert_eq!(1292, part1(&input1));
    assert_eq!(312, part2(&input2));
    assert_eq!(749883, part3(&input3));
}

#[test]
fn example() {
    let input1 = r"3,11,12,16,24,25";
    assert_eq!(11, part1(input1));
    assert_eq!(9, part2(input1));
    assert_eq!(61, part3(input1));
}

// Input parsed (27μs)
// 1. 1292 (6μs)
// 2. 312 (9μs)
// 3. 749883 (14μs)
// Total: 60μs
