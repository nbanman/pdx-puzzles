use advent::utilities::get_input::get_input;
use advent_ocr::ocr;
use itertools::Itertools;
use utilities::{parsing::get_numbers::ContainsNumbers, structs::{coord::Coord2U, grid::Grid2, stopwatch::{ReportDuration, Stopwatch}}};

type Input = Grid2<bool>;
type Pos = Coord2U;

fn main() {
    let mut stopwatch = Stopwatch::new();
    stopwatch.start();
    let input = get_input(16, 8).unwrap();
    let input = parse_input(&input);
    println!("Input parsed ({})", stopwatch.lap().report());
    println!("1. {} ({})", part1(&input), stopwatch.lap().report());
    println!("2. {} ({})", part2(&input), stopwatch.lap().report());
    println!("Total: {}", stopwatch.stop().report());
}

fn parse_input(input: &str) -> Input {
    let mut screen = Grid2::new2d(vec![false; 50 * 6], 50).unwrap();
    for line in input.lines() {
        let (n1, n2): (usize, usize) = line.get_numbers().collect_tuple().unwrap();
        if line.starts_with("rect") {
            Pos::for_rectangle(Pos::origin(), Pos::new2d(n1 - 1, n2 - 1), |pos| {
                screen[pos] = true;
            });
        } else if line.starts_with("rotate column") {
            let mut column: Vec<bool> = screen.column(n1).unwrap().copied().collect();
            column.rotate_right(n2);
            for (y, b) in column.into_iter().enumerate() {
                screen[Pos::new2d(n1, y)] = b;
            }
        } else if line.starts_with("rotate row") {
            let mut row: Vec<bool> = screen.row(n1).unwrap().copied().collect();
            row.rotate_right(n2);
            for (x, b) in row.into_iter().enumerate() {
                screen[Pos::new2d(x, n1)] = b;
            }
        } else {
            panic!("Unknown line: {}", line);
        }
    }
    screen
}

fn part1(screen: &Input) -> usize {
    screen.iter().filter(|&&pixel| pixel).count()
}

fn part2(screen: &Input) -> String {
    ocr(screen).unwrap()
}

#[test]
fn default() {
    let input = get_input(16, 8).unwrap();
    let input = parse_input(&input);
    assert_eq!(123, part1(&input));
    assert_eq!("AFBUPZBJPS".to_string(), part2(&input));
}

// Input parsed (1ms)
// 1. 123 (8μs)
// 2. AFBUPZBJPS (133μs)
// Total: 1ms
