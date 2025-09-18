use advent::utilities::get_input::get_input;
use utilities::{
    enums::cardinals::Cardinal,
    structs::{
        coord::Coord2,
        stopwatch::{ReportDuration, Stopwatch},
    },
};

type Instruction = (char, Output);
type Input = Vec<Instruction>;
type Output = usize;
type Pos = Coord2;

fn main() {
    let mut stopwatch = Stopwatch::new();
    stopwatch.start();
    let input = get_input(20, 12).unwrap();
    let input = parse_input(&input);
    println!("Input parsed ({})", stopwatch.lap().report());
    println!("1. {} ({})", part1(&input), stopwatch.lap().report());
    println!("2. {} ({})", part2(&input), stopwatch.lap().report());
    println!("Total: {}", stopwatch.stop().report());
}

#[derive(Debug)]
enum ShipState {
    Dir { pos: Pos, dir: Cardinal },
    Waypoint { pos: Pos, waypoint: Pos },
}

impl ShipState {
    fn next_state(&self, (action, amount): Instruction) -> Self {
        match *self {
            ShipState::Dir { pos, dir } => match action {
                'N' => Self::Dir {
                    pos: pos.move_direction(Cardinal::North, amount as i64).unwrap(),
                    dir,
                },
                'S' => Self::Dir {
                    pos: pos.move_direction(Cardinal::South, amount as i64).unwrap(),
                    dir,
                },
                'E' => Self::Dir {
                    pos: pos.move_direction(Cardinal::East, amount as i64).unwrap(),
                    dir,
                },
                'W' => Self::Dir {
                    pos: pos.move_direction(Cardinal::West, amount as i64).unwrap(),
                    dir,
                },
                'F' => Self::Dir {
                    pos: pos.move_direction(dir, amount as i64).unwrap(),
                    dir,
                },
                'L' => Self::Dir {
                    pos,
                    dir: multi_turn(dir, amount, Cardinal::left),
                },
                'R' => Self::Dir {
                    pos,
                    dir: multi_turn(dir, amount, Cardinal::right),
                },
                c => {
                    panic!("Invalid instruction: {c}");
                }
            },
            ShipState::Waypoint { pos, waypoint } => match action {
                'N' => Self::Waypoint {
                    pos,
                    waypoint: waypoint
                        .move_direction(Cardinal::North, amount as i64)
                        .unwrap(),
                },
                'S' => Self::Waypoint {
                    pos,
                    waypoint: waypoint
                        .move_direction(Cardinal::South, amount as i64)
                        .unwrap(),
                },
                'E' => Self::Waypoint {
                    pos,
                    waypoint: waypoint
                        .move_direction(Cardinal::East, amount as i64)
                        .unwrap(),
                },
                'W' => Self::Waypoint {
                    pos,
                    waypoint: waypoint
                        .move_direction(Cardinal::West, amount as i64)
                        .unwrap(),
                },
                'F' => Self::Waypoint {
                    pos: (0..amount).fold(pos, |acc, _| acc + waypoint),
                    waypoint,
                },
                'L' => Self::Waypoint {
                    pos,
                    waypoint: (0..(amount / 90) % 4)
                        .fold(waypoint, |acc, _| Pos::new2d(acc.y(), -acc.x())),
                },
                'R' => Self::Waypoint {
                    pos,
                    waypoint: (0..(amount / 90) % 4)
                        .fold(waypoint, |acc, _| Pos::new2d(-acc.y(), acc.x())),
                },
                c => {
                    panic!("Invalid instruction: {c}");
                }
            },
        }
    }

    fn distance(&self) -> Output {
        let pos = match self {
            ShipState::Dir { pos, dir: _ } => *pos,
            ShipState::Waypoint { pos, waypoint: _ } => *pos,
        };
        pos.manhattan_distance(&Pos::origin())
    }
}

fn multi_turn<F>(dir: Cardinal, amount: Output, turn: F) -> Cardinal
where
    F: Fn(&Cardinal) -> Cardinal,
{
    (0..(amount / 90) % 4).fold(dir, |acc, _| turn(&acc))
}

fn parse_input(input: &str) -> Input {
    input
        .lines()
        .map(|line| {
            let (action, amount) = line.split_at(1);
            let action = action.chars().next().unwrap();
            let amount = amount.parse().unwrap();
            (action, amount)
        })
        .collect()
}

fn solve(instructions: &Input, initial_ship_state: ShipState) -> Output {
    instructions
        .iter()
        .fold(initial_ship_state, |state, &instruction| {
            state.next_state(instruction)
        })
        .distance()
}

fn part1(instructions: &Input) -> Output {
    solve(
        instructions,
        ShipState::Dir {
            pos: Pos::origin(),
            dir: Cardinal::East,
        },
    )
}

fn part2(instructions: &Input) -> Output {
    solve(
        instructions,
        ShipState::Waypoint {
            pos: Pos::origin(),
            waypoint: Pos::new2d(10, -1),
        },
    )
}

#[test]
fn default() {
    let input = get_input(20, 12).unwrap();
    let input = parse_input(&input);
    assert_eq!(2280, part1(&input));
    assert_eq!(38693, part2(&input));
}

// Input parsed (41μs)
// 1. 2280 (15μs)
// 2. 38693 (11μs)
// Total: 71μs
