//! This is basically copied straight from u/maneatingape's solution, doing my best to understand the math and getting
//! about halfway there. If you want to see my ugly solution, see my Kotlin solution.

use advent::utilities::get_input::get_input;
use itertools::Itertools;
use utilities::structs::stopwatch::{ReportDuration, Stopwatch};

type Input<'a> = &'a str;
type Int = i128;

fn main() {
    let mut stopwatch = Stopwatch::new();
    stopwatch.start();
    let input = get_input(19, 22).unwrap();
    println!("Input parsed ({})", stopwatch.lap().report());
    println!("1. {} ({})", part1(&input), stopwatch.lap().report());
    println!("2. {} ({})", part2(&input), stopwatch.lap().report());
    println!("Total: {}", stopwatch.stop().report());
}

struct Deck {
    a: Int,
    c: Int,
    m: Int,
}

impl Deck {
    fn new(input: Input, m: Int) -> Self {
        input.lines()
            .map(|line| {
                let tokens = line.split(' ').collect_vec();
                match tokens[ .. ] {
                    [_, "into", _, _] => Self { a: m - 1, c: m - 1, m },
                    [_, "with", _, n] => {
                        let n: Int = n.parse().unwrap();
                        let a = (m + n % m) % m;
                        Self { a, c: 0, m }
                    },
                    ["cut", n] => {
                        let n: Int = n.parse().unwrap();
                        let c = (m - n % m) % m;
                        Self { a: 1, c, m }
                    }
                    _ => unreachable!(),
                }
            })
            .reduce(|a, b| a.compose(&b))
            .unwrap()
    }

    fn compose(&self, other: &Self) -> Self {
        Self {
            a: (self.a * other.a) % self.m,
            c: (other.a * self.c + other.c) % self.m,
            m: self.m,
        }
    }

    fn shuffle(&self, index: Int) -> Int {
        (self.a * index + self.c) % self.m
    }

    fn inverse(&self) -> Self {
        let m = self.m;
        let a = mod_inv(self.a, m);
        let c = m - (a * self.c) % m;
        Self { a, c, m }
    }

    fn power(&self, e: Int) -> Self {
        let m = self.m;
        let a = mod_pow(self.a, e, m);
        let c = (((a - 1) * mod_inv(self.a - 1, m) % m) * self.c) % m;
        Self { a, c, m }
    }
}

fn mod_inv(n: Int, m: Int) -> Int {
    let mut t = 0;
    let mut new_t = 1;
    let mut r = m;
    let mut new_r = n;

    while new_r != 0 {
        let quotient = r / new_r;
        (t, new_t) = (new_t, t - quotient * new_t);
        (r, new_r) = (new_r, r - quotient * new_r);
    }

    t + m
}

fn mod_pow(n: Int, mut e: Int, m: Int) -> Int {
    let mut b = n;
    let mut c = 1;

    while e > 0 {
        if e & 1 == 1 {
            c = (c * b) % m;
        }
        b = (b * b) % m;
        e = e >> 1;
    }

    c
}

fn part1(input: Input) -> Int {
    let deck = Deck::new(input, 10_007);
    deck.shuffle(2019)
}

fn part2(input: Input) -> Int {
    Deck::new(input, 119_315_717_514_047)
        .inverse()
        .power(101_741_582_076_661)
        .shuffle(2020)
}

#[test]
fn default() {
    let input = get_input(19, 22).unwrap();
    assert_eq!(6129, part1(&input));
    assert_eq!(71345377301237, part2(&input));
}

// Input parsed (14μs)
// 1. 6129 (13μs)
// 2. 71345377301237 (10μs)
// Total: 40μs
