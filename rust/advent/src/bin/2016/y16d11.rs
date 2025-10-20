use advent::utilities::get_input::get_input;
use indexmap::IndexSet;
use itertools::Itertools;
use lazy_regex::regex;
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

    fn next(&self, items: u8) -> Vec<Self> {
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
            let pot_flr_unmatched_chips = potential_floor.microchips & !potential_floor.generators;
            if pot_flr_unmatched_chips == 0 {
                let matchy = cur_floor.microchips & cur_floor.generators;
                for n in 0..items {
                    if (matchy >> n) & 1 == 1 {
                        let diff = 1 << n;
                        let new_floors: [Floor; 4] = std::array::from_fn(|i| {
                            let cur = self.floors[i];
                            match i {
                                i if i == self.elevator => Floor {
                                    microchips: cur.microchips - diff,
                                    generators: cur.generators - diff,
                                },
                                i if i == potential_elevator => Floor {
                                    microchips: cur.microchips + diff,
                                    generators: cur.generators + diff,
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
            let pot_flr_no_chips = potential_floor.microchips == 0;
            let cur_flr_unmatched_chips = cur_floor.generators & !cur_floor.generators;
            let pot_flr_no_gen = potential_floor.generators == 0;
            let pot_chips = if pot_flr_no_gen {
                cur_floor.microchips
            } else {
                cur_floor.microchips & potential_floor.generators
            };
            let mut pot_gens = if cur_flr_unmatched_chips == 0 {
                cur_floor.generators
            } else {
                cur_floor.generators & !cur_floor.microchips
            };
            if !pot_flr_no_chips {
                if pot_flr_unmatched_chips.count_ones() > 1 {
                    pot_gens = 0;
                } else {
                    pot_gens = pot_gens & potential_floor.microchips;
                }
            }
            for (item1, item2) in (0..items * 2).tuple_combinations() {
                let mut chip_change = 0;
                let mut gen_change = 0;
                if item1 < items {
                    if (pot_chips >> item1) & 1 == 1 {
                        chip_change |= 1 << item1;
                    }
                } else {
                    if (pot_gens >> (item1 - items)) & 1== 1 {
                        gen_change |= 1 << (item1 - items);
                    }
                }
                if item2 < items {
                    if (pot_chips >> item2) & 1 == 1 {
                        chip_change |= 1 << item2;
                    }
                } else {
                    if (pot_gens >> (item2 - items)) & 1== 1 {
                        gen_change |= 1 << (item2 - items);
                    }
                }
                if chip_change + gen_change > 0 {
                    let new_floors: [Floor; 4] = std::array::from_fn(|i| {
                        let cur = self.floors[i];
                        match i {
                            i if i == self.elevator => Floor {
                                microchips: cur.microchips - chip_change,
                                generators: cur.generators - gen_change,
                            },
                            i if i == potential_elevator => Floor {
                                microchips: cur.microchips + chip_change,
                                generators: cur.generators + gen_change,
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
    let mut names: IndexSet<&str> = IndexSet::new();
    let rx = regex!(r"(?<material>\w+)(?: |-compatible )(?<itemType>generator|microchip)");
    let floors = input.lines().collect_vec();
    let floors: [Floor; 4] = std::array::from_fn(|index| {
        let mut microchips = 0;
        let mut generators = 0;
        for caps in rx.captures_iter(floors[index]) {
            let name = caps.name("material").unwrap().as_str();
            let n = names.get_index_of(name).unwrap_or_else(|| {
                let n = names.len();
                names.insert(name);
                n
            });
            match caps.name("itemType").unwrap().as_str() {
                "generator" => { generators += 1 << n; },
                "microchip" => { microchips += 1 << n; },
                _ => unreachable!(),
            }
        }
        Floor { microchips, generators }
    });
    FloorState { elevator: 0, floors }
}

fn solve_floors(initial_state: FloorState, items: u8) -> usize {
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
            for neighbor in state.next(items) {
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
    solve_floors(initial_state, 5)
}

fn part2(mut initial_state: Input) -> Output {
    initial_state.floors[0].microchips += 96;
    initial_state.floors[0].generators += 96;
    solve_floors(initial_state, 7)
}

#[test]
fn default() {
    let input = get_input(16, 11).unwrap();
    let input = parse_input(&input);
    assert_eq!(47, part1(input.clone()));
    assert_eq!(71, part2(input));
}

// Input parsed (20μs)
// 1. 47 (21ms)
// 2. 71 (113ms)
// Total: 135ms
