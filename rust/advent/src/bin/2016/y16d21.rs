use advent::utilities::get_input::get_input;
use itertools::Itertools;
use utilities::structs::stopwatch::{ReportDuration, Stopwatch};

type Input = Vec<Command>;
type Output = String;

fn main() {
    let mut stopwatch = Stopwatch::new();
    stopwatch.start();
    let input = get_input(16, 21).unwrap();
    let input = parse_input(&input);
    println!("Input parsed ({})", stopwatch.lap().report());
    println!("1. {} ({})", part1(&input), stopwatch.lap().report());
    println!("2. {} ({})", part2(&input), stopwatch.lap().report());
    println!("Total: {}", stopwatch.stop().report());
}

#[derive(Debug, Copy, Clone)]
enum Command {
    Move(usize, usize),
    SwapPos(usize, usize),
    SwapLetter(char, char),
    Reverse(usize, usize),
    RotateRight(usize),
    RotateLeft(usize),
    RotateBased(char),
}

impl Command {
    fn execute(self, passwd: &mut[u8], reversed: bool) {
        match self {
            Command::Move(x, y) => {
                let (x, y) = if reversed {
                    (y, x)
                } else {
                    (x, y)
                };
                let mut moved = Vec::with_capacity(passwd.len());
                let xb = passwd[x];
                if x < y {
                    moved.extend_from_slice(&passwd[..x]);
                    moved.extend_from_slice(&passwd[x + 1..y + 1]);
                    moved.push(xb);
                    moved.extend_from_slice(&passwd[y + 1..]);
                } else {
                    moved.extend_from_slice(&passwd[..y]);
                    moved.push(xb);
                    moved.extend_from_slice(&passwd[y..x]);
                    moved.extend_from_slice(&passwd[x + 1..]);
                }
                passwd.copy_from_slice(&moved);
            },
            Command::SwapPos(x, y) => passwd.swap(x, y),
            Command::SwapLetter(x, y) => {
                for b in passwd.iter_mut() {
                    if *b == x as u8 {
                        *b = y as u8;
                    } else if *b == y as u8 {
                        *b = x as u8;
                    }
                }
            },
            Command::Reverse(x, y) => { passwd[x..=y].reverse(); },
            Command::RotateRight(x) => {
                if reversed {
                    passwd.rotate_left(x);
                } else {
                    passwd.rotate_right(x);
                }
            },
            Command::RotateLeft(x) => {
                if reversed {
                    passwd.rotate_right(x);
                } else {
                    passwd.rotate_left(x);
                }
            },
            Command::RotateBased(x) => {
                let index = passwd.iter().position(|&b| b == x as u8).unwrap();
                if reversed {
                    let rotations = if index & 1 == 1 || index == 0 { 1 } else { 5 };
                    let rotations = rotations + index / 2;
                    passwd.rotate_left(rotations % passwd.len());
                } else {
                    let rotations = if index >= 4 { 1 } else { 0 };
                    let rotations = rotations + index + 1;
                    passwd.rotate_right(rotations % passwd.len());
                }
            },
        }
    }
}

impl From<&str> for Command {
    fn from(value: &str) -> Self {
        let mut tokens = value.split(' ');
        match tokens.next().unwrap() {
            "swap" => {
                let (pos_or_let, x, _, _, y) = tokens.collect_tuple().unwrap();
                match pos_or_let {
                    "position" => Self::SwapPos(x.parse().unwrap(), y.parse().unwrap()),
                    "letter" => Self::SwapLetter(x.chars().next().unwrap(), y.chars().next().unwrap()),
                    _ => unreachable!(),
                }
            },
            "rotate" => {
                match tokens.next().unwrap() {
                    "left" => Self::RotateLeft(tokens.next().unwrap().parse().unwrap()),
                    "right" => Self::RotateRight(tokens.next().unwrap().parse().unwrap()),
                    "based" => {
                        let letter = tokens.last().unwrap().chars().next().unwrap();
                        Self::RotateBased(letter)
                    },
                    _ => unreachable!(),
                }
            },
            "reverse" => {
                let (_, x, _, y) = tokens.collect_tuple().unwrap();
                Self::Reverse(x.parse().unwrap(), y.parse().unwrap())
            },
            "move" => {
                let (_, x, _, _, y) = tokens.collect_tuple().unwrap();
                Self::Move(x.parse().unwrap(), y.parse().unwrap())
                
            },
            _ => unreachable!(),
        }
    }
}

fn parse_input(input: &str) -> Input {
    input.lines().map(Command::from).collect()
}

fn part1(commands: &Input) -> Output {
    let mut passwd: Vec<u8> = "abcdefgh".as_bytes().iter().copied().collect();
    for command in commands {
        command.execute(&mut passwd, false);
    }
    passwd.into_iter().map(|b| b as char).collect()
}

fn part2(commands: &Input) -> Output {
    let mut passwd: Vec<u8> = "fbgdceah".as_bytes().iter().copied().collect();
    for command in commands.iter().rev() {
        command.execute(&mut passwd, true);
    }
    passwd.into_iter().map(|b| b as char).collect()
}

#[test]
fn default() {
    let input = get_input(16, 21).unwrap();
    let input = parse_input(&input);
    assert_eq!("bfheacgd".to_string(), part1(&input));
    assert_eq!("gcehdbfa".to_string(), part2(&input));
}

// Input parsed (30μs)
// 1. bfheacgd (9μs)
// 2. gcehdbfa (6μs)
// Total: 48μs