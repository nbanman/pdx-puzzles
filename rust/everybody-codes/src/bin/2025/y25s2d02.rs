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

fn circular_shots(input: Input, repeats: usize) -> usize {
    let balloons = get_balloons(input).repeat(repeats);
    let len = balloons.len();
    let half_len = (len + 1) / 2;

    // tracks indices corresponding to balloons that were shot opposite the circle
    let mut already_shot = vec![false; half_len];

    // tracks total number of shots
    let mut shots = 0;

    // tracks the number of shots that also shot an opposite balloon. Needed to calculate the
    // number of remaining balloons.
    let mut double_shots = 0;

    // tracks the number of indices that need to be skipped because they were already opposite-shot.
    // Needed to find the front-facing balloon.
    let mut skips = 0;

    // tracks the index of the opposite balloon
    let mut opposite_index = half_len;

    'outer: while shots + skips < len {

        // tracks if shots counter has reached an index that has already been shot, needing to up
        // the skip counter
        if shots + skips >= half_len {
            while already_shot[shots + skips - half_len] {
                skips += 1;
                if shots + skips == len { break 'outer; }
            }
        }

        if (len - (shots + double_shots)) & 1 == 0 { // if even...
            if balloons[shots + skips] == shots % 3 { // if bolt color matches balloon...
                already_shot[opposite_index - half_len] = true; // mark opposite balloon killed
                double_shots += 1;
            }

            // opposite index increases when balloons are even regardless of whether the balloon is
            // killed
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

// Input parsed (32μs)
// 1. 131 (6μs)
// 2. 21665 (145μs)
// 3. 21477463 (64ms)
// Total: 64ms