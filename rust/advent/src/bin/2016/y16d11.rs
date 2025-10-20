use std::{iter::successors, num::NonZero};

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

impl Floor {
    fn is_valid(&self) -> bool {
        self.microchips == 0 || self.generators == 0 || self.microchips & self.generators == self.microchips
    }
}

#[derive(Clone, Debug, PartialEq, Eq, Hash)]
struct FloorState {
    elevator: usize,
    floors: [Floor; 4],
}

impl FloorState {
    fn is_valid(&self) -> bool {
        self.floors.iter().all(|floor| floor.is_valid())
    }

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
            let matchy = cur_floor.microchips & cur_floor.generators;
            for n in successors(Some(matchy), |&acc| {
                if acc == 0 {
                    None
                } else {
                    Some(acc >> 1)
                }
            }) {
                
            }
            let pot_flr_no_chips = potential_floor.microchips == 0;
            let pot_flr_unmatched_chips = potential_floor.microchips & !potential_floor.generators;
            let pot_flr_no_gen = potential_floor.generators == 0;
            let pot_chips = if pot_flr_no_gen {
                cur_floor.microchips
            } else {
                cur_floor.microchips & potential_floor.generators
            };
            let mut pot_gens = cur_floor.generators & !cur_floor.microchips;
            if !pot_flr_no_chips {
                if pot_flr_unmatched_chips.count_ones() > 1 {
                    pot_gens = 0;
                } else {
                    pot_gens = pot_gens & potential_floor.microchips;
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
    initial_state.floors[0].microchips += 96;
    initial_state.floors[0].generators += 96;
    solve_floors(initial_state)
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
