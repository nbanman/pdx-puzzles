use std::cmp::{max, min};
use advent::utilities::get_input::get_input;
use itertools::Itertools;
use utilities::{
    parsing::get_numbers::ContainsNumbers,
    structs::stopwatch::{ReportDuration, Stopwatch},
};

type Input = Vec<(u64, u64)>;
type Output = u64;

struct InvalidIds {
    n: u64,
    hi: u64,
    digits: u32,
    portion: u32,
    portion_digits: u32,
    top: u64,
    done: bool,
}

impl InvalidIds {
    fn new(n: u64, hi: u64, digits: u32, portion: u32) -> Self {
        let top = n / (10u64.pow(digits / portion * (portion - 1)));
        let portion_digits = digits / portion;
        Self {
            n,
            hi,
            digits,
            portion,
            portion_digits,
            top,
            done: false,
        }
    }
}

impl Iterator for InvalidIds {
    type Item = u64;
    fn next(&mut self) -> Option<u64> {
        if self.done {
            return None;
        }
        let candidate = (1..self.portion).fold(self.top, |acc, _| {
            acc * 10u64.pow(self.digits / self.portion) + self.top
        });

        if candidate > self.hi {
            self.done = true;
            return None;
        }

        self.top += 1;
        if get_digits(self.top) != self.portion_digits {
            // rolled over
            self.done = true;
        }

        if candidate >= self.n {
            return Some(candidate);
        }

        // escape valve
        self.next()
    }
}

#[derive(Debug, PartialEq, Eq)]
enum ZipperStatus {
    Zipping,
    Iter1Done,
    Iter2Done,
    Done,
}

pub struct ZipAndSort<T, U> {
    iter1: T,
    iter2: U,
    peek1: Option<u64>,
    peek2: Option<u64>,
    status: ZipperStatus,
}

impl<T, U> ZipAndSort<T, U>
where
    T: Iterator<Item = u64>,
    U: Iterator<Item = u64>,
{
    fn new(iter1: T, iter2: U) -> Self {
        Self {
            iter1,
            iter2,
            peek1: None,
            peek2: None,
            status: ZipperStatus::Zipping,
        }
    }
}

impl <T, U> Iterator for ZipAndSort<T, U>
where
    T: Iterator<Item = u64>,
    U: Iterator<Item = u64>,
{
    type Item = u64;
    fn next(&mut self) -> Option<Self::Item> {
        // early return if done
        if self.status == ZipperStatus::Done {
            return None;
        }

        // load up "peekable" areas so that they can be compared and stored
        if self.peek1.is_none() && self.status != ZipperStatus::Iter1Done {
            self.peek1 = self.iter1.next();
            if self.peek1.is_none() {
                if self.status == ZipperStatus::Iter2Done {
                    self.status = ZipperStatus::Done;
                    return None
                } else {
                    self.status = ZipperStatus::Iter1Done;
                }
            }
        }

        if self.peek2.is_none() && self.status != ZipperStatus::Iter2Done {
            self.peek2 = self.iter2.next();
            if self.peek2.is_none() {
                if self.status == ZipperStatus::Iter1Done {
                    self.status = ZipperStatus::Done;
                    return None
                } else {
                    self.status = ZipperStatus::Iter2Done;
                }
            }

        }

        match self.status {
            ZipperStatus::Zipping => {
                let Some(peek1) = self.peek1 else {
                    panic!("peek1 is None");
                };
                let Some(peek2) = self.peek2 else {
                    panic!("peek2 is None");
                };
                if peek1 < peek2 {
                    self.peek1 = None;
                    Some(peek1)
                } else {
                    self.peek2 = None;
                    Some(peek2)
                }
            },
            ZipperStatus::Iter1Done => {
                let Some(peek2) = self.peek2 else {
                    panic!("peek2 is None");
                };
                self.peek2 = None;
                Some(peek2)
            },
            ZipperStatus::Iter2Done => {
                let Some(peek1) = self.peek1 else {
                    panic!("peek1 is None");
                };
                self.peek1 = None;
                Some(peek1)

            },
            ZipperStatus::Done => None
        }
    }
}
fn main() {
    let mut stopwatch = Stopwatch::new();
    stopwatch.start();
    let input = get_input(25, 2).unwrap();
    let input = parse_input(&input);
    println!("Input parsed ({})", stopwatch.lap().report());
    println!("1. {} ({})", part1(&input), stopwatch.lap().report());
    println!("2. {} ({})", part2(&input), stopwatch.lap().report());
    println!("Total: {}", stopwatch.stop().report());
}

fn get_digits(n: u64) -> u32 {
    n.ilog10() + 1
}

fn parse_input(input: &str) -> Input {
    input
        .get_numbers()
        .tuples()
        .flat_map(|(lo, hi)| {
            let lo_digits = get_digits(lo);
            let hi_digits = get_digits(hi);
            (lo_digits..=hi_digits).map(move |digits| {
                let lo = max(lo, 10u64.pow(digits - 1));
                let hi = min(hi, 10u64.pow(digits) - 1);
                (lo, hi)
            })
        })
        .collect()
}

fn count_invalid_1(lo: u64, hi: u64) -> u64 {
    let digits = get_digits(lo);
    if digits & 1 == 1 {
        return 0;
    }
    InvalidIds::new(lo, hi, digits, 2).into_iter().sum()
}

fn count_invalid_2(lo: u64, hi: u64) -> u64 {
    let digits = get_digits(lo);
    match digits {
        1 => 0,
        2 | 4 | 8 => InvalidIds::new(lo, hi, digits, 2).into_iter().sum(),
        3 | 9 => InvalidIds::new(lo, hi, digits, 3).into_iter().sum(),
        5 | 7 | 11 => InvalidIds::new(lo, hi, digits, digits).into_iter().sum(),
        6 | 10 => {
            let halves = InvalidIds::new(lo, hi, digits, 2);
            let others = InvalidIds::new(lo, hi, digits, digits / 2);
            ZipAndSort::new(
                halves.into_iter(),
                others.into_iter(),
            )
                .into_iter()
                .dedup()
                .sum()
        },
        _ => panic!("This solver only goes to 11 digits!"),
    }
}

fn solve<F>(ids: &Input, count_invalid: F) -> Output
where
    F: Fn(u64, u64) -> u64,
{
    ids.iter()
        .map(|&(lo, hi)| count_invalid(lo, hi))
        .sum()
}

fn part1(ids: &Input) -> Output {
    solve(ids, count_invalid_1)
}

fn part2(ids: &Input) -> Output {
    solve(ids, count_invalid_2)
}

#[test]
fn default() {
    let input = get_input(25, 2).unwrap();
    let input = parse_input(&input);
    assert_eq!(28846518423, part1(&input));
    assert_eq!(31578210022, part2(&input));
}

#[test]
fn test1() {
    let input = r"11-22,95-115,998-1012,1188511880-1188511890,222220-222224,
1698522-1698528,446443-446449,38593856-38593862,565653-565659,
824824821-824824827,2121212118-2121212124";
    let input = parse_input(input);
    assert_eq!(1227775554, part1(&input));
}

#[test]
fn test2() {
    let input = r"11-22,95-115,998-1012,1188511880-1188511890,222220-222224,
1698522-1698528,446443-446449,38593856-38593862,565653-565659,
824824821-824824827,2121212118-2121212124";
    let input = parse_input(input);
    assert_eq!(4174379265, part2(&input));
}

// Input parsed (21μs)
// 1. 28846518423 (8μs)
// 2. 31578210022 (9μs)
// Total: 40μs