use rayon::iter::ParallelIterator;
use advent::utilities::get_input::get_input;
use itertools::Itertools;
use std::fmt::{Display, Formatter};
use std::ops::Add;
use rayon::iter::IntoParallelRefIterator;
use utilities::structs::stopwatch::{ReportDuration, Stopwatch};

type Input = Vec<Snailfish>;
type Output = usize;

fn main() {
    let mut stopwatch = Stopwatch::new();
    stopwatch.start();
    let input = get_input(21, 18).unwrap();
    let input = parse_input(&input);
    println!("Input parsed ({})", stopwatch.lap().report());
    println!("1. {} ({})", part1(input.clone()), stopwatch.lap().report());
    println!("2. {} ({})", part2(input), stopwatch.lap().report());
    println!("Total: {}", stopwatch.stop().report());
}

#[derive(Debug, Clone)]
enum Snailfish {
    Number(u8),
    Pair {
        left: Box<Snailfish>,
        right: Box<Snailfish>,
    }
}

impl Snailfish {
    fn magnitude(&self) -> usize {
        match self {
            Snailfish::Number(v) => *v as usize,
            Snailfish::Pair { left, right } => {
                left.magnitude() * 3 + right.magnitude() * 2
            },
        }
    }

    fn reduce(&mut self) {
        loop {
            loop {
                if self.explode(1) == ExplodeStatus::Nothing { break; }
            }
            if !self.split() { break; }
        }
    }

    fn split(&mut self) -> bool {
        match self {
            Snailfish::Number(v) => {
                if *v >= 10 {
                    let left_v = *v / 2;
                    let right_v = left_v + if *v & 1 == 1 { 1 } else { 0 };
                    *self = Self::Pair {
                        left: Box::new(Self::Number(left_v)),
                        right: Box::new(Self::Number(right_v))
                    };
                    true
                } else {
                    false
                }
            },
            Snailfish::Pair { left, right } => {
                left.split() || right.split()
            },
        }
    }
    
    fn explode(&mut self, level: u8) -> ExplodeStatus {
        match self {
            Snailfish::Number(_) => {
                ExplodeStatus::Nothing
            },
            Snailfish::Pair { left, right } => {
                if level >= 5 {
                    let left_v = left.value();
                    let right_v = right.value();
                    if let Some(left_v) = left_v && let Some(right_v) = right_v {
                        *self = Self::Number(0);
                        return ExplodeStatus::Place(left_v, right_v);
                    }
                }
                let left_status = left.explode(level + 1);
                match left_status {
                    ExplodeStatus::PlaceLeft(_) => {
                        return if level == 1 {
                            self.explode(1)
                        } else {
                            left_status
                        };
                    },
                    ExplodeStatus::PlaceRight(to_place) => {
                        right.place_left(to_place);
                        return if level == 1 {
                            self.explode(1)
                        } else {
                            ExplodeStatus::Reduced
                        };
                    },
                    ExplodeStatus::Place(place_left, place_right) => {
                        right.place_left(place_right);
                        return if level == 1 {
                            self.explode(1)
                        } else {
                            ExplodeStatus::PlaceLeft(place_left)
                        };
                    },
                    ExplodeStatus::Reduced => {
                        return if level == 1 {
                            let test = self.explode(1);
                            return test;
                        } else {
                            ExplodeStatus::Reduced
                        }
                    },
                    ExplodeStatus::Nothing => {}, // continue to right...
                }
                let right_status = right.explode(level + 1);
                match right_status {
                    ExplodeStatus::PlaceLeft(to_place) => {
                        left.place_right(to_place);
                        if level == 1 {
                            self.explode(1)
                        } else {
                            ExplodeStatus::Reduced
                        }
                    },
                    ExplodeStatus::PlaceRight(_) => {
                        if level == 1 {
                            self.explode(1)
                        } else {
                            right_status
                        }
                    },
                    ExplodeStatus::Place(place_left, place_right) => {
                        left.place_right(place_left);
                        if level == 1 {
                            self.explode(1)
                        } else {
                            ExplodeStatus::PlaceRight(place_right)
                        }
                    },
                    ExplodeStatus::Reduced => {
                        if level == 1 {
                            self.explode(1)
                        } else {
                            ExplodeStatus::Reduced
                        }
                    },
                    ExplodeStatus::Nothing => {
                        ExplodeStatus::Nothing
                    },
                }
            },
        }
    }

    fn value(&self) -> Option<u8> {
        match self {
            Snailfish::Number(v) => Some(*v),
            Snailfish::Pair { .. } => None,
        }
    }

    fn place_left(&mut self, to_place: u8) {
        match self {
            Snailfish::Number(v) => {
                *v += to_place;
            },
            Snailfish::Pair { left, .. } => { left.place_left(to_place); },
        }
    }

    fn place_right(&mut self, to_place: u8) {
        match self {
            Snailfish::Number(v) => {
                *v += to_place;
            },
            Snailfish::Pair { right, .. } => { right.place_right(to_place); },
        }
    }
    
}

impl Add<Snailfish> for Snailfish {
    type Output = Self;

    fn add(self, rhs: Snailfish) -> Self::Output {
        let left = Box::new(self);
        let right = Box::new(rhs);
        let mut added = Self::Pair { left, right };
        added.reduce();
        added
    }
}

impl Display for Snailfish {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        match self {
            Snailfish::Number(v) => write!(f, "{v}"),
            Snailfish::Pair { left, right } => write!(f, "[{left},{right}]"),
        }
    }
}

impl From<&str> for Snailfish {
    fn from(s: &str) -> Self {
        let mut bytes = s.as_bytes().into_iter();
        Self::from(&mut bytes)
    }
}

impl From<&mut std::slice::Iter<'_, u8>> for Snailfish {
    fn from(bytes: &mut std::slice::Iter<'_, u8>) -> Self {
        let next = *bytes.next().unwrap() as char;
        match next {
            '[' => {
                // Pair
                let left = Self::from(&mut *bytes);
                bytes.next();
                let right = Self::from(&mut *bytes);
                bytes.next();
                Self::Pair { left: Box::new(left), right: Box::new(right) }
            },
            c if c.is_ascii_digit() => {
                Self::Number(c as u8 - 48)
            }
            c => panic!("invalid byte {}", c),
        }
    }
}

#[derive(Debug, Clone, Copy, Eq, PartialEq)]
enum ExplodeStatus {
    PlaceLeft(u8),
    PlaceRight(u8),
    Place(u8, u8),
    Reduced,
    Nothing,
}

fn parse_input(input: &str) -> Input {
    input.lines().map(|it| it.into()).collect()
}

fn part1(snailfish: Input) -> Output {
    snailfish.into_iter().reduce(Snailfish::add).unwrap().magnitude()
}

fn part2(snailfish: Input) -> Output {
    let combos = snailfish.iter().permutations(2)
        .collect_vec();
    combos
        .par_iter()
        .map(|combo| (combo[0].clone() + combo[1].clone()).magnitude())
        .max()
        .unwrap()
}

#[test]
fn default() {
    let input = get_input(21, 18).unwrap();
    let input = parse_input(&input);
    assert_eq!(3806, part1(input.clone()));
    assert_eq!(4727, part2(input));
}

// Input parsed (72μs)
// 1. 3806 (1ms)
// 2. 4727 (5ms)
// Total: 7ms
