use std::ops::Add;

use advent::utilities::get_input::get_input;
use utilities::structs::stopwatch::{ReportDuration, Stopwatch};

type Input = Vec<Pattern>;
type Output = usize;

fn main() {
    let mut stopwatch = Stopwatch::new();
    stopwatch.start();
    let input = get_input(23, 13).unwrap();
    let input = parse_input(&input);
    println!("Input parsed ({})", stopwatch.lap().report());
    println!("1. {} ({})", part1(&input), stopwatch.lap().report());
    println!("2. {} ({})", part2(&input), stopwatch.lap().report());
    println!("Total: {}", stopwatch.stop().report());
}

#[derive(Debug)]
struct Pattern {
    rows: Vec<Vec<bool>>,
    cols: Vec<Vec<bool>>,
}

impl Pattern {
    fn seam_summary(&self, smudged: bool) -> usize {
        if let Some(hz_seam) = self.find_seam(true, smudged) {
            hz_seam
        } else {
            self.find_seam(false, smudged).unwrap() * 100
        }
    }

    fn find_seam(&self, is_hz: bool, smudged: bool) -> Option<usize> {
        let lines = if is_hz { &self.cols } else { &self.rows };
        for i in 0..lines.len() - 1 {
            let initial_diff = Pattern::diff(&lines[i], &lines[i + 1]);
            if initial_diff == Diff::Zero || (initial_diff == Diff::One && smudged) {
                let mut diff = initial_diff;
                for j in 1..=i {
                    if j + i + 1 == lines.len() { break; }
                    diff = diff + Pattern::diff(&lines[i - j], &lines[i + j + 1]);
                    if diff == Diff::More || (diff == Diff::One && !smudged) { break; }
                }
                if smudged && diff == Diff::One { return Some(i + 1); }
                if !smudged && diff == Diff::Zero { return Some(i + 1); }
            }
        }
        None
    }

    fn diff(a: &[bool], b: &[bool]) -> Diff {
        match a.iter().zip(b.iter()).filter(|(&aa, &bb)| aa != bb).count() {
            0 => Diff::Zero,
            1 => Diff::One,
            _ => Diff::More,
        }
    }
}

#[derive(Clone, Copy, Debug, PartialEq)]
enum Diff {
    Zero,
    One,
    More,
}

impl Add for Diff {
    type Output = Self;

    fn add(self, rhs: Self) -> Self::Output {
        match self {
            Diff::Zero => {
                match rhs {
                    Diff::Zero => Diff::Zero,
                    Diff::One => Diff::One,
                    Diff::More => Diff::More,
                }
            },
            Diff::One => if rhs == Diff::Zero { Diff:: One } else { Diff::More },
            Diff::More => Diff::More,
        }
    }
}


fn parse_input(input: &str) -> Input {
    input.split("\n\n")
        .map(|stanza| {
            let width = stanza.find('\n').unwrap();
            let mirrors: Vec<bool> = stanza.as_bytes().iter()
                .filter(|&&c| c != b'\n')
                .map(|&c| c == b'#')
                .collect();
            let height = mirrors.len() / width;
            let rows = (0..height)
                .map(|n| {
                    let offset = n * width;
                    mirrors[offset..offset + width].iter()
                        .copied()
                        .collect()
                })
                .collect();
            let cols = (0..width)
                .map(move |n| {
                    (0..height).map(|row| mirrors[row * width + n]).collect() 
                })
                .collect();
            Pattern { rows, cols } 
        })
        .collect()
}

fn part1(patterns: &Input) -> Output {
    patterns.iter().map(|pattern| pattern.seam_summary(false)).sum()
}

fn part2(patterns: &Input) -> Output {
    patterns.iter().map(|pattern| pattern.seam_summary(true)).sum()
}

#[test]
fn default() {
    let input = get_input(23, 13).unwrap();
    let input = parse_input(&input);
    assert_eq!(27505, part1(&input));
    assert_eq!(22906, part2(&input));
}

// Input parsed (158μs)
// 1. 27505 (18μs)
// 2. 22906 (17μs)
// Total: 195μs