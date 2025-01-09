use std::collections::HashMap;

use advent::utilities::get_input::get_input;
use utilities::structs::stopwatch::{ReportDuration, Stopwatch};

type Input = Vec<Vec<char>>;
type Output = usize;

fn main() {
    let mut stopwatch = Stopwatch::new();
    stopwatch.start();
    let input = get_input(23, 14).unwrap();
    let input = parse_input(&input);
    println!("Input parsed ({})", stopwatch.lap().report());
    println!("1. {} ({})", part1(&input), stopwatch.lap().report());
    println!("2. {} ({})", part2(&input), stopwatch.lap().report());
    println!("Total: {}", stopwatch.stop().report());
}

fn parse_input(input: &str) -> Input {
    input.lines().map(|line| line.chars().collect()).collect()
}

fn load(rocks: &Input) -> usize {
    rocks.iter().enumerate()
        .map(|(index, row)| {
            (rocks.len() - index) * row.iter().filter(|c| **c == 'O').count()
        })
        .sum()
}

fn tilt_up(rocks: &Input) -> Vec<Vec<char>> {
    let mut tilted = vec![vec!['.'; rocks.len()]; rocks.len()];

    rocks.into_iter().enumerate().for_each(|(y, row)| {
        row.into_iter().enumerate().for_each(|(x, c)| {
            match c {
                '#' => tilted[y][x] = '#',
                'O' => {
                    for yy in (0..=y).rev() {
                        let next = yy as isize - 1;
                        if next < 0 || "#O".contains(tilted[next as usize][x]) {
                            tilted[yy][x] = 'O';
                            break;
                        }
                    }
                }
                _ => {}
            }
        })
    });
    tilted
}

fn spin_cycle(rocks: &Input) -> Vec<Vec<char>> {
    (1..=4).fold(rocks.clone(), |acc, _| {
        rotate(&tilt_up(&acc))
    })
}

fn rotate(rocks: &Input) -> Vec<Vec<char>> {
    let mut rotated = vec![vec!['.'; rocks.len()]; rocks.len()];
    rocks.iter().enumerate().for_each(|(y, row)| {
        row.iter().enumerate().for_each(|(x, c)| {
            if *c != '.' {
                rotated[x][rocks.len() - 1 - y] = c.to_owned();
            }
        })
    });
    rotated
}

fn part1(rocks: &Input) -> Output {
    load(&tilt_up(rocks))
}

fn part2(initial: &Input) -> Output {
    let mut rock_formations = HashMap::new();
    let mut rocks = initial.clone();
    let mut index = 0_usize;
    let first_index_of_cycle = loop {
        if let Some(first_index_of_cycle) = rock_formations.insert(rocks.clone(), index) {
            break first_index_of_cycle;
        }
        rocks = spin_cycle(&rocks);
        index += 1;
    };
    let cycle_space = 1_000_000_000 - first_index_of_cycle;
    let cycle_length = rock_formations.len() - first_index_of_cycle;
    let answer = rock_formations.iter()
        .find(|(_, v)| {
            **v == first_index_of_cycle + cycle_space % cycle_length
        }).unwrap().0;
    load(answer)
}

#[test]
fn default() {
    let input = get_input(23, 14).unwrap();
    let input = parse_input(&input);
    assert_eq!(106990, part1(&input));
    assert_eq!(100531, part2(&input));
}

    // Input parsed (41μs)
    // 1. 106990 (65μs)
    // 2. 100531 (45ms)
    // Total: 45ms