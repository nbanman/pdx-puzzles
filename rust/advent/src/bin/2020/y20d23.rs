use advent::utilities::get_input::get_input;
use utilities::structs::stopwatch::{ReportDuration, Stopwatch};

type Input = Vec<usize>;
type Output = usize;

fn main() {
    let mut stopwatch = Stopwatch::new();
    stopwatch.start();
    let input = get_input(20, 23).unwrap();
    let input = parse_input(&input);
    println!("Input parsed ({})", stopwatch.lap().report());
    println!("1. {} ({})", part1(&input), stopwatch.lap().report());
    println!("2. {} ({})", part2(&input), stopwatch.lap().report());
    println!("Total: {}", stopwatch.stop().report());
}

fn parse_input(input: &str) -> Input {
    input.as_bytes().iter().map(|&b| b as usize - 48).collect()
}

fn crab_cups(number_of_cups: usize, first_labels: &Vec<usize>, rounds: usize) -> Vec<usize> {
    let start = first_labels[0];
    let mut current = start;
    let mut cups: Vec<_> = (1..=number_of_cups + 1).collect();
    for &next in &first_labels[1..] {
        cups[current] = next;
        current = next;
    }

    if number_of_cups == first_labels.len() {
        // all cups are individually labeled, which means "current" is at the end. So link current
        // back to the beginning
        cups[current] = start;
    } else {
        // many other cups exist after the individually labeled ones, so link current to the first
        // generically labeled one
        cups[current] = 10;

        // the last number (1_000_000) links back to the beginning
        cups[number_of_cups] = start;
    }

    current = start;

    for _ in 0..rounds {
        // grab next 3
        let a = cups[current];
        let b = cups[a];
        let c = cups[b];

        // cups[0] is not used because there is no number that would ever call it, since the
        // numbers range from 1 to 1_000_000. So the destination starts at one less than the
        // current, or starts from the very end. Then it just keeps going backward until the
        // destination is not a, b, or c.
        let mut destination = if current > 1 { current - 1 } else { cups.len() - 1 };
        while destination == a || destination == b || destination == c {
            destination = if destination > 1 { destination - 1 } else { cups.len() - 1 };
        }

        // makes cups[current] the value after c
        cups[current] = cups[c];

        // moves current to the next
        current = cups[c];

        // connects cups c to the destination
        cups[c] = cups[destination];

        // connects the destination to a
        cups[destination] = a;
    }
    cups
}

fn part1(first_labels: &Input) -> Output {
    let cups = crab_cups(first_labels.len(), first_labels, 100);
    (0..8).fold((0, 1), |(acc, i), _| (10 * acc + cups[i], cups[i])).0
}

fn part2(first_labels: &Input) -> Output {
    let cups = crab_cups(1_000_000, first_labels, 10_000_000);
    let first = cups[1];
    let second = cups[first];
    first * second
}

#[test]
fn default() {
    let input = get_input(20, 23).unwrap();
    let input = parse_input(&input);
    assert_eq!(94238657, part1(&input));
    assert_eq!(3072905352, part2(&input));
}

