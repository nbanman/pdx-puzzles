use rayon::iter::ParallelIterator;
use everybody_codes::utilities::inputs::get_event_inputs;
use itertools::Itertools;
use rayon::iter::IntoParallelRefIterator;
use utilities::{minmax::minmax, parsing::get_numbers::ContainsNumbers, structs::stopwatch::{ReportDuration, Stopwatch}};

type Input<'a> = &'a str;

fn main() {
    let mut stopwatch = Stopwatch::new();
    stopwatch.start();
    let (input1, input2, input3) = get_event_inputs(25, 8);
    println!("Input parsed ({})", stopwatch.lap().report());
    println!("1. {} ({})", part1(&input1), stopwatch.lap().report());
    println!("2. {} ({})", part2(&input2), stopwatch.lap().report());
    println!("3. {} ({})", part3(&input3), stopwatch.lap().report());
    println!("Total: {}", stopwatch.stop().report());
}

fn parse(input: Input) -> Vec<isize> {
    input.get_numbers::<isize>().map(|n| n - 1).collect()
}

fn part1(instructions: Input) -> usize {
    let half = 32 / 2;
    let instructions = parse(instructions);

    instructions.iter().tuple_windows()
        .filter(|&(a, b)| (a - b).abs() == half)
        .count()
}

fn part2(instructions: Input) -> usize {
    let instructions = parse(instructions);
    let crosses = instructions.iter().tuple_windows()
        .map(|(a, b)| {
            let (min, max) = minmax(a, b);
            (*min, *max)
        });

    let mut completed: Vec<(isize, isize)> = Vec::new();
    let mut knots = 0;

    for (a, b) in crosses {
        for &(ca, cb) in completed.iter() {
            let semicircle = ca + 1 .. cb;
            if semicircle.contains(&a) {
                // b must be outside
                if b < ca || b > cb {
                    knots += 1;
                }
            } else if semicircle.contains(&b) {
                if a < ca || a > cb {
                    knots += 1;
                }
            } 
        }
        completed.push((a, b));
    }
    knots
}

fn part3(instructions: Input) -> usize {
    let instructions = parse(instructions);
    let nails = 256;
    let crosses: Vec<_> = instructions.iter().tuple_windows()
        .map(|(a, b)| {
            let (min, max) = minmax(a, b);
            (*min, *max)
        })
        .collect();
    let combinations = (0..nails).tuple_combinations().collect::<Vec<(_, _)>>();
    combinations.par_iter()
        .map(|&(a, b)| {
            let mut cuts = 0;
            for &(ca, cb) in crosses.iter() {
                if a == ca && b == cb {
                    cuts += 1;
                    continue;
                }
                let semicircle = ca + 1 .. cb;
                if semicircle.contains(&a) {
                    // b must be outside
                    if b < ca || b > cb {
                        cuts += 1;
                    }
                } else if semicircle.contains(&b) {
                    if a < ca || a > cb {
                        cuts += 1;
                    }
                }
            }
            cuts
        })
        .max()
        .unwrap()
}

#[test]
fn default() {
    let (input1, input2, input3) = get_event_inputs(25, 8);
    assert_eq!(58, part1(&input1));
    assert_eq!(2924358, part2(&input2));
    assert_eq!(2792, part3(&input3));
}

// Input parsed (42μs)
// 1. 58 (6μs)
// 2. 2924358 (8.009ms)
// 3. 2792 (26.008ms)
// Total: 34.071ms
