use advent::utilities::{
    get_input::get_input,
    intcode::{IntCode, State},
};
use utilities::{
    parsing::get_numbers::ContainsNumbers,
    structs::{
        coord::Coord2U,
        grid::Grid2,
        stopwatch::{ReportDuration, Stopwatch},
    },
};

type Input = Vec<i64>;
type Output = usize;
type Pos = Coord2U;

fn main() {
    let mut stopwatch = Stopwatch::new();
    stopwatch.start();
    let input = get_input(19, 13).unwrap();
    let input = parse_input(&input);
    println!("Input parsed ({})", stopwatch.lap().report());
    println!("1. {} ({})", part1(input.clone()), stopwatch.lap().report());
    println!("2. {} ({})", part2(input), stopwatch.lap().report());
    println!("Total: {}", stopwatch.stop().report());
}

fn parse_input(input: &str) -> Input {
    input.get_numbers().collect()
}

fn part1(input: Input) -> Output {
    let mut pong = IntCode::new(&input);
    let mut screen = Grid2::new2d(vec![0u32; 5000], 100).unwrap();
    while let (State::Output(x), State::Output(y), State::Output(n)) =
        (pong.run(), pong.run(), pong.run())
    {
        let pos = Pos::new2d(x as usize, y as usize);
        screen[pos] = n as u32;
    }
    screen.into_iter().filter(|&pixel| pixel == 2).count()
}

fn part2(mut input: Input) -> Output {
    input[0] = 2;
    let mut pong = IntCode::new(&input);
    let mut screen = Grid2::new2d(vec![0u32; 5000], 100).unwrap();
    let mut score = 0;
    let mut ball_position = Pos::origin();
    let mut paddle_position = Pos::origin();

    loop {
        let (state, mut from_pong) = pong.run_while_able();
        while let (Some(x), Some(y), Some(block)) = (
            from_pong.pop_front(),
            from_pong.pop_front(),
            from_pong.pop_front(),
        ) {
            if x == -1 {
                score = block;
            } else {
                let pos = Pos::new2d(x as usize, y as usize);
                screen[pos] = block as u32;
                if block == 3 {
                    paddle_position = pos;
                } else if block == 4 {
                    ball_position = pos;
                }
            }
        }
        if matches!(state, State::Halted) {
            return score as usize;
        }
        let delta = ball_position.x().checked_sub(paddle_position.x());
        let to_pong = if delta == None {
            -1
        } else if delta == Some(0) {
            0
        } else {
            1
        };
        pong.input(to_pong);
    }
}

#[test]
fn default() {
    let input = get_input(19, 13).unwrap();
    let input = parse_input(&input);
    assert_eq!(348, part1(input.clone()));
    assert_eq!(16999, part2(input));
}

// Input parsed (50μs)
// 1. 348 (175μs)
// 2. 16999 (4ms)
// Total: 4ms
