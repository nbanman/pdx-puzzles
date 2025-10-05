use std::iter::successors;

use advent::utilities::get_input::get_input;
use itertools::Itertools;
use rustc_hash::FxHashMap;
use utilities::{
    enums::cardinals::Cardinal,
    structs::{
        coord::Coord2U,
        grid::Grid2,
        stopwatch::{ReportDuration, Stopwatch},
    },
};

type Output = String;
type Pos = Coord2U;

fn main() {
    let mut stopwatch = Stopwatch::new();
    stopwatch.start();
    let input = get_input(18, 13).unwrap();
    let input = parse_input(&input);
    println!("Input parsed ({})", stopwatch.lap().report());
    println!("1. {} ({})", part1(input.clone()), stopwatch.lap().report());
    println!("2. {} ({})", part2(input), stopwatch.lap().report());
    println!("Total: {}", stopwatch.stop().report());
}

#[derive(Debug, Copy, Clone)]
enum TurnState {
    Left,
    Straight,
    Right,
}

impl TurnState {
    fn advance(&self) -> Self {
        match self {
            TurnState::Left => Self::Straight,
            TurnState::Straight => Self::Right,
            TurnState::Right => Self::Left,
        }
    }

    fn dir(&self, dir: Cardinal) -> Cardinal {
        match self {
            TurnState::Left => dir.left(),
            TurnState::Straight => dir,
            TurnState::Right => dir.right(),
        }
    }
}

#[derive(Debug, Copy, Clone)]
struct Car {
    dir: Cardinal,
    turn_state: TurnState,
}

fn parse_input(input: &str) -> impl Iterator<Item = (FxHashMap<Pos, Car>, Option<Pos>)> + Clone {
    let racetrack = Grid2::try_from(input).unwrap();
    let cars: FxHashMap<Pos, Car> = racetrack
        .iter()
        .enumerate()
        .filter_map(|(i, &c)| {
            match c {
                '^' => Some(Cardinal::North),
                '>' => Some(Cardinal::East),
                'v' => Some(Cardinal::South),
                '<' => Some(Cardinal::West),
                _ => None,
            }
            .map(|dir| {
                (
                    racetrack.coord_of(i).unwrap(),
                    Car {
                        dir,
                        turn_state: TurnState::Left,
                    },
                )
            })
        })
        .collect();
    let initial_state: (FxHashMap<Pos, Car>, Option<Pos>) = (cars, None);

    let race = successors(Some(initial_state), move |(racers, _)| {
        let mut still_racing = racers.clone();

        // Tracks the coordinates of any crashes
        let mut crash_pos: Option<Pos> = None;

        // Moves cars along track. If there's a collision, remove collided cars from map and note coordinates
        for (&pos, &car) in racers.iter().sorted_unstable_by_key(|&(&pos, _)| pos) {
            // break early if there was already a crash at this position
            if crash_pos == Some(pos) {
                continue;
            }

            let new_pos = pos.move_direction(car.dir, 1).unwrap();
            still_racing.remove(&pos);

            if still_racing.contains_key(&new_pos) {
                // if crash...
                crash_pos = Some(new_pos);
                still_racing.remove(&new_pos);
            } else {
                // no crash...
                // place car in new position, adjusting dir and TurnState as appropriate
                match racetrack[new_pos] {
                    '+' => {
                        still_racing.insert(
                            new_pos,
                            Car {
                                dir: car.turn_state.dir(car.dir),
                                turn_state: car.turn_state.advance(),
                            },
                        );
                    }
                    '\\' => {
                        let new_dir = match car.dir {
                            Cardinal::North => Cardinal::West,
                            Cardinal::South => Cardinal::East,
                            Cardinal::East => Cardinal::South,
                            Cardinal::West => Cardinal::North,
                        };
                        still_racing.insert(
                            new_pos,
                            Car {
                                dir: new_dir,
                                ..car
                            },
                        );
                    }
                    '/' => {
                        let new_dir = match car.dir {
                            Cardinal::North => Cardinal::East,
                            Cardinal::South => Cardinal::West,
                            Cardinal::East => Cardinal::North,
                            Cardinal::West => Cardinal::South,
                        };
                        still_racing.insert(
                            new_pos,
                            Car {
                                dir: new_dir,
                                ..car
                            },
                        );
                    }

                    _ => {
                        still_racing.insert(new_pos, car);
                    }
                }
            }
        }
        Some((still_racing, crash_pos))
    });
    race
}

fn part1(race: impl Iterator<Item = (FxHashMap<Pos, Car>, Option<Pos>)>) -> Output {
    race.into_iter()
        .find(|(_, crash)| crash.is_some())
        .map(|(_, crash)| {
            let crash_pos = crash.unwrap();
            format!("{},{}", crash_pos.x(), crash_pos.y())
        })
        .unwrap()
}

fn part2(race: impl Iterator<Item = (FxHashMap<Pos, Car>, Option<Pos>)>) -> Output {
    race.into_iter()
        .find(|(cars, _)| cars.len() == 1)
        .map(|(cars, _)| {
            let pos = cars.keys().into_iter().next().unwrap();
            format!("{},{}", pos.x(), pos.y())
        })
        .unwrap()
}

#[test]
fn default() {
    let input = get_input(18, 13).unwrap();
    let input = parse_input(&input);
    assert_eq!("86,118".to_string(), part1(input.clone()));
    assert_eq!("2,81".to_string(), part2(input));
}

// Input parsed (159μs)
// 1. 86,118 (165μs)
// 2. 2,81 (5ms)
// Total: 5ms
