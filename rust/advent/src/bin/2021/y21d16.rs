use advent::utilities::get_input::get_input;
use itertools::Itertools;
use utilities::structs::stopwatch::{ReportDuration, Stopwatch};

type Input = Packet;
type Output = usize;

fn main() {
    let mut stopwatch = Stopwatch::new();
    stopwatch.start();
    let input = get_input(21, 16).unwrap();
    let input = parse_input(&input);
    println!("Input parsed ({})", stopwatch.lap().report());
    println!("1. {} ({})", part1(&input), stopwatch.lap().report());
    println!("2. {} ({})", part2(&input), stopwatch.lap().report());
    println!("Total: {}", stopwatch.stop().report());
}

#[derive(Debug)]
enum Packet {
    Literal {
        version: u8,
        value: usize,
    },
    Operator {
        version: u8,
        type_id: u8,
        sub_packets: Vec<Box<Packet>>,
    }
}

impl Packet {
    fn value(&self) -> usize {
        match self {
            Packet::Literal { value, .. } => *value,
            Packet::Operator { sub_packets, type_id, .. } => {
                match *type_id {
                    0 => sub_packets.iter().map(|it| it.value()).sum(),
                    1 => sub_packets.iter().map(|it| it.value()).product(),
                    2 => sub_packets.iter().map(|it| it.value()).min().unwrap(),
                    3 => sub_packets.iter().map(|it| it.value()).max().unwrap(),
                    5 | 6 | 7 => {
                        let first = sub_packets[0].value();
                        let last = sub_packets[sub_packets.len() - 1].value();
                        let pred = match *type_id {
                            5 => first > last,
                            6 => first < last,
                            7 => first == last,
                            _ => false,
                        };
                        if pred { 1 } else { 0 }
                    },
                    _ => 0,
                }
            },
        }
    }

    fn version_sum(&self) -> usize {
        match self {
            Packet::Literal { version, .. } => *version as usize,
            Packet::Operator { version, sub_packets, .. } => {
                *version as usize + sub_packets.iter().map(|it| it.version_sum()).sum::<usize>()
            },
        }
    }
}

impl From<&mut BitProvider> for Packet {
    fn from(bp: &mut BitProvider) -> Self {
        let version = bp.get_int(3) as u8;
        let type_id = bp.get_int(3) as u8;
        if type_id == 4 {
            let mut value = 0;
            while bp.get_int(1) == 1 {
                value = value << 4;
                value += bp.get_int(4) as usize;
            }
            value = value << 4;
            value += bp.get_int(4) as usize;
            Self::Literal { version, value }
        } else {
            let length_id = bp.get_int(1);
            let length = bp.get_int(if length_id == 0 { 15 } else { 11 }) as usize;
            let sub_packets = match length_id {
                0 => {
                    let mut sub_bp = bp.sub_bp(length);
                    let mut sub_packets = Vec::new();
                    while sub_bp.len() > 0 {
                        let sub_packet = Packet::from(&mut sub_bp);
                        sub_packets.push(Box::new(sub_packet));
                    }
                    sub_packets
                },
                1 => {
                    let mut sub_packets = Vec::new();
                    for _ in 0..length {
                        sub_packets.push(Box::new(Packet::from(&mut *bp)));
                    }
                    sub_packets
                },
                _ => unreachable!()
            };
            Self::Operator { version, type_id, sub_packets }
        }
    }
}

#[derive(Debug)]
struct BitProvider {
    binary: Vec<bool>,
    parser: usize,
}

impl BitProvider {
    fn new(binary: Vec<bool>) -> Self {
        Self { binary, parser: 0 }
    }

    fn len(&self) -> usize {
        self.binary.len().checked_sub(self.parser).unwrap_or_default()
    }

    fn get_int(&mut self, bits: usize) -> u32 {
        let int = self.binary[self.parser..self.parser + bits].iter()
            .fold(0, |acc, &b| (acc << 1) + b as u32);
        self.parser += bits;
        int
    }

    fn sub_bp(&mut self, length: usize) -> Self {
        let slice = &self.binary[self.parser..self.parser + length];
        self.parser += length;
        Self::from(slice)
    }
}

impl From<&str> for BitProvider {
    fn from(value: &str) -> Self {
        let mut binary = Vec::with_capacity(value.len() * 4);
        for &hex in value.as_bytes().iter() {
            let hex = if hex.is_ascii_uppercase() {
                hex - 55
            } else {
                hex - 48
            };
            let bits = (0..4).rev().map(|i| (hex >> i) & 1 == 1);
            binary.extend(bits);
        }
        Self::new(binary)
    }
}

impl From<&[bool]> for BitProvider {
    fn from(value: &[bool]) -> Self {
        let binary = value.iter().copied().collect_vec();
        Self::new(binary)
    }
}

fn parse_input(input: &str) -> Input {
    let mut bp: BitProvider = input.into();
    Packet::from(&mut bp)
}

fn part1(packet: &Input) -> Output {
    packet.version_sum()
}

fn part2(packet: &Input) -> Output {
    packet.value()
}

#[test]
fn default() {
    let input = get_input(21, 16).unwrap();
    let input = parse_input(&input);
    assert_eq!(979, part1(&input));
    assert_eq!(277110354175, part2(&input));
}

// Input parsed (43μs)
// 1. 979 (6μs)
// 2. 277110354175 (4μs)
// Total: 57μs