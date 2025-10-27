use advent::utilities::get_input::get_input;
use utilities::{parsing::get_numbers::ContainsNumbers, structs::stopwatch::{ReportDuration, Stopwatch}};

type Input = Vec<Vec<usize>>;
type Output = usize;

fn main() {
    let mut stopwatch = Stopwatch::new();
    stopwatch.start();
    let input = get_input(15, 17).unwrap();
    let input = parse_input(&input);
    println!("Input parsed ({})", stopwatch.lap().report());
    println!("1. {} ({})", part1(&input), stopwatch.lap().report());
    println!("2. {} ({})", part2(&input), stopwatch.lap().report());
    println!("Total: {}", stopwatch.stop().report());
}

fn parse_input(input: &str) -> Input {
    let storage = 150;
    let containers: Vec<usize> = input.get_numbers().collect();
    let mut combos: Input = Vec::new();

    for container in containers {
        for index in 0..combos.len() {
            let combo = combos.get(index).unwrap();
            if combo.iter().sum::<usize>() + container <= storage {
                combos.push(combo.iter().copied().chain(std::iter::once(container)).collect());
            }
        }
        combos.push(vec![container]);
    }
    
    combos.into_iter()
        .filter(|combo| combo.iter().sum::<usize>() == storage)
        .collect()
}

fn part1(containers: &Input) -> Output {
    containers.len()
}

fn part2(containers: &Input) -> Output {
    let mininmum_containers = containers.iter()
        .map(|container| container.len())
        .min()
        .unwrap();
    containers.iter()
        .filter(|container| container.len() == mininmum_containers)
        .count()
}

#[test]
fn default() {
    let input = get_input(15, 17).unwrap();
    let input = parse_input(&input);
    assert_eq!(1638, part1(&input));
    assert_eq!(17, part2(&input));
}

// Input parsed (3ms)
// 1. 1638 (8μs)
// 2. 17 (2μs)
// Total: 3ms
