use advent::utilities::get_input::get_input;
use advent_ocr::ocr;
use utilities::structs::stopwatch::{ReportDuration, Stopwatch};

type Input = Vec<(usize, isize)>;

fn main() {
    let mut stopwatch = Stopwatch::new();
    stopwatch.start();
    let input = get_input(22, 10).unwrap();
    let input = parse_input(&input);
    println!("Input parsed ({})", stopwatch.lap().report());
    println!("1. {} ({})", part1(&input), stopwatch.lap().report());
    println!("2. {} ({})", part2(&input), stopwatch.lap().report());
    println!("Total: {}", stopwatch.stop().report());
}

fn parse_input(input: &str) -> Input {
    input
        .trim_end()
        .split(['\n', ' '])
        .map(|word| {
            match word {
                "addx" | "noop" => 0,
                d => d.parse::<isize>().unwrap()
            }
        })
        .scan(1isize, |state, x| {
            let old_state = state.clone();
            *state += x;
            Some(old_state)
        })
        .enumerate()
        .collect()
}

fn part1(cpu: &Input) -> isize {
    cpu.iter()
        .filter(|(cycle, _)| (cycle + 19) % 40 == 0)
        .map(|(cycle, register)| (cycle + 1) as isize * register)
        .sum()
}

fn part2(cpu: &Input) -> String {
    let mut s = String::new();
    cpu.into_iter()
        .take(240)
        .map(|(cycle, register)| {
            ((register - 1)..=(register + 1)).contains(&(*cycle as isize % 40))
        })
        .enumerate()
        .for_each(|(index, on)| {
            if index != 0 && index % 40 == 0 {
                s.push('\n');
            }
            if on {
                s.push('*');
            } else {
                s.push('.');
            }
        });
    ocr(&*s).unwrap()
}

#[test]
fn default() {
    let input = get_input(22, 10).unwrap();
    let input = parse_input(&input);
    assert_eq!(16406, part1(&input));
    assert_eq!("ZKJFBJFZ".to_string(), part2(&input));
}

// Input parsed (17μs)
// 1. 16406 (4μs)
// 2. ZKJFBJFZ (12μs)
// Total: 36μs