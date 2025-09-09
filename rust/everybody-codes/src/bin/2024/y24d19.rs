use everybody_codes::utilities::inputs::get_event_inputs;
use itertools::Itertools;
use utilities::structs::coord::Coord2U;
use utilities::structs::grid::Grid2;
use utilities::structs::stopwatch::{ReportDuration, Stopwatch};

type Input<'a> = &'a str;
type Pos = Coord2U;

fn main() {
    let mut stopwatch = Stopwatch::new();
    stopwatch.start();
    let (input1, input2, input3) = get_event_inputs(24, 19);
    println!("Input parsed ({})", stopwatch.lap().report());
    println!("1. {} ({})", part1(&input1), stopwatch.lap().report());
    println!("2. {} ({})", part2(&input2), stopwatch.lap().report());
    println!("3. {} ({})", part3(&input3), stopwatch.lap().report());
    println!("Total: {}", stopwatch.stop().report());
}

fn part1(input: Input) -> String {
    solve(input, 1)
}

fn part2(input: Input) -> String {
    solve(input, 100)
}

fn part3(input: Input) -> String {
    solve(input, 1048576000)
}

fn solve(input: Input, rounds: usize) -> String {
    let (grid, ops) = parse_input(input);
    let cycles = get_cycles(&grid, ops, rounds);
    cycle_grid(&grid, &cycles, rounds)
        .into_iter()
        .skip_while(|&c| c != '>')
        .dropping(1)
        .take_while(|&c| c != '<')
        .collect()

}

fn parse_input(input: Input<'_>) -> (Grid2<char>, &str) {
    let (ops, grid) = input.split_once("\n\n").unwrap();
    let grid = Grid2::try_from(grid).unwrap();
    (grid, ops)
}

fn get_cycles(grid: &Grid2<char>, ops: &str, rounds: usize) -> Vec<Vec<usize>> {
    let mut ops = ops.as_bytes().iter().cycle();
    let mut lookup = Grid2::new2d_with_fn(grid.width(), grid.height(), |i| i);
    let y = grid.width() as isize;
    let x = 1isize;
    let rl: Vec<(isize, isize)> = vec![-x - y, -y, x - y, x, x + y, y, y - x, -x]
        .into_iter()
        .tuple_windows()
        .collect();
    let rr: Vec<(isize, isize)> = vec![-x - y, -x, y - x, y, x + y, x, x - y, -y]
        .into_iter()
        .tuple_windows()
        .collect();
    for row in 1..grid.height() - 1 {
        for col in 1..grid.width() - 1 {
            let pos = grid.index_of(Pos::new2d(col, row)).unwrap() as isize;
            let tl = lookup[(pos - x - y) as usize];
            match ops.next().unwrap() {
                b'L' => {
                    for &(a, b) in rl.iter() {
                        lookup[(pos + a) as usize] = lookup[(pos + b) as usize];
                    }
                    lookup[(pos - x) as usize] = tl;
                }
                b'R' => {
                    for &(a, b) in rr.iter() {
                        lookup[(pos + a) as usize] = lookup[(pos + b) as usize];
                    }
                    lookup[(pos - y) as usize] = tl;
                }
                _ => {}
            }
        }
    }

    // generate cycles
    let mut cycles = Vec::new();
    for pos in 0..grid.len() {
        let mut cycle = Vec::new();
        let mut current = pos;
        let mut seen = vec![false; grid.len()];
        for _ in 0..=rounds {
            if seen[current] {
                break;
            }
            seen[current] = true;
            cycle.push(current);
            current = lookup[current];
        }
        cycles.push(cycle);
    }
    cycles
}

fn cycle_grid(grid: &Grid2<char>, cycles: &Vec<Vec<usize>>, rounds: usize) -> Grid2<char> {
    Grid2::new2d_with_fn(grid.width(), grid.height(), |pos| {
        let cycle = &cycles[pos];
        let index = cycle[rounds % cycle.len()];
        grid[index]
    })
}


#[test]
fn default() {
    let (input1, input2, _input3) = get_event_inputs(24, 19);
    assert_eq!("8762334189768578".to_string(), part1(&input1));
    assert_eq!("6795785362142233", part2(&input2));
    // assert_eq!(ZZ, part3(&input3));
}

#[test]
fn examples() {
    let inputs = [r"LR

>-IN-
-----
W---<", r"RRLL

A.VI..>...T
.CC...<...O
.....EIB.R.
.DHB...YF..
.....F..G..
D.H........"];
    assert_eq!("WIN".to_string(), part1(inputs[0]));
    assert_eq!("VICTORY".to_string(), part2(inputs[1]));
}

// Input parsed (44μs)
// 1. 8762334189768578 (27μs)
// 2. 6795785362142233 (533μs)
// 3. 8629494914828132 (644ms)
// Total: 645ms