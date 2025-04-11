use advent::utilities::get_input::get_input;
use itertools::Itertools;
use utilities::{parsing::get_numbers::ContainsNumbers, structs::stopwatch::{ReportDuration, Stopwatch}};

type Int = i64;
type Input = (Vec<Int>, Int, Int);
type Output = Int;

fn main() {
    let mut stopwatch = Stopwatch::new();
    stopwatch.start();
    let input = get_input(21, 7).unwrap();
    let input = parse_input(&input);
    println!("Input parsed ({})", stopwatch.lap().report());
    println!("1. {} ({})", part1(&input), stopwatch.lap().report());
    println!("2. {} ({})", part2(&input), stopwatch.lap().report());
    println!("Total: {}", stopwatch.stop().report());
}

fn parse_input(input: &str) -> Input {
    let crabs: Vec<_> = input.get_numbers().collect();
    let (&min, &max) = crabs.iter().minmax().into_option().unwrap();
    (crabs, min, max)
}

fn optimal_alignment_cost<F>(crabs: &Vec<Int>, mut min: Int, mut max: Int, fuel_cost: F) -> Int 
where F: Fn(Int) -> Int,
{
    while min != max {
        let mp = mid_point(min, max);
        let min_cost = alignment_cost(crabs, mid_point(min, mp), &fuel_cost);
        let max_cost = alignment_cost(crabs, mid_point(mp, max), &fuel_cost);
        if min_cost <= max_cost {
            max = mp;
        } else {
            min = mp;
        }
    }
    alignment_cost(crabs, min, &fuel_cost)
}

fn alignment_cost<F>(crabs: &Vec<Int>, position: Int, fuel_cost: &F) -> Int 
where F: Fn(Int) -> Int,
{
    crabs.iter().map(|&crab| fuel_cost((crab - position).abs())).sum()
}

fn mid_point(min: Int, max: Int) -> Int {
    (max - min) / 2 + min
}

fn part1((crabs, min, max): &Input) -> Output {
    optimal_alignment_cost(crabs, *min, *max, |it| it)
}

fn part2((crabs, min, max): &Input) -> Output {
    optimal_alignment_cost(crabs, *min, *max, |it| {
        (1..=it).sum()
    })
}

#[test]
fn default() {
    let input = get_input(21, 7).unwrap();
    let input = parse_input(&input);
    assert_eq!(343468, part1(&input));
    assert_eq!(96086265, part2(&input));
}

// Input parsed (34μs)
// 1. 343468 (10μs)
// 2. 96086265 (32μs)
// Total: 78μs