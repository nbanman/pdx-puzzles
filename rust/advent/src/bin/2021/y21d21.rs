use advent::utilities::get_input::get_input;
use itertools::Itertools;
use utilities::{parsing::get_numbers::ContainsNumbers, structs::stopwatch::{ReportDuration, Stopwatch}};

type Input = (usize, usize);
type Output = usize;

fn main() {
    let mut stopwatch = Stopwatch::new();
    stopwatch.start();
    let input = get_input(21, 21).unwrap();
    let input = parse_input(&input);
    println!("Input parsed ({})", stopwatch.lap().report());
    println!("1. {} ({})", part1(&input), stopwatch.lap().report());
    println!("2. {} ({})", part2(&input), stopwatch.lap().report());
    println!("Total: {}", stopwatch.stop().report());
}

struct Die {
    die: usize,
    counter: usize,
}

impl Die {
    fn roll(&mut self, n: usize) -> usize {
        self.counter += n;
        (0..n).fold(0, |acc, _| {
            self.die = if self.die == 100 { 1 } else { self.die + 1 };
            acc + self.die
        })
    }
}


fn parse_input(input: &str) -> Input {
    let (_, p1, _, p2) = input.get_numbers().collect_tuple().unwrap();
    (p1, p2)
}

fn part1((p1_start, p2_start): &Input) -> Output {
    let advance = |p: usize, n: usize| (p - 1 + n) % 10 + 1;
    let mut p1_score = 0;
    let mut p2_score = 0;
    let mut p1_pos = *p1_start;
    let mut p2_pos = *p2_start;
    let mut die = Die { die: 0, counter: 0 };

    loop {
        let p1_roll = die.roll(3);
        p1_pos = advance(p1_pos, p1_roll);
        p1_score += p1_pos;
        // println!("Player 1 rolls {p1_roll} and moves to space {p1_pos} for a total score of {p1_score}.");
        if p1_score >= 1_000 {
            return die.counter * p2_score;
        }
        let p2_roll = die.roll(3);
        p2_pos = advance(p2_pos, p2_roll);
        p2_score += p2_pos;
        // println!("Player 2 rolls {p2_roll} and moves to space {p2_pos} for a total score of {p2_score}.");
        if p2_score >= 1_000 {
            return die.counter * p1_score;
        }
    }
}

fn part2((p1_start, p2_start): &Input) -> Output {
    let rf = [(3, 1), (4, 3), (5, 6), (6, 7), (7, 6), (8, 3), (9, 1)];

    fn wins(p1: i64, t1: i64, p2: i64, t2: i64, rf: &[(i64, i64)]) -> (i64, i64) {
        if t2 <= 0 {
            return (0, 1);
        }

        let mut w1 = 0;
        let mut w2 = 0;
        for &(roll, frequency) in rf {
            let (c2, c1) = wins(p2, t2, (p1 + roll) % 10, t1 - 1 - (p1 + roll) % 10, rf);
            w1 += frequency * c1;
            w2 += frequency * c2;
        }
        (w1, w2)
    }
    let (w1, w2) = wins(
        *p1_start as i64 - 1,
        21,
        *p2_start as i64 - 1,
        21,
        &rf
    );
    if w1 > w2 { w1 as usize } else { w2 as usize }
}

#[test]
fn default() {
    let input = get_input(21, 21).unwrap();
    let input = parse_input(&input);
    assert_eq!(605070, part1(&input));
    assert_eq!(218433063958910, part2(&input));
}

// Input parsed (19μs)
// 1. 605070 (8μs)
// 2. 218433063958910 (190ms)
// Total: 190ms