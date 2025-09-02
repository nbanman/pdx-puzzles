use itertools::Itertools;
use everybody_codes::utilities::inputs::get_story_inputs;
use utilities::parsing::get_numbers::ContainsNumbers;
use utilities::structs::stopwatch::{ReportDuration, Stopwatch};

type Input<'a> = &'a str;

fn main() {
    let mut stopwatch = Stopwatch::new();
    stopwatch.start();
    let (input1, input2, input3) = get_story_inputs(25, 1, 3);
    println!("Input parsed ({})", stopwatch.lap().report());
    println!("1. {} ({})", position_sum(&input1), stopwatch.lap().report());
    println!("2. {} ({})", days_until_alignment(&input2), stopwatch.lap().report());
    println!("3. {} ({})", days_until_alignment(&input3), stopwatch.lap().report());
    println!("Total: {}", stopwatch.stop().report());
}

fn parse<'a>(input: &'a str) -> impl Iterator<Item = Snail> + 'a {
    input
        .get_numbers()
        .tuples()
        .map(|(x, y)| Snail { x, y })
}

fn position_sum(input: Input) -> usize {
    parse(input).map(|snail| snail.score(100)).sum()
}

fn days_until_alignment(input: Input) -> usize {
    parse(input)
        .map(|snail| snail.positions_and_offset())
        .reduce(|(interval, days), (positions, offset)| {
            let next_days = (days..)
                .step_by(interval as usize)
                .find(|&it| (it - offset).rem_euclid(positions) == 0)
                .unwrap();

            let next_interval = interval * positions;
            (next_interval, next_days)
        })
        .unwrap()
        .1 as usize
}

struct Snail { x: usize, y: usize }


impl Snail {
    fn score(&self, day: usize) -> usize {
        let modulo = self.x + self.y - 1;
        let x = (self.x - 1 + day) % modulo + 1;
        let y = (self.y as isize - 1 - day as isize).rem_euclid(modulo as isize) as usize + 1;
        x + 100 * y
    }

    fn positions_and_offset(&self) -> (isize, isize) {
        let positions = (self.x + self.y - 1) as isize;
        let offset = self.y as isize - 1;
        (positions, offset)
    }
}

#[test]
fn default() {
    let (input1, input2, input3) = get_story_inputs(25, 1, 3);
    assert_eq!(2754, position_sum(&input1));
    assert_eq!(1034698, days_until_alignment(&input2));
    assert_eq!(91517344388, days_until_alignment(&input3));
}

#[test]
fn examples() {
    let inputs = [
        r"x=1 y=2
x=2 y=3
x=3 y=4
x=4 y=4",
        r"x=12 y=2
x=8 y=4
x=7 y=1
x=1 y=5
x=1 y=3",
        r"x=3 y=1
x=3 y=9
x=1 y=5
x=4 y=10
x=5 y=3",
    ];
    assert_eq!(1310, position_sum(inputs[0]));
    assert_eq!(14, days_until_alignment(inputs[1]));
    assert_eq!(13659, days_until_alignment(inputs[2]));
}

// Input parsed (24μs)
// 1. 2754 (5μs)
// 2. 1034698 (2μs)
// 3. 91517344388 (3μs)
// Total: 39μs
