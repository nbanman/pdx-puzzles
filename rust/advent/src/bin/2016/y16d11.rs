use advent::utilities::get_input::get_input;
use itertools::Itertools;
use rustc_hash::FxHashSet;
use utilities::structs::stopwatch::{ReportDuration, Stopwatch};

type Input = FloorState;
type Output = usize;

fn main() {
    let mut stopwatch = Stopwatch::new();
    stopwatch.start();
    let input = get_input(16, 11).unwrap();
    let input = parse_input(&input);
    println!("Input parsed ({})", stopwatch.lap().report());
    println!("1. {} ({})", part1(input.clone()), stopwatch.lap().report());
    println!("2. {} ({})", part2(input), stopwatch.lap().report());
    println!("Total: {}", stopwatch.stop().report());
}

#[derive(Debug, Copy, Clone, PartialEq, Eq, Hash)]
struct Floor {
    microchips: u8,
    generators: u8,
}

#[derive(Clone, Debug, PartialEq, Eq, Hash)]
struct FloorState {
    elevator: usize,
    floors: [Floor; 4],
}

impl FloorState {
    fn is_solved(&self) -> bool {
        self.floors.iter().dropping_back(1).all(|floor| floor.microchips == 0 && floor.generators == 0)
    }

    fn next(&self) -> Vec<Self> {
        let mut valid_moves = Vec::new();
        let potential_elevators = match self.elevator {
            0 => vec![1],
            1 => vec![0, 2],
            2 => vec![1, 3],
            _ => vec![2],
        };
        let cur_floor = self.floors[self.elevator];
        
        // Nested loop that tries all valid loads on all potential floors. If the floor with the new load is valid,
        // add it to the list of potential states.
        for potential_elevator in potential_elevators {
            let potential_floor = self.floors[potential_elevator];

            // add matched pair case
            if cur_floor.microchips > 0 && cur_floor.generators > 0 {
                let new_floors: [Floor; 4] = std::array::from_fn(|i| {
                    let cur = self.floors[i];
                    match i {
                        i if i == self.elevator => Floor {
                            microchips: cur.microchips - 1,
                            generators: cur.generators - 1,
                        },
                        i if i == potential_elevator => Floor {
                            microchips: cur.microchips + 1,
                            generators: cur.generators + 1,
                        },
                        _ => cur,
                    }
                });
                valid_moves.push(
                    FloorState { elevator: potential_elevator, floors: new_floors }
                );
            }

            // add one chip case
            if cur_floor.microchips > 0 {
                if potential_floor.generators == 0
                    || potential_floor.generators > potential_floor.microchips
                {
                    let new_floors: [Floor; 4] = std::array::from_fn(|i| {
                        let cur = self.floors[i];
                        match i {
                            i if i == self.elevator => Floor {
                                microchips: cur.microchips - 1,
                                generators: cur.generators,
                            },
                            i if i == potential_elevator => Floor {
                                microchips: cur.microchips + 1,
                                generators: cur.generators,
                            },
                            _ => cur,
                        }
                    });
                    valid_moves.push(
                        FloorState { elevator: potential_elevator, floors: new_floors }
                    );
                }
            }

            // add two chip case
            if cur_floor.microchips > 1 {
                if potential_floor.generators == 0
                    || potential_floor.generators > potential_floor.microchips + 1
                {
                    let new_floors: [Floor; 4] = std::array::from_fn(|i| {
                        let cur = self.floors[i];
                        match i {
                            i if i == self.elevator => Floor {
                                microchips: cur.microchips - 2,
                                generators: cur.generators,
                            },
                            i if i == potential_elevator => Floor {
                                microchips: cur.microchips + 2,
                                generators: cur.generators,
                            },
                            _ => cur,
                        }
                    });
                    valid_moves.push(
                        FloorState { elevator: potential_elevator, floors: new_floors }
                    );
                }
            }

            // add one gen case
            if cur_floor.generators > 0 {
                let can_leave = cur_floor.generators == 1
                    || cur_floor.generators > cur_floor.microchips;
                let can_gain = can_leave && (potential_floor.microchips == 0
                    || potential_floor.generators >= potential_floor.microchips
                    || potential_floor.generators == 0 && potential_floor.microchips == 1);
                if can_gain {
                    let new_floors: [Floor; 4] = std::array::from_fn(|i| {
                        let cur = self.floors[i];
                        match i {
                            i if i == self.elevator => Floor {
                                microchips: cur.microchips,
                                generators: cur.generators - 1,
                            },
                            i if i == potential_elevator => Floor {
                                microchips: cur.microchips,
                                generators: cur.generators + 1,
                            },
                            _ => cur,
                        }
                    });
                    valid_moves.push(
                        FloorState { elevator: potential_elevator, floors: new_floors }
                    );
                }
            }

            // add two gen case
            if cur_floor.generators > 1 {
                let can_leave = cur_floor.generators == 2
                    || cur_floor.generators > cur_floor.microchips + 1;
                let can_gain = can_leave && (potential_floor.microchips == 0
                    || potential_floor.generators >= potential_floor.microchips + 1
                    || potential_floor.generators == 0 && potential_floor.microchips == 2);
                if can_gain {
                    let new_floors: [Floor; 4] = std::array::from_fn(|i| {
                        let cur = self.floors[i];
                        match i {
                            i if i == self.elevator => Floor {
                                microchips: cur.microchips,
                                generators: cur.generators - 2,
                            },
                            i if i == potential_elevator => Floor {
                                microchips: cur.microchips,
                                generators: cur.generators + 2,
                            },
                            _ => cur,
                        }
                    });
                    valid_moves.push(
                        FloorState { elevator: potential_elevator, floors: new_floors }
                    );
                }
            }
        }
        valid_moves
    }
}

fn parse_input(input: &str) -> FloorState {
    let floors: [Floor; 4] = input.lines()
        .map(|line| {
            let microchips = line.matches("microchip").count() as u8;
            let generators = line.matches("generator").count() as u8;
            Floor { microchips, generators }
        })
        .collect_vec()
        .try_into()
        .unwrap();
    FloorState { elevator: 0, floors }
}

fn solve_floors(initial_state: FloorState) -> usize {
    let mut steps = 0;
    
    // cache used to prune previously visited states.
    let mut visited = FxHashSet::default();
    visited.insert(initial_state.clone());
    let mut todo: Vec<FloorState> = Vec::new();
    let mut next: Vec<FloorState> = Vec::new();
    next.push(initial_state);
    loop {
        steps += 1;
        std::mem::swap(&mut todo, &mut next);
        for state in todo.drain( .. ) {
            for neighbor in state.next() {
                if !visited.contains(&neighbor) {
                    if neighbor.is_solved() {
                        return steps
                    }
                    visited.insert(neighbor.clone());
                    next.push(neighbor);
                }
            }
        }
    }
}

fn part1(initial_state: Input) -> Output {
    solve_floors(initial_state)
}

fn part2(mut initial_state: Input) -> Output {
    initial_state.floors[0].microchips += 2;
    initial_state.floors[0].generators += 2;
    solve_floors(initial_state)
}

#[test]
fn default() {
    let input = get_input(16, 11).unwrap();
    let input = parse_input(&input);
    assert_eq!(47, part1(input.clone()));
    assert_eq!(71, part2(input));
}

// Input parsed (17μs)
// 1. 47 (1ms)
// 2. 71 (5ms)
// Total: 7ms
