use std::iter::successors;
use indexmap::IndexMap;
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

impl Floor {
    fn is_empty(&self) -> bool {
        self.microchips == 0 && self.generators == 0
    }

    fn is_valid(&self) -> bool {
        self.microchips == 0 || self.generators == 0 || self.generators >= self.microchips
    }
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

    fn is_valid(&self) -> bool {
        self.floors.iter().all(|floor| floor.is_valid())
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
            if potential_elevator == 0 && potential_floor.is_empty() { continue; }
            if potential_elevator == 1
                && self.elevator == 2
                && potential_floor.is_empty()
                && self.floors[0].is_empty()
            {
                continue;
            }

            // add matched pair case. Matched pairs can always be moved together unless there are
            // unmatched chips on the floor to be moved.
            let can_leave = cur_floor.microchips > 0 && cur_floor.generators > 0;
            let can_gain = can_leave && potential_floor.microchips <= potential_floor.generators;
            if can_gain {
                valid_moves.push(
                    self.generate_next(potential_elevator, 1, 1)
                );
            }

            let mut added_one_chip = false;

            // add one chip case. A chip can be moved when the floor to move to has no generators,
            // or has more generators then chips. This latter case means that the chip matches with
            // the spare generator.
            let can_leave = cur_floor.microchips > 0;
            let can_gain = cur_floor.generators == 0 && potential_floor.generators > potential_floor.microchips;
            let can_gain = can_gain || potential_floor.generators == 0;
            if can_leave && can_gain {
                if potential_floor.generators == 0
                    || potential_floor.generators > potential_floor.microchips
                {
                    added_one_chip = true;
                    valid_moves.push(
                        self.generate_next(potential_elevator, 1, 0)
                    );
                }
            }

            // add two chip case. Two chips can be moved when the floor to move to has no generators,
            // or has at least two more generators than chips.
            if !added_one_chip || potential_elevator > self.elevator || true {
                let can_leave = cur_floor.microchips > 1;
                let can_gain = cur_floor.generators == 0 && potential_floor.generators > potential_floor.microchips + 1;
                let can_gain = can_gain || potential_floor.generators == 0;
                if can_gain && can_leave
                {
                    if potential_elevator > self.elevator && added_one_chip {
                        valid_moves.pop();
                    }
                    valid_moves.push(
                        self.generate_next(potential_elevator, 2, 0)
                    );
                }
            }

            let mut added_one_gen = false;

            // add one gen case. A floor that loses a gen is valid if that was the only generator,
            // because no gens == no problems. If there is another gen, that is a problem because
            // that implies an unmatched chip that will fry. A floor that gains a gen is valid if
            // it has no microchips (nothing to fry), or there are already enough gens for all
            // microchips (all chips are protected),
            if cur_floor.generators > 0 {
                let can_leave = cur_floor.generators == 1
                    || cur_floor.generators > cur_floor.microchips;
                let can_gain = can_leave && (potential_floor.microchips == 0
                    || potential_floor.generators >= potential_floor.microchips - 1
                    || potential_floor.microchips == 1
                );
                if can_gain {
                    added_one_gen = true;
                    valid_moves.push(
                        self.generate_next(potential_elevator, 0, 1)
                    );
                }
            }

            // add two gen case. A floor that loses two gens is valid if they are the only
            // generators, because no gens == no problems.  A floor that gains two gens is valid if
            // it has no microchips (nothing to fry), or there are already enough gens for all
            // microchips (all chips are protected),
            if (!added_one_gen || potential_elevator > self.elevator) && cur_floor.generators > 1 {
                let can_leave = cur_floor.generators == 2
                    || cur_floor.generators > cur_floor.microchips + 1;
                let can_gain = can_leave && (potential_floor.microchips == 0
                    || potential_floor.generators >= potential_floor.microchips
                    || potential_floor.microchips == 2
                );
                if can_gain {
                    if potential_elevator > self.elevator && added_one_gen {
                        valid_moves.pop();
                    }
                    valid_moves.push(
                        self.generate_next(potential_elevator, 0, 2)
                    );
                }
            }
        }
        valid_moves
    }

    fn generate_next(&self, next_elevator: usize, microchips: u8, generators: u8) -> FloorState {
        let new_floors: [Floor; 4] = std::array::from_fn(|i| {
            let cur = self.floors[i];
            match i {
                i if i == self.elevator => Floor {
                    microchips: cur.microchips - microchips,
                    generators: cur.generators - generators,
                },
                i if i == next_elevator => Floor {
                    microchips: cur.microchips + microchips,
                    generators: cur.generators + generators,
                },
                _ => cur,
            }
        });
        let fs = FloorState { elevator: next_elevator, floors: new_floors };
        if !fs.is_valid() {
            panic!("invalid state: {:?}", fs);
        }
        fs
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
    let mut visited = IndexMap::new();
    visited.insert(initial_state.clone(), 0);
    let mut todo = Vec::new();
    let mut next = Vec::new();
    next.push((initial_state, 0));
    loop {
        steps += 1;
        std::mem::swap(&mut todo, &mut next);
        for (state, parent) in todo.drain( .. ) {
            println!("e: {}, 0: {}:{}, 1: {}:{}, 2: {}:{}, 3: {}:{}",
                    state.elevator,
                    state.floors[0].microchips,
                    state.floors[0].generators,
                    state.floors[1].microchips,
                    state.floors[1].generators,
                    state.floors[2].microchips,
                    state.floors[2].generators,
                    state.floors[3].microchips,
                    state.floors[3].generators,
            );
            for neighbor in state.next() {
                if !visited.contains_key(&neighbor) {
                    if neighbor.is_solved() {
                        let mut path = Vec::new();
                        let last = format!("e: {}, 0: {}:{}, 1: {}:{}, 2: {}:{}, 3: {}:{}",
                                 neighbor.elevator,
                                           neighbor.floors[0].microchips,
                                           neighbor.floors[0].generators,
                                           neighbor.floors[1].microchips,
                                           neighbor.floors[1].generators,
                                           neighbor.floors[2].microchips,
                                           neighbor.floors[2].generators,
                                           neighbor.floors[3].microchips,
                                           neighbor.floors[3].generators,
                        );
                        path.push(last);
                        let path_it = successors(Some((state, parent)), |(state, parent)| {
                            if *parent == 0 {
                                None
                            } else {
                                let (new_state, &new_parent) = visited.get_index(*parent).unwrap();
                                Some((new_state.clone(), new_parent))
                            }
                        }).map(|(state, _)| state);
                        for neighbor in path_it {
                            path.push(
                                format!("e: {}, 0: {}:{}, 1: {}:{}, 2: {}:{}, 3: {}:{}",
                                        neighbor.elevator,
                                        neighbor.floors[0].microchips,
                                        neighbor.floors[0].generators,
                                        neighbor.floors[1].microchips,
                                        neighbor.floors[1].generators,
                                        neighbor.floors[2].microchips,
                                        neighbor.floors[2].generators,
                                        neighbor.floors[3].microchips,
                                        neighbor.floors[3].generators,
                                )
                            );
                        }
                        let neighbor = visited.get_index(0).unwrap().0;
                        let first = format!("e: {}, 0: {}:{}, 1: {}:{}, 2: {}:{}, 3: {}:{}",
                                 neighbor.elevator,
                                neighbor.floors[0].microchips,
                                neighbor.floors[0].generators,
                                neighbor.floors[1].microchips,
                                neighbor.floors[1].generators,
                                neighbor.floors[2].microchips,
                                neighbor.floors[2].generators,
                                neighbor.floors[3].microchips,
                                neighbor.floors[3].generators,
                        );
                        path.push(first);
                        println!("All done! Path:\n");
                        for s in path.into_iter().rev() {
                            println!("{}", s);
                        }
                        return steps
                    }
                    visited.insert(neighbor.clone(), parent);
                    next.push((neighbor, visited.len() - 1));
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

#[test]
fn other() {
    let input = r"The first floor contains a promethium generator and a promethium-compatible microchip.
The second floor contains a cobalt generator, a curium generator, a ruthenium generator, and a plutonium generator.
The third floor contains a cobalt-compatible microchip, a curium-compatible microchip, a ruthenium-compatible microchip, and a plutonium-compatible microchip.
The fourth floor contains nothing relevant.";
    let input = parse_input(input);
    assert_eq!(33, part1(input.clone()));
    assert_eq!(57, part2(input));
}

// Input parsed (17μs)
// 1. 47 (1ms)
// 2. 71 (5ms)
// Total: 7ms
