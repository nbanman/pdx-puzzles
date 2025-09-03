use everybody_codes::utilities::inputs::get_story_inputs;
use utilities::structs::stopwatch::{ReportDuration, Stopwatch};

type Input<'a> = &'a str;

fn main() {
    let mut stopwatch = Stopwatch::new();
    stopwatch.start();
    let (input1, input2, input3) = get_story_inputs(25, 2, 2);
    println!("Input parsed ({})", stopwatch.lap().report());
    println!("1. {} ({})", linear_shots(&input1), stopwatch.lap().report());
    println!("2. {} ({})", circular_shots(&input2, 100), stopwatch.lap().report());
    println!("3. {} ({})", circular_shots(&input3, 100000), stopwatch.lap().report());
    println!("Total: {}", stopwatch.stop().report());
}

fn linear_shots(input: Input) -> usize {
    let balloons = get_balloons(input);
    (0..)
        .scan(0usize, |index, bolt| {
            if *index == balloons.len() { return None; }
            // print!("index: {index}, bolt: {}, ", bolt % 3);
            let temp_index = index.to_owned();
            *index = (*index..balloons.len())
                .find(|i| balloons[*i] != bolt % 3)
                .unwrap_or(balloons.len() - 1)
                + 1;
            // println!("shot {}", *index - temp_index);
            Some(*index)
        })
        .count()
}

fn get_balloons(input: Input) -> Vec<usize> {
    input.as_bytes().into_iter()
        .map(|&b| {
            match b {
                b'R' => 0,
                b'G' => 1,
                b'B' => 2,
                c => panic!("{} is not a valid input", c as char),
            }
        })
        .collect()
}

fn circular_shots(input: Input, repeats: usize) -> usize {
    let mut balloons = get_balloons(input);
    balloons.reverse();
    balloons = balloons.repeat(repeats);
    let mut shots = 0;
    while let Some(shot) = balloons.pop() {
        let remaining = balloons.len();
        if remaining & 1 == 1 && shot == shots % 3 {
            balloons.remove(remaining / 2);
        }
        shots += 1;
    }
    shots
}

fn part3(input: Input) -> usize {
    todo!()
}

#[test]
fn default() {
    let (input1, input2, input3) = get_story_inputs(25, 2, 2);
    // assert_eq!(ZZ, part1(&input1));
    // assert_eq!(ZZ, part2(&input2));
    // assert_eq!(ZZ, part3(&input3));
}

#[test]
fn examples() {
    let inputs = [r"GRBGGGBBBRRRRRRRR", r"GGBR", r"BBRGGRRGBBRGGBRGBBRRBRRRBGGRRRBGBGG"];
    assert_eq!(7, linear_shots(inputs[0]));
    assert_eq!(14, circular_shots(inputs[1], 5));
    assert_eq!(304, circular_shots(inputs[2], 10));
    assert_eq!(1464, circular_shots(inputs[2], 50));
    assert_eq!(2955, circular_shots(inputs[2], 100));
}