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
            *index = (*index..balloons.len())
                .find(|i| balloons[*i] != bolt % 3)
                .unwrap_or(balloons.len() - 1)
                + 1;
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
    let balloons = get_balloons(input).repeat(repeats);
    let len = balloons.len();
    let mut killed = vec![false; len];
    let mut shots = 0;
    let mut doubles = 0;
    let mut skips = 0;
    let mut opposite_index = (len + 1) / 2;
    'outer: while shots + skips < len {
        // tracks if shots counter has reached an index that has already been killed, needing to up
        // the skip counter
        while killed[shots + skips] {
            skips += 1;
            if shots + skips == len { break 'outer; }
        }

        if (len - (shots + doubles)) & 1 == 0 { // if even
            if balloons[shots + skips] == shots % 3 {
                killed[opposite_index] = true;
                doubles += 1;
            }
            opposite_index += 1;
        }
        shots += 1;
    }
    shots
}

#[test]
fn default() {
    let (input1, input2, input3) = get_story_inputs(25, 2, 2);
    assert_eq!(131, linear_shots(&input1));
    assert_eq!(21665, circular_shots(&input2, 100));
    assert_eq!(21477463, circular_shots(&input3, 100_000));
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

// Input parsed (31μs)
// 1. 131 (9μs)
// 2. 21665 (186μs)
// 3. 21477463 (70ms)
// Total: 70ms