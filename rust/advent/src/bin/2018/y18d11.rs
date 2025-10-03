use advent::utilities::get_input::get_input;
use utilities::structs::{grid::Grid2, stopwatch::{ReportDuration, Stopwatch}};

type Input = Grid2<i64>;
type Output = String;
const LENGTH: usize = 300;

fn main() {
    let mut stopwatch = Stopwatch::new();
    stopwatch.start();
    let input = get_input(18, 11).unwrap();
    let input = parse_input(&input);
    println!("Input parsed ({})", stopwatch.lap().report());
    println!("1. {} ({})", part1(&input), stopwatch.lap().report());
    println!("2. {} ({})", part2(&input), stopwatch.lap().report());
    println!("Total: {}", stopwatch.stop().report());
}

fn parse_input(input: &str) -> Input {
    let serial_number: usize = input.trim_end().parse().unwrap();
    Grid2::new2d_with_fn(LENGTH, LENGTH, |i| {
        let x = (i % LENGTH) + 1;
        let y = (i / LENGTH) + 1;
        let rack_id = x + 10;
        ((((rack_id * y + serial_number) * rack_id) % 1000) / 100) as i64 - 5
    })
}

#[derive(Debug, Copy, Clone)]
struct Square {
    x: i64,
    y: i64,
    size: i64,
    power: i64,
}

fn solve(cells: &Input, smallest: usize, largest: usize) -> Square {
    // create a "working" grid that starts as a copy of the "cells" grid. But after each pass of a row, each 
    // cell in that row is updated to include cells from lower rows in accordance with the size of the squares
    // being evaluated. This way we can avoid re-summing when evaluating larger squares.
    let mut grid = cells.clone();

    // Track the square that has the largest power. Default is zero.
    let mut max = Square { x: 0, y: 0, size: 0, power: 0 };

    // Outer loop gradually increases the size of the square to be evaluated up to "largest." Starts at 1
    // rather than "smallest" because even if sums are not calculated, the grid needs to be updated for later 
    // passes.
    for size in 1..=largest {
        // First nested loop runs through all the rows to be evaluated. The grid update process grabs cell 
        // values from lower rows so this maxes out at the length minus the size.
        for y in 0..= LENGTH - size {
            // The Grid class stores data in a 1-D array so this grabs all the values in that row for easy access
            let row: Vec<i64> = grid.row(y).unwrap().copied().collect();

            // Only do summing activity if the square size is at least as large as "smallest."
            if size >= smallest {
                // start with the left-most cell. Because of grid updating grabbing values from lower rows, 
                // this is always the sum of all cells below it that are part of the square
                let mut power: i64 = row[..size].iter().sum();

                // move right with a "windowed" movement, adding the next to the right and subtracting the last
                // from the left
                for x in size..row.len() {
                    // update max if needed 
                    if power > max.power {
                        max = Square { x: x as i64 - size as i64 + 1, y: y as i64 + 1, size: size as i64, power }
                    } 

                    // calculate power for next square in row
                    power += row[x] - row[x - size]
                }
            }
            // update grid by adding the row with row + size
            if y < LENGTH - size {
                for x in 0..row.len() {
                    grid[y * LENGTH + x] += cells[(y + size) * LENGTH + x];
                }
            }
        }
    }
    max
}

fn part1(input: &Input) -> Output {
    let square = solve(input, 3, 3);
    format!("{},{}", square.x, square.y)
}

fn part2(input: &Input) -> Output {
    let square = solve(input, 1, 300);
    format!("{},{},{}", square.x, square.y, square.size)
}

#[test]
fn default() {
    let input = get_input(18, 11).unwrap();
    let input = parse_input(&input);
    assert_eq!("235,48".to_string(), part1(&input));
    assert_eq!("285,113,11".to_string(), part2(&input));
}

// Input parsed (456μs)
// 1. 235,48 (432μs)
// 2. 285,113,11 (8ms)
// Total: 9ms