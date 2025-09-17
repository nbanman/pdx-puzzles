use itertools::Itertools;
use advent::utilities::get_input::get_input;
use utilities::structs::stopwatch::{ReportDuration, Stopwatch};

type Input = Vec<Output>;
type Output = usize;

fn main() {
    let mut stopwatch = Stopwatch::new();
    stopwatch.start();
    let input = get_input(20, 5).unwrap();
    let input = parse_input(&input);
    println!("Input parsed ({})", stopwatch.lap().report());
    println!("1. {} ({})", part1(&input), stopwatch.lap().report());
    println!("2. {} ({})", part2(&input), stopwatch.lap().report());
    println!("Total: {}", stopwatch.stop().report());
}

fn parse_input(input: &str) -> Input {
    // The seat id is Row * 8 + Column. Since there are 8 columns this means that the seatIDs are
    // sequential from 0 to 1016, and the FBLR code is just a binary number with 'F' and 'L' meaning
    // '0' and 'B' and 'R' meaning 1.
    input.lines()
        .map(|it| {
            it.as_bytes().iter().enumerate().fold(0, |acc, (idx, &b)| {
                acc + if b == b'B' || b == b'R' { 1 << (9 - idx) } else { 0 }
            })
        })
        .sorted_unstable()
        .collect()
}

fn part1(seat_ids: &Input) -> Output {
    *seat_ids.last().unwrap()
}

fn part2(seat_ids: &Input) -> Output {
    // The seatIds should all be contiguous. Yours is missing, so look for the first non-contiguous
    // seatId in the sorted list of seatIds. Yours would be the seatId immediately below that.
    seat_ids.iter()
        .tuple_windows() // pair up previous seatId and the next seatId
        .find(|&(prev, next)| *prev + 1 != *next)// find the first instance where the next seatId is not contiguous
        .map(|(prev, _)| prev + 1) // add 1 since ticket is the missing seat_id
        .unwrap()
}

#[test]
fn default() {
    let input = get_input(20, 5).unwrap();
    let input = parse_input(&input);
    assert_eq!(922, part1(&input));
    assert_eq!(747, part2(&input));
}

// Input parsed (75μs)
// 1. 922 (4μs)
// 2. 747 (1μs)
// Total: 85μs