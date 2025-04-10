use advent::utilities::get_input::get_input;
use utilities::structs::{coord::Coord2, stopwatch::{ReportDuration, Stopwatch}};

type Input = Vec<(char, Int)>;
type Output = Int;
type Int = i64;
type Pos = Coord2;

fn main() {
    let mut stopwatch = Stopwatch::new();
    stopwatch.start();
    let input = get_input(21, 2).unwrap();
    let input = parse_input(&input);
    println!("Input parsed ({})", stopwatch.lap().report());
    println!("1. {} ({})", part1(&input), stopwatch.lap().report());
    println!("2. {} ({})", part2(&input), stopwatch.lap().report());
    println!("Total: {}", stopwatch.stop().report());
}

fn parse_input(input: &str) -> Input {
    input
        .lines()
        .map(|line| {
            let (command, amt) = line.split_once(' ').unwrap();
            (command.chars().next().unwrap(), amt.parse().unwrap())  
        })
        .collect()
}

fn solve<F>(commands: &Input, f: F) -> Output
where 
    F: Fn((Pos, Int), &(char, Int)) -> (Pos, Int),
{
    let (pos, _) = commands.iter()
        .fold((Pos::origin(), 0), f);
    pos.x() * pos.y()
}

fn part1(commands: &Input) -> Output {
    solve(commands, |(pos, _), (dir, amt)| {
        let dir = match &dir {
            'f' => Pos::new2d(pos.x() + amt, pos.y()),
            'u' => Pos::new2d(pos.x(), pos.y() - amt), 
            'd' => Pos::new2d(pos.x(), pos.y() + amt),
            _ => panic!("invalid command"),
        };
        (dir, 0)
    })
}

fn part2(commands: &Input) -> Output {
    solve(commands, |(pos, aim), (dir, amt)| {
        match &dir {
            'f' => (Pos::new2d(pos.x() + amt, pos.y() + aim * amt), aim),
            'u' => (pos, aim - *amt), 
            'd' => (pos, aim + *amt),
            _ => panic!("invalid command"),
        }
    })
}

#[test]
fn default() {
    let input = get_input(21, 2).unwrap();
    let input = parse_input(&input);
    assert_eq!(2117664, part1(&input));
    assert_eq!(2073416724, part2(&input));
}

// Input parsed (42μs)
// 1. 2117664 (15μs)
// 2. 2073416724 (8μs)
// Total: 68μs