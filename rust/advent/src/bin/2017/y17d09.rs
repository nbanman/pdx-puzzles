use advent::utilities::get_input::get_input;
use utilities::structs::stopwatch::{ReportDuration, Stopwatch};

type Input = (usize, usize);
type Output = usize;

fn main() {
    let mut stopwatch = Stopwatch::new();
    stopwatch.start();
    let input = get_input(17, 9).unwrap();
    let input = parse_input(&input);
    println!("Input parsed ({})", stopwatch.lap().report());
    println!("1. {} ({})", part1(&input), stopwatch.lap().report());
    println!("2. {} ({})", part2(&input), stopwatch.lap().report());
    println!("Total: {}", stopwatch.stop().report());
}

fn parse_input(input: &str) -> Input {
    let mut in_garbage = false;
    let mut garbage = 0;
    let mut depth = 0;
    let mut score = 0;
    let mut last = b' ';
    for &b in input.as_bytes() {
        if in_garbage {
            if b == b'>' && last != b'!' {
                in_garbage = false;
            }
            if !(b == b'!' || b == b'>' || last == b'!') {
                garbage += 1;
            }
            last = if last == b'!' { b' ' } else { b };
        } else {
            match b {
                b'<' => {
                    in_garbage = true;
                }
                b'{' => {
                    depth += 1;
                    score += depth;
                }
                b'}' => {
                    depth -= 1;
                }
                _ => {}
            }
        }
    }
    (score, garbage)
}

fn part1(input: &Input) -> Output {
    input.0
}

fn part2(input: &Input) -> Output {
    input.1
}

#[test]
fn default() {
    let input = get_input(17, 9).unwrap();
    let input = parse_input(&input);
    assert_eq!(9251, part1(&input));
    assert_eq!(4322, part2(&input));
}

// Input parsed (219μs)
// 1. 9251 (4μs)
// 2. 4322 (1μs)
// Total: 227μs
