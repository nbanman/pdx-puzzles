use std::iter::successors;

use advent::utilities::get_input::get_input;
use utilities::structs::{grid::Grid2, stopwatch::{ReportDuration, Stopwatch}};

type Input = Grid2<bool>;
type Output = usize;

fn main() {
    let mut stopwatch = Stopwatch::new();
    stopwatch.start();
    let input = get_input(15, 18).unwrap();
    let input = parse_input(&input);
    println!("Input parsed ({})", stopwatch.lap().report());
    println!("1. {} ({})", part1(&input), stopwatch.lap().report());
    println!("2. {} ({})", part2(&input), stopwatch.lap().report());
    println!("Total: {}", stopwatch.stop().report());
}

fn parse_input(input: &str) -> Input {
    Grid2::new2d_map_str(input, |c| c == '#').unwrap()
}

fn iterate(lights: &Input, corners_stuck: bool) -> Input {
    let corners: Option<[usize; 4]> = if corners_stuck {
        Some([0, lights.width() - 1, lights.len() - lights.width(), lights.len() - 1])
    } else {
        None
    };

    Grid2::new2d_with_fn(lights.width(), lights.height(), |i| {
        if let Some(corners) = corners && corners.contains(&i) {
            true
        } else {
            let neighbors_on = lights
                .adjacent(i, true)
                .unwrap()
                .filter(|adj| *adj.value)
                .count();
            neighbors_on == 3 || (neighbors_on == 2 && lights[i])
        }
    })
}

fn solve(lights: &Input, corners_stuck: bool) -> Output {
    successors(Some(lights.clone()), |it| Some(iterate(it, corners_stuck)))
        .take(101)
        .last()
        .unwrap()
        .iter()
        .filter(|&&it| it)
        .count()
}

fn part1(lights: &Input) -> Output {
    solve(lights, false)
}

fn part2(lights: &Input) -> Output {
    solve(lights, true)
}

#[test]
fn default() {
    let input = get_input(15, 18).unwrap();
    let input = parse_input(&input);
    assert_eq!(1061, part1(&input));
    assert_eq!(1006, part2(&input));
}

// Input parsed (72μs)
// 1. 1061 (108ms)
// 2. 1006 (106ms)
// Total: 214ms
