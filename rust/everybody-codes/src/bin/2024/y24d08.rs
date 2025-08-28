use everybody_codes::utilities::inputs::get_event_inputs;
use itertools::Itertools;
use utilities::structs::stopwatch::{ReportDuration, Stopwatch};

fn main() {
    let mut stopwatch = Stopwatch::new();
    stopwatch.start();
    let (input1, input2, input3) = get_event_inputs(24, 8);
    let input1 = input1.parse().unwrap();
    let input2 = input2.parse().unwrap();
    let input3 = input3.parse().unwrap();
    println!("Inputs loaded ({})", stopwatch.lap().report());
    println!(
        "1. {} ({})",
        solve(1, 2, input1, false),
        stopwatch.lap().report()
    );
    println!(
        "2. {} ({})",
        solve(input2, 1111, 20_240_000, false),
        stopwatch.lap().report()
    );
    println!(
        "3. {} ({})",
        solve(input3, 10, 202_400_000, true),
        stopwatch.lap().report()
    );
    println!("Total: {}", stopwatch.stop().report());
}

fn solve(priests: usize, acolytes: usize, blocks_available: usize, part3: bool) -> usize {
    let mut blocks = 1;
    let mut width = 1;
    let mut thickness = 1;
    let mut columns = vec![1];
    let mut inner_dimensions = 0;
    while blocks - inner_dimensions < blocks_available {
        width += 2;
        thickness = (thickness * priests) % acolytes;
        if part3 {
            thickness += acolytes;
            columns.push(0);
            for column in columns.iter_mut() {
                *column += thickness;
            }
            let removal = |column: usize| ((priests * width) * column) % acolytes;
            inner_dimensions = columns
                .iter()
                .dropping(1)
                .dropping_back(1)
                .map(|&column| removal(column) * 2)
                .sum::<usize>()
                + removal(*columns.first().unwrap());
        }
        let layer_blocks = thickness * width;
        blocks += layer_blocks;
    }
    if part3 {
        blocks - inner_dimensions - blocks_available
    } else {
        width * (blocks - blocks_available)
    }
}

#[test]
fn tests() {
    let tests = ["13", "3", "2"];
    let tests: Vec<usize> = tests.iter().map(|s| s.parse().unwrap()).collect();
    assert_eq!(21, solve(1, 2, tests[0], false));
    assert_eq!(27, solve(tests[1], 5, 50, false));
    assert_eq!(2, solve(tests[2], 5, 160, true));
}
