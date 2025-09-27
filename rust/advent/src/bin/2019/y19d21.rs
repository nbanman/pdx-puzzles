use advent::utilities::{get_input::get_input, intcode::IntCode};
use utilities::{parsing::get_numbers::ContainsNumbers, structs::stopwatch::{ReportDuration, Stopwatch}};

type Input = Vec<i64>;
type Output = i64;

fn main() {
    let mut stopwatch = Stopwatch::new();
    stopwatch.start();
    let input = get_input(19, 21).unwrap();
    let input = parse_input(&input);
    println!("Input parsed ({})", stopwatch.lap().report());
    println!("1. {} ({})", part1(&input), stopwatch.lap().report());
    println!("2. {} ({})", part2(&input), stopwatch.lap().report());
    println!("Total: {}", stopwatch.stop().report());
}

fn parse_input(input: &str) -> Input {
    input.get_numbers().collect()
}

fn execute(input: &Input, commands: &str) -> Output {
    let mut answer = 0;
    let mut ic = IntCode::new(input);
    ic.run();
    for command in commands.split('\n') {
        ic.input_ascii(command);
        ic.input(10);
        let (_, values) = ic.run_while_able();
        if !values.is_empty() {
            answer = values[values.len() - 1];
        }
    }
    answer
}

fn part1(input: &Input) -> Output {
    let commands = r"NOT A T
NOT C J
OR T J
AND D J
WALK";
    execute(input, commands)
}

fn part2(input: &Input) -> Output {
    let commands = r"OR A J
AND B J  
AND C J  
NOT J J  
AND D J  
OR E T  
OR H T  
AND T J  
RUN";
    execute(input, commands)
}

#[test]
fn default() {
    let input = get_input(19, 21).unwrap();
    let input = parse_input(&input);
    assert_eq!(19349530, part1(&input));
    assert_eq!(1142805439, part2(&input));
}

// Input parsed (60μs)
// 1. 19349530 (163μs)
// 2. 1142805439 (3ms)
// Total: 3ms
