use advent::utilities::get_input::get_input;
use itertools::Itertools;
use lazy_regex::regex;
use utilities::{parsing::get_numbers::ContainsNumbers, structs::stopwatch::{ReportDuration, Stopwatch}};

type Input<'a> = &'a str;
type Output = usize;

fn main() {
    let mut stopwatch = Stopwatch::new();
    stopwatch.start();
    let input = get_input(16, 9).unwrap();
    println!("Input parsed ({})", stopwatch.lap().report());
    println!("1. {} ({})", part1(&input), stopwatch.lap().report());
    println!("2. {} ({})", part2(&input), stopwatch.lap().report());
    println!("Total: {}", stopwatch.stop().report());
}

fn solve(data: Input, recursive: bool) -> usize {
    let marker_rx = regex!(r"\(\d+x\d+\)");
    let mut length = 0;
    let mut index = 0;
    while index < data.len() {
        // get the next marker. If none found, return early, adding the length of the remaining unparsed string
        let Some(marker) = marker_rx.find(&data[index..]) else {
            return length + data[index..].len();
        };

        // add the length of any characters preceding the marker
        length += data[index..index + marker.start()].len();

        // get the length and number of repeats in the marker
        let (sequence_length, repeats): (usize, usize) = marker.as_str().get_numbers().collect_tuple().unwrap();

        // get the length of the sequence affected by the marker
        let post_marker = index + marker.end();
        let sequence = &data[post_marker..post_marker + sequence_length];
        let sequence = if recursive {
            solve(&sequence, recursive)
        } else {
            sequence.len()
        } * repeats;

        length += sequence;
        index += marker.end() + sequence_length;
    }
    length
}

fn part1(data: Input) -> Output {
    solve(data, false)
}

fn part2(data: Input) -> Output {
    solve(data, true)
}

#[test]
fn default() {
    let input = get_input(16, 9).unwrap();
    assert_eq!(110346, part1(&input));
    assert_eq!(10774309173, part2(&input));
}

// Input parsed (28μs)
// 1. 110346 (293μs)
// 2. 10774309173 (138μs)
// Total: 462μs