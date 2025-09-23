use advent::utilities::{get_input::get_input, intcode::{IntCode, State}};
use utilities::{enums::cardinals::Cardinal, structs::{coord::Coord2U, grid::Grid2, stopwatch::{ReportDuration, Stopwatch}}};

type Input = Grid2<Sector>;
type Output = usize;
type Pos = Coord2U;

fn main() {
    let mut stopwatch = Stopwatch::new();
    stopwatch.start();
    let input = get_input(19, 15).unwrap();
    let input = parse_input(&input);
    println!("Input parsed ({})", stopwatch.lap().report());
    println!("1. {} ({})", part1(&input), stopwatch.lap().report());
    println!("2. {} ({})", part2(&input), stopwatch.lap().report());
    println!("Total: {}", stopwatch.stop().report());
}

#[derive(Debug, Copy, Clone)]
enum Sector { Unexplored, Wall, Path, O2}

fn parse_input(input: &str) -> Input {
    let mut compy = IntCode::from(input);
    let mut ship = Grid2::new2d_with_fn(1_000, 1_000, |_| Sector::Unexplored);
    let start_pos = Pos::new2d(500, 500);
    ship[start_pos] = Sector::Path;
    let mut pos = start_pos;
    let mut dir = Cardinal::North;
    
    loop {
        compy.input(dir_to_input(dir));
        let State::Output(output) = compy.run() else {
            panic!("Computer failed to return output")
        };
        let attempted_pos = pos.move_direction(dir, 1).unwrap();
        match output_to_sector(output) {
            Sector::Wall => {
                ship[attempted_pos] = Sector::Wall;
                dir = dir.right();
            },
            Sector::Path => {
                pos = attempted_pos;
                ship[pos] = Sector::Path;
            },
            Sector::O2 => {
                pos = attempted_pos;
                ship[pos] = Sector::O2;
            }
            Sector::Unexplored => { panic!("Computer returned \"unexplored\""); },
        }
        if pos == start_pos { break; }
    }
    ship
}

fn dir_to_input(dir: Cardinal) -> i64 {
    match dir {
        Cardinal::North => 1,
        Cardinal::East => 2,
        Cardinal::South => 3,
        Cardinal::West => 4,
    }
}

fn output_to_sector(output: i64) -> Sector {
    match output {
        0 => Sector::Wall,
        1 => Sector::Path,
        2 => Sector::O2,
        d => { panic!("{d} is not a valid output"); },
    }
}

fn part1(input: &Input) -> Output {

    todo!()
}

fn part2(input: &Input) -> Output {

    todo!()
}

#[test]
fn default() {
    let input = get_input(19, 15).unwrap();
    let input = parse_input(&input);
    assert_eq!(250, part1(&input));
    // assert_eq!(332, part2(&input));
}

