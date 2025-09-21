use std::collections::HashMap;

use advent::utilities::{get_input::get_input, intcode::{IntCode, State}};
use advent_ocr::ocr;
use itertools::Itertools;
use utilities::{enums::cardinals::Cardinal, structs::{coord::Coord2, stopwatch::{ReportDuration, Stopwatch}}};

type Input = IntCode;
type Pos = Coord2;

fn main() {
    let mut stopwatch = Stopwatch::new();
    stopwatch.start();
    let input = get_input(19, 11).unwrap();
    let input = parse_input(&input);
    println!("Input parsed ({})", stopwatch.lap().report());
    println!("1. {} ({})", part1(input.clone()), stopwatch.lap().report());
    println!("2. {} ({})", part2(input), stopwatch.lap().report());
    println!("Total: {}", stopwatch.stop().report());
}

fn parse_input(input: &str) -> Input {
    IntCode::from(input)
}

#[derive(Debug, Copy, Clone, PartialEq, Eq)]
#[repr(u8)]
enum Panel { Black, White }

fn run_robot(mut robot: Input, first_panel: Panel) -> HashMap<Pos, Panel> {
    let mut grid = HashMap::new();
    grid.insert(Pos::origin(), first_panel);
    let mut pos = Pos::origin();
    let mut dir = Cardinal::North;
    robot.input(first_panel as i64);
    while let (State::Output(new_panel), State::Output(new_dir)) = (robot.run(), robot.run()) {
        let new_panel = if new_panel == 0 { Panel::Black } else { Panel::White };
        grid.insert(pos, new_panel);
        dir = if new_dir == 0 { dir.left() } else { dir.right() };
        pos = pos.move_direction(dir, 1).unwrap();
        let input = grid.get(&pos)
            .map(|&panel| {
                if panel == Panel::Black { 0 } else { 1 }
            })
            .unwrap_or_default();
        robot.input(input);
    }
    grid
}

fn part1(input: Input) -> usize {
    let grid = run_robot(input, Panel::Black);
    grid.values().len()
}

fn part2(input: Input) -> String {
    let grid = run_robot(input, Panel::White);
    let (min_x, max_x) = grid.keys()
        .minmax_by_key(|pos| pos.x())
        .into_option()
        .map(|(min, max)| (min.x(), max.x()))
        .unwrap();
    let (min_y, max_y) = grid.keys()
        .minmax_by_key(|pos| pos.y())
        .into_option()
        .map(|(min, max)| (min.y(), max.y()))
        .unwrap();
    let width = max_x - min_x + 1;
    let height = max_y - min_y + 1;

    let mut screen = String::new();
    for y in 0..height {
        for x in 0..width {
            let c = grid.get(&Pos::new2d(x + min_x, y + min_y))
                .map(|&panel| if panel == Panel::Black { '.' } else { '#' })
                .unwrap_or('.');
            screen.push(c);
        }
        screen.push('\n');
    }
    ocr(screen.as_str()).unwrap()
}

#[test]
fn default() {
    let input = get_input(19, 11).unwrap();
    let input = parse_input(&input);
    assert_eq!(2720, part1(input.clone()));
    assert_eq!("JZPJRAGJ".to_string(), part2(input));
}

// Input parsed (35μs)
// 1. 2720 (1ms)
// 2. JZPJRAGJ (95μs)
// Total: 1ms