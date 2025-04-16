use advent::utilities::get_input::get_input;
use utilities::structs::{grid::{Grid, Grid2, GridIterator}, stopwatch::{ReportDuration, Stopwatch}};

type Int = usize;
type Input = Grid2<Int>;
type Output = Int;

fn main() {
    let mut stopwatch = Stopwatch::new();
    stopwatch.start();
    let input = get_input(21, 11).unwrap();
    let input = parse_input(&input);
    println!("Input parsed ({})", stopwatch.lap().report());
    println!("1. {} ({})", part1(input.clone()), stopwatch.lap().report());
    println!("2. {} ({})", part2(input), stopwatch.lap().report());
    println!("Total: {}", stopwatch.stop().report());
}

fn flash<F>(mut cave: Input, terminate: F) -> (Int, Int) 
where F: Fn(Int, Int) -> bool
{
    let mut turn = 0;
    let mut flashes = 0;
    loop {
        for pos in cave.iter_mut() {
            *pos += 1;
        }

        let mut flashed_this_turn = 0;
        
        loop {
            let flasher_indices: Vec<Int> = cave.iter().enumerate()
                .filter(|(_, energy)| **energy > 9)
                .map(|(i, _)| i)
                .collect();

            if flasher_indices.is_empty() {
                break;
            }

            for &index in flasher_indices.iter() {
                flashes += 1;
                flashed_this_turn += 1;
                cave[index] = 0;
            }

            for index in flasher_indices {
                let add_energy: Vec<Int> = cave
                    .adjacent(index, true)
                    .unwrap() 
                    .filter(|adj| *adj.value != 0)
                    .map(|adj| adj.index)
                    .collect();
                for ii in add_energy {
                    cave[ii] += 1;
                }
            }
        }
        turn += 1;
        if terminate(turn, flashed_this_turn) {
            return (turn, flashes)
        }
    }
}

fn parse_input(input: &str) -> Input {
    let cave = Grid::try_from(input).unwrap();
    let width = cave.width();
    cave
        .into_iter()
        .map(|c| c.to_digit(10).unwrap() as Int)
        .try_collect_grid(width)
        .unwrap()
}

fn part1(cave: Input) -> Output {
    flash(cave, |turn, _| turn == 100).1
}

fn part2(cave: Input) -> Output {
    let len = cave.len();
    flash(cave, |_, flashes| flashes == len).0
}

#[test]
fn default() {
    let input = get_input(21, 11).unwrap();
    let input = parse_input(&input);
    assert_eq!(1669, part1(input.clone()));
    assert_eq!(351, part2(input));
}

// Input parsed (33μs)
// 1. 1669 (302μs)
// 2. 351 (931μs)
// Total: 1ms