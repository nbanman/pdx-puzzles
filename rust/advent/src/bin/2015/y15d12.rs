use advent::utilities::get_input::get_input;
use utilities::{parsing::get_numbers::ContainsNumbers, structs::stopwatch::{ReportDuration, Stopwatch}};

type Input<'a> = &'a str;
type Output = i64;
type Block = (i64, usize);

const DIVIDERS: [u8; 4] = [b'[', b']', b'{', b'}'];

fn main() {
    let mut stopwatch = Stopwatch::new();
    stopwatch.start();
    let input = get_input(15, 12).unwrap();
    println!("Input parsed ({})", stopwatch.lap().report());
    println!("1. {} ({})", part1(&input), stopwatch.lap().report());
    println!("2. {} ({})", part2(&input), stopwatch.lap().report());
    println!("Total: {}", stopwatch.stop().report());
}

fn get_block(doc: &[u8], start: usize, already_red: bool) -> Block {
    let mut index = start;
    if already_red {
        // run a simplified algorithm that counts nesting blocks without worrying about
        // numbers, outputting a Block with value 0
        let mut depth = 0;
        while depth >= 0 {
            index += 1;
            match doc[index] {
                b'[' | b'{' => { depth += 1; },
                b']' | b'}' => { depth -= 1; },
                _ => {},
            }
        }
        return (0, index)
    } else {
        let mut is_red = false;
        let mut value: i64 = 0;

        // keep looping until the block's closing bracket is found.
        loop {
            let end_index = doc[index + 1..].iter()
                .position(|b| DIVIDERS.contains(b)).unwrap() + index + 1;
            let snippet = unsafe { std::str::from_utf8_unchecked(&doc[index..end_index]) };

            // add value of any numbers in snippet, or to flag as red if :"Red" is found in snippet
            if !is_red {
                if snippet.contains(r#":"red""#) {
                    is_red = true;
                    value = 0;
                } else {
                    value += snippet.get_numbers::<i64>().sum::<i64>();
                }
            }

            let end = doc[end_index];
            if end == b']' || end == b'}' { // if the block closes...
                return (value, end_index);
            } else { // ...else if a nesting block exists
                let (inner_value, inner_index) = get_block(doc, end_index, is_red);
                value += inner_value;
                index = inner_index;
            }
        }
    }
}

fn part1(input: Input) -> Output {
    input.get_numbers::<i64>().sum()
}

fn part2(input: Input) -> Output {
    get_block(input.as_bytes(), 0, false).0
}

#[test]
fn default() {
    let input = get_input(15, 12).unwrap();
    assert_eq!(111754, part1(&input));
    assert_eq!(65402, part2(&input));
}

// Input parsed (30μs)
// 1. 111754 (50μs)
// 2. 65402 (76μs)
// Total: 159μs