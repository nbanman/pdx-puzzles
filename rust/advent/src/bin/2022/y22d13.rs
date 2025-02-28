use std::cmp::Ordering;

use advent::utilities::get_input::get_input;
use itertools::Itertools;
use lazy_regex::regex;
use utilities::structs::stopwatch::{ReportDuration, Stopwatch};

type Input = Vec<Packet>;
type Output = usize;

#[derive(Debug, PartialEq, Eq)]
enum Packet {
    List(Vec<Packet>),
    Value(Value),
}

#[derive(Copy, Clone, Debug, PartialEq, Eq, PartialOrd, Ord)]
struct Value(usize);

impl Value {
    fn to_packet_list(&self) -> Packet {
        let list = vec![Packet::Value(*self)];
        Packet::List(list)
    }
}

impl Ord for Packet {
    fn cmp(&self, other: &Self) -> std::cmp::Ordering {
        match self {
            Packet::List(a_list) => match other {
                Packet::List(b_list) => {
                    for (a_item, b_item) in a_list.iter().zip(b_list.iter()) {
                        match a_item.cmp(b_item) {
                            Ordering::Equal => {
                                continue;
                            }
                            eval => {
                                return eval;
                            }
                        }
                    }
                    a_list.len().cmp(&b_list.len())
                }
                Packet::Value(b_val) => self.cmp(&b_val.to_packet_list()),
            },
            Packet::Value(a_val) => match other {
                Packet::List(_) => a_val.to_packet_list().cmp(&other),
                Packet::Value(b_val) => a_val.0.cmp(&b_val.0),
            },
        }
    }
}

impl PartialOrd for Packet {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        Some(self.cmp(other))
    }
}

impl Packet {
    /**
     * Factory method. Finds all relevant tokens, uses a consumable iterator so that makePacket() can go
     * recursive without losing its place.
     */
    fn new(line: &str) -> Self {
        let rx = regex!(r"\[|]|\d+");
        let mut iterator = rx.find_iter(line).map(|m| m.as_str());
        Self::make_packet(&mut iterator)
    }

    fn make_packet<'a>(iterator: &mut impl Iterator<Item = &'a str>) -> Packet {
        let mut list = Vec::new();
        while let Some(next) = iterator.next() {
            match next {
                "[" => list.push(Self::make_packet(iterator)),
                "]" => {
                    break;
                }
                next => list.push(Packet::Value(Value(next.parse().unwrap()))),
            }
        }
        Self::List(list)
    }
}

fn main() {
    let mut stopwatch = Stopwatch::new();
    stopwatch.start();
    let input = get_input(22, 13).unwrap();
    let input = parse_input(&input);
    println!("Input parsed ({})", stopwatch.lap().report());
    println!("1. {} ({})", part1(&input), stopwatch.lap().report());
    println!("2. {} ({})", part2(&input), stopwatch.lap().report());
    println!("Total: {}", stopwatch.stop().report());
}

fn parse_input(input: &str) -> Input {
    input
        .lines()
        .filter(|line| !line.is_empty())
        .map(Packet::new)
        .collect()
}

fn part1(packets: &Input) -> Output {
    packets
        .iter()
        .tuples::<(_, _)>()
        .enumerate()
        .map(|(index, (a, b))| if a < b { index + 1 } else { 0 })
        .sum()
}

fn part2(packets: &Input) -> Output {
    let divider_packets = [Packet::new("[[2]]"), Packet::new("[[6]]")];

    divider_packets
        .iter()
        .enumerate()
        .map(|(index, packet)| packets.iter().filter(|&other| packet > other).count() + index + 1)
        .reduce(std::ops::Mul::mul)
        .unwrap()
}

#[test]
fn default() {
    let input = get_input(22, 13).unwrap();
    let input = parse_input(&input);
    assert_eq!(5506, part1(&input));
    assert_eq!(21756, part2(&input));
}

// Input parsed (1ms)
// 1. 5506 (12μs)
// 2. 21756 (17μs)
// Total: 1ms
